pub mod demo_board;

use crate::http::{Http, HttpError};
use serde::Serialize;

pub struct Credential {
    pub username: String,
    pub password: String,
}

pub struct PostContent {
    pub title: String,
    pub body: String,
}

/// 자동으로 넘길 수 없어 사람이 마무리해야 하는 지점이다.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HumanReason {
    Captcha,
    PhoneVerification,
    EmailVerification,
    TermsChange,
}

impl HumanReason {
    pub fn describe(self) -> &'static str {
        match self {
            Self::Captcha => "자동등록방지 입력이 필요합니다.",
            Self::PhoneVerification => "휴대폰 본인확인이 필요합니다.",
            Self::EmailVerification => "이메일 인증이 필요합니다.",
            Self::TermsChange => "변경된 약관 동의가 필요합니다.",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LoginOutcome {
    Ok,
    WrongCredential,
    NeedsHuman(HumanReason),
    Blocked(String),
}

#[derive(Debug, PartialEq)]
pub enum SubmitOutcome {
    Posted { url: Option<String> },
    Rejected(String),
    NeedsHuman(HumanReason),
}

/// 등록 요청 성공과 실제 열람 가능 여부는 서로 다른 사실이다.
#[derive(Debug, PartialEq)]
pub enum VerifyOutcome {
    PubliclyVisible,
    LoginRequired,
    NotFound,
}

impl VerifyOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PubliclyVisible => "public",
            Self::LoginRequired => "login_required",
            Self::NotFound => "not_found",
        }
    }
}

pub trait SiteAdapter: Send + Sync {
    fn id(&self) -> &'static str;

    fn login(
        &self,
        http: &dyn Http,
        base_url: &str,
        credential: &Credential,
    ) -> Result<LoginOutcome, HttpError>;

    fn submit(
        &self,
        http: &dyn Http,
        base_url: &str,
        content: &PostContent,
    ) -> Result<SubmitOutcome, HttpError>;

    /// 로그인하지 않은 별도 세션으로 호출해야 공개 여부를 제대로 판단할 수 있다.
    fn verify(&self, http: &dyn Http, url: &str) -> Result<VerifyOutcome, HttpError>;
}

pub fn resolve(adapter_id: &str) -> Option<Box<dyn SiteAdapter>> {
    match adapter_id {
        "demo_board" => Some(Box::new(demo_board::DemoBoardAdapter)),
        _ => None,
    }
}
