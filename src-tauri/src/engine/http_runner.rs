use super::{
    ItemOutcome, TargetJob, TargetRunner, STATE_NEEDS_HUMAN, STATE_SUBMITTED, STATE_VERIFIED,
};
use crate::adapter::{self, Credential, LoginOutcome, PostContent, SubmitOutcome, VerifyOutcome};
use crate::http::UreqClient;
use crate::secret::SecretStore;

pub struct HttpRunner<'a> {
    secrets: &'a dyn SecretStore,
}

impl<'a> HttpRunner<'a> {
    pub fn new(secrets: &'a dyn SecretStore) -> Self {
        Self { secrets }
    }
}

impl TargetRunner for HttpRunner<'_> {
    fn submit(&self, job: &TargetJob, content: &PostContent) -> ItemOutcome {
        let Some(site) = adapter::resolve(&job.adapter_id) else {
            return ItemOutcome::failed(
                "adapter_missing",
                format!("'{}' 어댑터를 찾지 못했습니다.", job.adapter_id),
            );
        };

        let password = match self.secrets.retrieve(&job.secret_ref) {
            Ok(value) => value,
            Err(error) => return ItemOutcome::failed("secret_missing", error.to_string()),
        };
        let credential = Credential {
            username: job.username.clone(),
            password,
        };

        // 계정마다 별도 클라이언트를 써서 세션이 섞이지 않도록 한다.
        let client = UreqClient::new();
        match site.login(&client, &job.site_url, &credential) {
            Ok(LoginOutcome::Ok) => {}
            Ok(LoginOutcome::WrongCredential) => {
                return ItemOutcome::failed("wrong_credential", "로그인 정보가 맞지 않습니다.")
            }
            Ok(LoginOutcome::NeedsHuman(reason)) => {
                return ItemOutcome {
                    state: STATE_NEEDS_HUMAN.to_string(),
                    posted_url: None,
                    visibility: None,
                    failure_kind: Some("login_needs_human".to_string()),
                    message: reason.describe().to_string(),
                }
            }
            Ok(LoginOutcome::Blocked(message)) => {
                return ItemOutcome::failed("login_blocked", message)
            }
            Err(error) => return ItemOutcome::failed("network", error.to_string()),
        }

        match site.submit(&client, &job.site_url, content) {
            Ok(SubmitOutcome::Posted { url }) => ItemOutcome {
                state: STATE_SUBMITTED.to_string(),
                posted_url: url,
                visibility: None,
                failure_kind: None,
                message: "등록 요청이 처리됐습니다. 열람 확인이 남았습니다.".to_string(),
            },
            Ok(SubmitOutcome::Rejected(message)) => ItemOutcome::failed("rejected", message),
            Ok(SubmitOutcome::NeedsHuman(reason)) => ItemOutcome {
                state: STATE_NEEDS_HUMAN.to_string(),
                posted_url: None,
                visibility: None,
                failure_kind: Some("submit_needs_human".to_string()),
                message: reason.describe().to_string(),
            },
            Err(error) => ItemOutcome::failed("network", error.to_string()),
        }
    }

    fn verify(&self, job: &TargetJob, url: &str) -> ItemOutcome {
        let Some(site) = adapter::resolve(&job.adapter_id) else {
            return ItemOutcome::failed(
                "adapter_missing",
                format!("'{}' 어댑터를 찾지 못했습니다.", job.adapter_id),
            );
        };

        // 로그인하지 않은 새 클라이언트로 확인해야 실제 공개 여부를 알 수 있다.
        let anonymous = UreqClient::new();
        match site.verify(&anonymous, url) {
            Ok(outcome) => {
                let visibility = Some(outcome.as_str().to_string());
                match outcome {
                    VerifyOutcome::PubliclyVisible => ItemOutcome {
                        state: STATE_VERIFIED.to_string(),
                        posted_url: Some(url.to_string()),
                        visibility,
                        failure_kind: None,
                        message: "등록한 글이 외부에서 보입니다.".to_string(),
                    },
                    VerifyOutcome::LoginRequired => ItemOutcome {
                        state: STATE_SUBMITTED.to_string(),
                        posted_url: Some(url.to_string()),
                        visibility,
                        failure_kind: Some("login_required".to_string()),
                        message: "등록은 됐지만 로그인해야 보이는 글입니다.".to_string(),
                    },
                    VerifyOutcome::NotFound => ItemOutcome {
                        state: STATE_SUBMITTED.to_string(),
                        posted_url: Some(url.to_string()),
                        visibility,
                        failure_kind: Some("not_found".to_string()),
                        message: "등록 주소를 열었지만 글을 찾지 못했습니다.".to_string(),
                    },
                }
            }
            Err(error) => ItemOutcome::failed("network", error.to_string()),
        }
    }
}
