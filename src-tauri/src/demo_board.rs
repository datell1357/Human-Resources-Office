use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tiny_http::{Header, Request, Response, Server};

struct DemoUser {
    password: String,
    requires_captcha: bool,
}

struct BoardPost {
    subject: String,
    content: String,
    members_only: bool,
}

struct BoardState {
    users: HashMap<String, DemoUser>,
    sessions: HashSet<String>,
    posts: Vec<BoardPost>,
    next_session: u64,
}

impl BoardState {
    fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(
            "worker01".to_string(),
            DemoUser {
                password: "demo-pass".to_string(),
                requires_captcha: false,
            },
        );
        users.insert(
            "worker02".to_string(),
            DemoUser {
                password: "demo-pass".to_string(),
                requires_captcha: false,
            },
        );
        users.insert(
            "worker03".to_string(),
            DemoUser {
                password: "demo-pass".to_string(),
                requires_captcha: false,
            },
        );
        users.insert(
            "captcha01".to_string(),
            DemoUser {
                password: "demo-pass".to_string(),
                requires_captcha: true,
            },
        );
        Self {
            users,
            sessions: HashSet::new(),
            posts: Vec::new(),
            next_session: 1,
        }
    }
}

fn page(title: &str, body: &str) -> String {
    format!("<!doctype html><html><head><title>{title}</title></head><body>{body}</body></html>")
}

fn read_form(request: &mut Request) -> HashMap<String, String> {
    let mut raw = String::new();
    let _ = request.as_reader().read_to_string(&mut raw);
    url::form_urlencoded::parse(raw.as_bytes())
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect()
}

fn session_of(request: &Request) -> Option<String> {
    let cookie = request
        .headers()
        .iter()
        .find(|header| header.field.equiv("Cookie"))?
        .value
        .as_str()
        .to_string();
    cookie
        .split(';')
        .map(str::trim)
        .find_map(|entry| entry.strip_prefix("PHPSESSID=").map(str::to_string))
}

fn query_value(url: &str, key: &str) -> Option<String> {
    let query = url.split_once('?')?.1;
    url::form_urlencoded::parse(query.as_bytes())
        .find(|(name, _)| name == key)
        .map(|(_, value)| value.into_owned())
}

struct Reply {
    status: u16,
    html: String,
    session: Option<String>,
}

impl Reply {
    fn ok(html: String) -> Self {
        Self {
            status: 200,
            html,
            session: None,
        }
    }

    fn not_found() -> Self {
        Self {
            status: 404,
            html: page("없는 글", "요청한 글을 찾을 수 없습니다."),
            session: None,
        }
    }
}

fn handle_login(state: &Mutex<BoardState>, request: &mut Request) -> Reply {
    let form = read_form(request);
    let identifier = form.get("mb_id").cloned().unwrap_or_default();
    let password = form.get("mb_password").cloned().unwrap_or_default();

    let mut state = state.lock().unwrap();
    let Some(user) = state.users.get(&identifier) else {
        return Reply::ok(page("로그인", "아이디 또는 비밀번호가 올바르지 않습니다."));
    };

    if user.requires_captcha {
        return Reply::ok(page("로그인", "자동등록방지 숫자를 입력해 주세요."));
    }
    if user.password != password {
        return Reply::ok(page("로그인", "아이디 또는 비밀번호가 올바르지 않습니다."));
    }

    let session = format!("demo{}", state.next_session);
    state.next_session += 1;
    state.sessions.insert(session.clone());

    Reply {
        status: 200,
        html: page("로그인", "로그인되었습니다."),
        session: Some(session),
    }
}

fn handle_write(state: &Mutex<BoardState>, request: &mut Request) -> Reply {
    let session = session_of(request);
    let form = read_form(request);
    let mut state = state.lock().unwrap();

    let authenticated = session
        .as_ref()
        .map(|value| state.sessions.contains(value))
        .unwrap_or(false);
    if !authenticated {
        return Reply::ok(page("글쓰기", "로그인 후 이용해 주세요."));
    }

    // 세 번째 글마다 회원 전용으로 등록된다. 등록은 성공해도 외부에서는 보이지 않는다.
    let members_only = state.posts.len() % 3 == 2;
    state.posts.push(BoardPost {
        subject: form.get("wr_subject").cloned().unwrap_or_default(),
        content: form.get("wr_content").cloned().unwrap_or_default(),
        members_only,
    });
    let identifier = state.posts.len();

    Reply::ok(page(
        "글쓰기",
        &format!(
            "<p>등록이 완료되었습니다.</p><a class=\"posted-link\" href=\"/bbs/board.php?bo_table=job&amp;wr_id={identifier}\">등록한 글 보기</a>"
        ),
    ))
}

fn handle_view(state: &Mutex<BoardState>, request: &Request) -> Reply {
    let session = session_of(request);
    let Some(identifier) =
        query_value(request.url(), "wr_id").and_then(|value| value.parse::<usize>().ok())
    else {
        return Reply::not_found();
    };

    let state = state.lock().unwrap();
    let Some(post) = identifier
        .checked_sub(1)
        .and_then(|index| state.posts.get(index))
    else {
        return Reply::not_found();
    };

    let authenticated = session
        .as_ref()
        .map(|value| state.sessions.contains(value))
        .unwrap_or(false);
    if post.members_only && !authenticated {
        return Reply::ok(page("회원 전용", "로그인 후 이용해 주세요."));
    }

    Reply::ok(page(
        &post.subject,
        &format!(
            "<h1>{}</h1><div class=\"posted-body\">{}</div>",
            post.subject, post.content
        ),
    ))
}

/// 실행 엔진을 실제로 검증하기 위한 로컬 데모 게시판이다.
/// 그누보드 계열의 경로와 응답 문구를 흉내 내며 127.0.0.1에만 바인딩한다.
pub struct DemoBoard {
    base_url: String,
    server: Arc<Server>,
    worker: Option<JoinHandle<()>>,
}

impl DemoBoard {
    pub fn start() -> std::io::Result<Self> {
        let server = Server::http("127.0.0.1:0")
            .map_err(|error| std::io::Error::other(error.to_string()))?;
        let server = Arc::new(server);
        let port = server
            .server_addr()
            .to_ip()
            .map(|address| address.port())
            .unwrap_or_default();
        let base_url = format!("http://127.0.0.1:{port}");

        let worker_server = Arc::clone(&server);
        let worker = std::thread::spawn(move || {
            let state = Mutex::new(BoardState::new());
            for mut request in worker_server.incoming_requests() {
                let url = request.url().to_string();
                let reply = if url.contains("/bbs/login.php") {
                    handle_login(&state, &mut request)
                } else if url.contains("/bbs/write.php") {
                    handle_write(&state, &mut request)
                } else if url.contains("/bbs/board.php") {
                    handle_view(&state, &request)
                } else {
                    Reply::not_found()
                };

                let mut response = Response::from_string(reply.html).with_status_code(reply.status);
                response.add_header(
                    Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
                        .expect("고정 헤더"),
                );
                if let Some(session) = reply.session {
                    if let Ok(header) = Header::from_bytes(
                        &b"Set-Cookie"[..],
                        format!("PHPSESSID={session}; Path=/").as_bytes(),
                    ) {
                        response.add_header(header);
                    }
                }
                let _ = request.respond(response);
            }
        });

        Ok(Self {
            base_url,
            server,
            worker: Some(worker),
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl Drop for DemoBoard {
    fn drop(&mut self) {
        self.server.unblock();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
