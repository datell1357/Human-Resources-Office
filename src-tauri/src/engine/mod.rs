pub mod http_runner;

use crate::adapter::PostContent;
use crate::now_millis;
use rusqlite::{params, Connection, Result as SqlResult};
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub const STATE_QUEUED: &str = "queued";
pub const STATE_SUBMITTED: &str = "submitted";
pub const STATE_VERIFIED: &str = "verified";
pub const STATE_FAILED: &str = "failed";
pub const STATE_NEEDS_HUMAN: &str = "needs_human";

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemOutcome {
    pub state: String,
    pub posted_url: Option<String>,
    pub visibility: Option<String>,
    pub failure_kind: Option<String>,
    pub message: String,
}

impl ItemOutcome {
    pub fn failed(kind: &str, message: impl Into<String>) -> Self {
        Self {
            state: STATE_FAILED.to_string(),
            posted_url: None,
            visibility: None,
            failure_kind: Some(kind.to_string()),
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TargetJob {
    pub item_id: i64,
    pub seq: i64,
    pub site_name: String,
    pub site_url: String,
    pub adapter_id: String,
    pub username: String,
    pub secret_ref: String,
    pub posted_url: Option<String>,
    pub state: String,
}

pub trait TargetRunner: Send + Sync {
    fn submit(&self, job: &TargetJob, content: &PostContent) -> ItemOutcome;
    fn verify(&self, job: &TargetJob, url: &str) -> ItemOutcome;
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunSummary {
    pub run_id: i64,
    pub total: usize,
    pub processed: usize,
    pub verified: usize,
    pub submitted: usize,
    pub failed: usize,
    pub needs_human: usize,
    pub stopped: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunProgress {
    pub run_id: i64,
    pub seq: i64,
    pub total: usize,
    pub site_name: String,
    pub outcome: ItemOutcome,
}

/// 활성 계정이 있는 사용 중인 사이트를 실행 목록으로 만든다.
/// 같은 실행에서 사이트·계정 조합은 한 번만 들어간다.
pub fn plan_run(connection: &Connection, post_id: i64) -> SqlResult<i64> {
    let now = now_millis() as i64;
    connection.execute(
        "INSERT INTO runs (post_id, state, started_at) VALUES (?1, 'running', ?2)",
        params![post_id, now],
    )?;
    let run_id = connection.last_insert_rowid();

    let mut statement = connection.prepare(
        "SELECT accounts.id, accounts.site_id FROM accounts \
         JOIN sites ON sites.id = accounts.site_id \
         WHERE accounts.status = 'active' AND sites.enabled = 1 \
         ORDER BY sites.id, accounts.id",
    )?;
    let targets = statement
        .query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?
        .collect::<SqlResult<Vec<_>>>()?;

    for (index, (account_id, site_id)) in targets.iter().enumerate() {
        connection.execute(
            "INSERT INTO run_items (run_id, site_id, account_id, seq) VALUES (?1, ?2, ?3, ?4)",
            params![run_id, site_id, account_id, index as i64 + 1],
        )?;
    }

    Ok(run_id)
}

fn pending_jobs(connection: &Connection, run_id: i64) -> SqlResult<Vec<TargetJob>> {
    let mut statement = connection.prepare(
        "SELECT run_items.id, run_items.seq, run_items.state, run_items.posted_url, \
                sites.name, sites.url, sites.adapter, accounts.username, accounts.secret_ref \
         FROM run_items \
         JOIN sites ON sites.id = run_items.site_id \
         JOIN accounts ON accounts.id = run_items.account_id \
         WHERE run_items.run_id = ?1 AND run_items.state IN ('queued', 'submitted') \
         ORDER BY run_items.seq",
    )?;
    let jobs = statement
        .query_map([run_id], |row| {
            Ok(TargetJob {
                item_id: row.get(0)?,
                seq: row.get(1)?,
                state: row.get(2)?,
                posted_url: row.get(3)?,
                site_name: row.get(4)?,
                site_url: row.get(5)?,
                adapter_id: row.get(6)?,
                username: row.get(7)?,
                secret_ref: row.get(8)?,
            })
        })?
        .collect::<SqlResult<Vec<_>>>()?;
    Ok(jobs)
}

fn store_outcome(connection: &Connection, item_id: i64, outcome: &ItemOutcome) -> SqlResult<()> {
    let now = now_millis() as i64;
    let submitted_at = if outcome.state == STATE_SUBMITTED || outcome.state == STATE_VERIFIED {
        Some(now)
    } else {
        None
    };
    let verified_at = if outcome.visibility.is_some() {
        Some(now)
    } else {
        None
    };

    connection.execute(
        "UPDATE run_items SET state = ?1, posted_url = COALESCE(?2, posted_url), \
         submitted_at = COALESCE(?3, submitted_at), verified_at = COALESCE(?4, verified_at), \
         visibility = COALESCE(?5, visibility), failure_kind = ?6, message = ?7 WHERE id = ?8",
        params![
            outcome.state,
            outcome.posted_url,
            submitted_at,
            verified_at,
            outcome.visibility,
            outcome.failure_kind,
            outcome.message,
            item_id
        ],
    )?;
    Ok(())
}

fn wait_between_targets(stop: &AtomicBool, delay_ms: u64) {
    let mut waited = 0;
    while waited < delay_ms && !stop.load(Ordering::Relaxed) {
        let slice = u64::min(100, delay_ms - waited);
        std::thread::sleep(Duration::from_millis(slice));
        waited += slice;
    }
}

/// 중지 신호가 오면 진행 중인 대상은 끝까지 마무리하고 다음 대상을 시작하지 않는다.
/// 이미 등록된 대상은 다시 제출하지 않고 열람 확인만 이어서 수행한다.
pub fn execute_run(
    connection: &Connection,
    run_id: i64,
    content: &PostContent,
    runner: &dyn TargetRunner,
    stop: &AtomicBool,
    delay_ms: u64,
    mut progress: impl FnMut(RunProgress),
) -> SqlResult<RunSummary> {
    let jobs = pending_jobs(connection, run_id)?;
    let total: usize = connection.query_row(
        "SELECT COUNT(*) FROM run_items WHERE run_id = ?1",
        [run_id],
        |row| row.get::<_, i64>(0),
    )? as usize;

    let mut summary = RunSummary {
        run_id,
        total,
        ..RunSummary::default()
    };

    for (index, job) in jobs.iter().enumerate() {
        if stop.load(Ordering::Relaxed) {
            summary.stopped = true;
            break;
        }

        let outcome = match (job.state.as_str(), job.posted_url.as_deref()) {
            (STATE_SUBMITTED, Some(url)) => runner.verify(job, url),
            _ => {
                let submitted = runner.submit(job, content);
                match (submitted.state.as_str(), submitted.posted_url.as_deref()) {
                    (STATE_SUBMITTED, Some(url)) => {
                        store_outcome(connection, job.item_id, &submitted)?;
                        runner.verify(job, url)
                    }
                    _ => submitted,
                }
            }
        };

        store_outcome(connection, job.item_id, &outcome)?;
        summary.processed += 1;
        match outcome.state.as_str() {
            STATE_VERIFIED => summary.verified += 1,
            STATE_SUBMITTED => summary.submitted += 1,
            STATE_NEEDS_HUMAN => summary.needs_human += 1,
            _ => summary.failed += 1,
        }

        progress(RunProgress {
            run_id,
            seq: job.seq,
            total,
            site_name: job.site_name.clone(),
            outcome,
        });

        if index + 1 < jobs.len() {
            wait_between_targets(stop, delay_ms);
        }
    }

    let remaining: i64 = connection.query_row(
        "SELECT COUNT(*) FROM run_items WHERE run_id = ?1 AND state IN ('queued', 'submitted')",
        [run_id],
        |row| row.get(0),
    )?;
    let state = if summary.stopped {
        "stopped"
    } else if remaining > 0 {
        "incomplete"
    } else {
        "done"
    };
    connection.execute(
        "UPDATE runs SET state = ?1, finished_at = ?2 WHERE id = ?3",
        params![state, now_millis() as i64, run_id],
    )?;

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::Database;
    use std::collections::HashMap;
    use std::sync::Mutex;

    enum Behavior {
        Public,
        MembersOnly,
        Captcha,
    }

    struct FakeRunner {
        behavior: HashMap<i64, Behavior>,
        submitted: Mutex<Vec<i64>>,
        verified: Mutex<Vec<i64>>,
        stop_after_first: Option<&'static AtomicBool>,
    }

    impl FakeRunner {
        fn new(behavior: Vec<(i64, Behavior)>) -> Self {
            Self {
                behavior: behavior.into_iter().collect(),
                submitted: Mutex::new(Vec::new()),
                verified: Mutex::new(Vec::new()),
                stop_after_first: None,
            }
        }
    }

    impl TargetRunner for FakeRunner {
        fn submit(&self, job: &TargetJob, _content: &PostContent) -> ItemOutcome {
            self.submitted.lock().unwrap().push(job.seq);
            if let Some(stop) = self.stop_after_first {
                stop.store(true, Ordering::Relaxed);
            }
            match self.behavior.get(&job.seq) {
                Some(Behavior::Captcha) => ItemOutcome {
                    state: STATE_NEEDS_HUMAN.to_string(),
                    posted_url: None,
                    visibility: None,
                    failure_kind: Some("login_needs_human".to_string()),
                    message: "자동등록방지 입력이 필요합니다.".to_string(),
                },
                _ => ItemOutcome {
                    state: STATE_SUBMITTED.to_string(),
                    posted_url: Some(format!("http://127.0.0.1/view?id={}", job.seq)),
                    visibility: None,
                    failure_kind: None,
                    message: "등록 요청이 처리됐습니다.".to_string(),
                },
            }
        }

        fn verify(&self, job: &TargetJob, url: &str) -> ItemOutcome {
            self.verified.lock().unwrap().push(job.seq);
            match self.behavior.get(&job.seq) {
                Some(Behavior::MembersOnly) => ItemOutcome {
                    state: STATE_SUBMITTED.to_string(),
                    posted_url: Some(url.to_string()),
                    visibility: Some("login_required".to_string()),
                    failure_kind: Some("login_required".to_string()),
                    message: "등록은 됐지만 로그인해야 보입니다.".to_string(),
                },
                _ => ItemOutcome {
                    state: STATE_VERIFIED.to_string(),
                    posted_url: Some(url.to_string()),
                    visibility: Some("public".to_string()),
                    failure_kind: None,
                    message: "외부에서 보입니다.".to_string(),
                },
            }
        }
    }

    fn seeded_database(site_count: usize) -> Database {
        let database = Database::open_in_memory().unwrap();
        let connection = database.connection();
        connection
            .execute(
                "INSERT INTO posts (title, body, created_at, updated_at) VALUES ('공고', '본문', 1, 1)",
                [],
            )
            .unwrap();
        for index in 0..site_count {
            connection
                .execute(
                    "INSERT INTO sites (name, url, adapter, created_at) VALUES (?1, ?2, 'demo_board', 1)",
                    params![format!("사이트{index}"), format!("http://127.0.0.1/{index}")],
                )
                .unwrap();
            connection
                .execute(
                    "INSERT INTO accounts (site_id, username, secret_ref, status, created_at) \
                     VALUES (?1, ?2, ?3, 'active', 1)",
                    params![
                        index as i64 + 1,
                        format!("worker{index}"),
                        format!("account:{}:worker{index}", index + 1)
                    ],
                )
                .unwrap();
        }
        database
    }

    fn content() -> PostContent {
        PostContent {
            title: "공고".to_string(),
            body: "본문".to_string(),
        }
    }

    #[test]
    fn plans_one_target_per_active_account_and_skips_disabled_sites() {
        let database = seeded_database(3);
        let connection = database.connection();
        connection
            .execute("UPDATE sites SET enabled = 0 WHERE id = 2", [])
            .unwrap();
        connection
            .execute("UPDATE accounts SET status = 'excluded' WHERE id = 3", [])
            .unwrap();

        let run_id = plan_run(connection, 1).unwrap();
        let count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM run_items WHERE run_id = ?1",
                [run_id],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn keeps_members_only_posts_out_of_the_verified_count() {
        let database = seeded_database(2);
        let connection = database.connection();
        let run_id = plan_run(connection, 1).unwrap();
        let runner = FakeRunner::new(vec![(1, Behavior::Public), (2, Behavior::MembersOnly)]);
        let stop = AtomicBool::new(false);

        let summary =
            execute_run(connection, run_id, &content(), &runner, &stop, 0, |_| {}).unwrap();

        assert_eq!(summary.verified, 1);
        assert_eq!(summary.submitted, 1);
        assert_eq!(summary.failed, 0);

        let visibility: String = connection
            .query_row(
                "SELECT visibility FROM run_items WHERE run_id = ?1 AND seq = 2",
                [run_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(visibility, "login_required");
    }

    #[test]
    fn routes_captcha_targets_to_the_human_queue_instead_of_failing() {
        let database = seeded_database(1);
        let connection = database.connection();
        let run_id = plan_run(connection, 1).unwrap();
        let runner = FakeRunner::new(vec![(1, Behavior::Captcha)]);
        let stop = AtomicBool::new(false);

        let summary =
            execute_run(connection, run_id, &content(), &runner, &stop, 0, |_| {}).unwrap();

        assert_eq!(summary.needs_human, 1);
        assert_eq!(summary.failed, 0);
    }

    #[test]
    fn stopping_leaves_the_remaining_targets_queued() {
        static STOP: AtomicBool = AtomicBool::new(false);
        STOP.store(false, Ordering::Relaxed);

        let database = seeded_database(3);
        let connection = database.connection();
        let run_id = plan_run(connection, 1).unwrap();
        let mut runner = FakeRunner::new(vec![
            (1, Behavior::Public),
            (2, Behavior::Public),
            (3, Behavior::Public),
        ]);
        runner.stop_after_first = Some(&STOP);

        let summary =
            execute_run(connection, run_id, &content(), &runner, &STOP, 0, |_| {}).unwrap();

        assert!(summary.stopped);
        assert_eq!(summary.processed, 1);

        let queued: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM run_items WHERE run_id = ?1 AND state = 'queued'",
                [run_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(queued, 2);

        let run_state: String = connection
            .query_row("SELECT state FROM runs WHERE id = ?1", [run_id], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(run_state, "stopped");
    }

    #[test]
    fn resuming_verifies_an_already_submitted_target_instead_of_posting_again() {
        let database = seeded_database(2);
        let connection = database.connection();
        let run_id = plan_run(connection, 1).unwrap();

        connection
            .execute(
                "UPDATE run_items SET state = 'submitted', posted_url = 'http://127.0.0.1/view?id=1' \
                 WHERE run_id = ?1 AND seq = 1",
                [run_id],
            )
            .unwrap();

        let runner = FakeRunner::new(vec![(1, Behavior::Public), (2, Behavior::Public)]);
        let stop = AtomicBool::new(false);

        execute_run(connection, run_id, &content(), &runner, &stop, 0, |_| {}).unwrap();

        assert_eq!(*runner.submitted.lock().unwrap(), vec![2]);
        assert_eq!(*runner.verified.lock().unwrap(), vec![1, 2]);
    }
}
