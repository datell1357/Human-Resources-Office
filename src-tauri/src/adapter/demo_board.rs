use super::{
    Credential, HumanReason, LoginOutcome, PostContent, SiteAdapter, SubmitOutcome, VerifyOutcome,
};
use crate::http::{Http, HttpError};

const POSTED_LINK_MARKER: &str = "class=\"posted-link\" href=\"";

fn extract_posted_link(body: &str) -> Option<String> {
    let start = body.find(POSTED_LINK_MARKER)? + POSTED_LINK_MARKER.len();
    let rest = &body[start..];
    let end = rest.find('"')?;
    // href 안의 &amp; 는 실제 주소에서 & 로 되돌려야 한다.
    Some(rest[..end].replace("&amp;", "&"))
}

fn absolute(base_url: &str, path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        path.to_string()
    } else {
        format!("{}{}", base_url.trim_end_matches('/'), path)
    }
}

/// 그누보드 계열 게시판의 로그인·글쓰기 흐름을 그대로 따른 어댑터다.
pub struct DemoBoardAdapter;

impl SiteAdapter for DemoBoardAdapter {
    fn id(&self) -> &'static str {
        "demo_board"
    }

    fn login(
        &self,
        http: &dyn Http,
        base_url: &str,
        credential: &Credential,
    ) -> Result<LoginOutcome, HttpError> {
        let response = http.post_form(
            &absolute(base_url, "/bbs/login.php"),
            &[
                ("mb_id", credential.username.as_str()),
                ("mb_password", credential.password.as_str()),
            ],
        )?;

        if response.body.contains("자동등록방지") {
            return Ok(LoginOutcome::NeedsHuman(HumanReason::Captcha));
        }
        if response.body.contains("휴대폰 본인확인") {
            return Ok(LoginOutcome::NeedsHuman(HumanReason::PhoneVerification));
        }
        if response
            .body
            .contains("아이디 또는 비밀번호가 올바르지 않습니다")
        {
            return Ok(LoginOutcome::WrongCredential);
        }
        if response.status != 200 {
            return Ok(LoginOutcome::Blocked(format!(
                "서버가 {} 응답을 돌려줬습니다.",
                response.status
            )));
        }
        if response.body.contains("로그인되었습니다") {
            return Ok(LoginOutcome::Ok);
        }
        Ok(LoginOutcome::Blocked(
            "로그인 결과를 판단할 수 없습니다.".to_string(),
        ))
    }

    fn submit(
        &self,
        http: &dyn Http,
        base_url: &str,
        content: &PostContent,
    ) -> Result<SubmitOutcome, HttpError> {
        let response = http.post_form(
            &absolute(base_url, "/bbs/write.php"),
            &[
                ("bo_table", "job"),
                ("wr_subject", content.title.as_str()),
                ("wr_content", content.body.as_str()),
            ],
        )?;

        if response.body.contains("자동등록방지") {
            return Ok(SubmitOutcome::NeedsHuman(HumanReason::Captcha));
        }
        if response.body.contains("로그인 후 이용해 주세요") {
            return Ok(SubmitOutcome::Rejected(
                "로그인 세션이 유지되지 않았습니다.".to_string(),
            ));
        }
        if let Some(path) = extract_posted_link(&response.body) {
            return Ok(SubmitOutcome::Posted {
                url: Some(absolute(base_url, &path)),
            });
        }
        Ok(SubmitOutcome::Rejected(format!(
            "게시글 주소를 찾지 못했습니다. 응답 코드 {}.",
            response.status
        )))
    }

    fn verify(&self, http: &dyn Http, url: &str) -> Result<VerifyOutcome, HttpError> {
        let response = http.get(url)?;
        if response.status == 404 {
            return Ok(VerifyOutcome::NotFound);
        }
        if response.body.contains("로그인 후 이용해 주세요") {
            return Ok(VerifyOutcome::LoginRequired);
        }
        if response.body.contains("posted-body") {
            return Ok(VerifyOutcome::PubliclyVisible);
        }
        Ok(VerifyOutcome::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::FakeHttp;

    fn credential() -> Credential {
        Credential {
            username: "worker01".to_string(),
            password: "demo-pass".to_string(),
        }
    }

    #[test]
    fn reads_each_login_result_from_the_response() {
        let cases = [
            ("로그인되었습니다.", LoginOutcome::Ok),
            (
                "아이디 또는 비밀번호가 올바르지 않습니다.",
                LoginOutcome::WrongCredential,
            ),
            (
                "자동등록방지 숫자를 입력해 주세요.",
                LoginOutcome::NeedsHuman(HumanReason::Captcha),
            ),
            (
                "휴대폰 본인확인이 필요합니다.",
                LoginOutcome::NeedsHuman(HumanReason::PhoneVerification),
            ),
        ];

        for (body, expected) in cases {
            let http = FakeHttp::new(&[("/bbs/login.php", 200, body)]);
            let outcome = DemoBoardAdapter
                .login(&http, "http://127.0.0.1:9000", &credential())
                .unwrap();
            assert_eq!(outcome, expected, "본문: {body}");
        }
    }

    #[test]
    fn takes_the_posted_address_from_the_write_response() {
        let http = FakeHttp::new(&[(
            "/bbs/write.php",
            200,
            "<a class=\"posted-link\" href=\"/bbs/board.php?bo_table=job&amp;wr_id=7\">등록한 글 보기</a>",
        )]);

        let outcome = DemoBoardAdapter
            .submit(
                &http,
                "http://127.0.0.1:9000",
                &PostContent {
                    title: "제목".to_string(),
                    body: "본문".to_string(),
                },
            )
            .unwrap();

        assert_eq!(
            outcome,
            SubmitOutcome::Posted {
                url: Some("http://127.0.0.1:9000/bbs/board.php?bo_table=job&wr_id=7".to_string())
            }
        );
    }

    #[test]
    fn treats_a_lost_session_as_rejected_rather_than_posted() {
        let http = FakeHttp::new(&[("/bbs/write.php", 200, "로그인 후 이용해 주세요.")]);

        let outcome = DemoBoardAdapter
            .submit(
                &http,
                "http://127.0.0.1:9000",
                &PostContent {
                    title: "제목".to_string(),
                    body: "본문".to_string(),
                },
            )
            .unwrap();

        assert!(matches!(outcome, SubmitOutcome::Rejected(_)));
    }

    #[test]
    fn separates_public_view_from_members_only_and_missing_pages() {
        let public = FakeHttp::new(&[("wr_id=1", 200, "<div class=\"posted-body\">본문</div>")]);
        let members = FakeHttp::new(&[("wr_id=2", 200, "로그인 후 이용해 주세요.")]);
        let missing = FakeHttp::new(&[("wr_id=3", 404, "")]);

        assert_eq!(
            DemoBoardAdapter
                .verify(&public, "http://x/wr_id=1")
                .unwrap(),
            VerifyOutcome::PubliclyVisible
        );
        assert_eq!(
            DemoBoardAdapter
                .verify(&members, "http://x/wr_id=2")
                .unwrap(),
            VerifyOutcome::LoginRequired
        );
        assert_eq!(
            DemoBoardAdapter
                .verify(&missing, "http://x/wr_id=3")
                .unwrap(),
            VerifyOutcome::NotFound
        );
    }
}
