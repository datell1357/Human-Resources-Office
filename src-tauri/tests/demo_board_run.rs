//! 데모 게시판을 실제로 띄우고 HTTP 로그인·글쓰기·열람 확인까지 수행하는 통합 검증이다.
//! 포트를 열어야 하므로 기본 테스트에서는 제외한다.
//! 실행: cargo test --test demo_board_run -- --ignored --nocapture

use human_resources_office_lib::testing::{
    adapter::PostContent,
    demo_board::DemoBoard,
    engine::{execute_run, http_runner::HttpRunner, plan_run},
    secret::{MemorySecretStore, SecretStore},
    storage::Database,
};
use rusqlite::params;
use std::sync::atomic::AtomicBool;

#[test]
#[ignore = "로컬 포트를 열어야 한다"]
fn posts_to_the_demo_board_and_separates_public_from_members_only() {
    let board = DemoBoard::start().expect("데모 게시판 시작");
    let database = Database::open_in_memory().expect("저장소");
    let connection = database.connection();
    let secrets = MemorySecretStore::default();

    connection
        .execute(
            "INSERT INTO posts (title, body, created_at, updated_at) VALUES ('구인 공고', '본문입니다.', 1, 1)",
            [],
        )
        .expect("글 저장");

    for (index, username) in ["worker01", "worker02", "worker03", "captcha01"]
        .into_iter()
        .enumerate()
    {
        let site_id = index as i64 + 1;
        connection
            .execute(
                "INSERT INTO sites (name, url, adapter, created_at) VALUES (?1, ?2, 'demo_board', 1)",
                params![
                    format!("데모 게시판 {site_id}"),
                    format!("{}/s{site_id}", board.base_url())
                ],
            )
            .expect("사이트 저장");
        let reference = format!("account:{site_id}:{username}");
        secrets
            .store(&reference, "demo-pass")
            .expect("비밀번호 보관");
        connection
            .execute(
                "INSERT INTO accounts (site_id, username, secret_ref, status, created_at) \
                 VALUES (?1, ?2, ?3, 'active', 1)",
                params![site_id, username, reference],
            )
            .expect("계정 저장");
    }

    let run_id = plan_run(connection, 1).expect("실행 계획");
    let runner = HttpRunner::new(&secrets);
    let stop = AtomicBool::new(false);
    let content = PostContent {
        title: "구인 공고".to_string(),
        body: "본문입니다.".to_string(),
    };

    let summary = execute_run(
        connection,
        run_id,
        &content,
        &runner,
        &stop,
        0,
        |progress| {
            println!(
                "{}/{} {} -> {} ({})",
                progress.seq,
                progress.total,
                progress.site_name,
                progress.outcome.state,
                progress.outcome.message
            );
        },
    )
    .expect("실행");

    assert_eq!(summary.total, 4);
    assert_eq!(summary.verified, 2, "공개 글 2건이 열람 확인돼야 한다");
    assert_eq!(
        summary.submitted, 1,
        "회원 전용 글은 등록만 된 상태여야 한다"
    );
    assert_eq!(
        summary.needs_human, 1,
        "자동등록방지 계정은 사람 처리로 남아야 한다"
    );
    assert_eq!(summary.failed, 0);

    let members_only: String = connection
        .query_row(
            "SELECT visibility FROM run_items WHERE run_id = ?1 AND seq = 3",
            [run_id],
            |row| row.get(0),
        )
        .expect("회원 전용 결과");
    assert_eq!(members_only, "login_required");

    let posted: String = connection
        .query_row(
            "SELECT posted_url FROM run_items WHERE run_id = ?1 AND seq = 1",
            [run_id],
            |row| row.get(0),
        )
        .expect("게시글 주소");
    assert!(
        posted.starts_with(board.base_url()),
        "실제 게시글 주소: {posted}"
    );
}
