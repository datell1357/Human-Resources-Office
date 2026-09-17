use std::fmt;

#[derive(Debug)]
pub struct HttpError(pub String);

impl fmt::Display for HttpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

/// 어댑터가 실제 네트워크 없이도 검증될 수 있도록 전송 계층을 분리한다.
pub trait Http {
    fn get(&self, url: &str) -> Result<HttpResponse, HttpError>;
    fn post_form(&self, url: &str, fields: &[(&str, &str)]) -> Result<HttpResponse, HttpError>;
}

pub struct UreqClient {
    agent: ureq::Agent,
}

impl UreqClient {
    pub fn new() -> Self {
        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .build();
        Self {
            agent: config.into(),
        }
    }
}

impl Default for UreqClient {
    fn default() -> Self {
        Self::new()
    }
}

fn into_response(
    result: Result<ureq::http::Response<ureq::Body>, ureq::Error>,
) -> Result<HttpResponse, HttpError> {
    let mut response = result.map_err(|error| HttpError(error.to_string()))?;
    let status = response.status().as_u16();
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|error| HttpError(error.to_string()))?;
    Ok(HttpResponse { status, body })
}

impl Http for UreqClient {
    fn get(&self, url: &str) -> Result<HttpResponse, HttpError> {
        into_response(self.agent.get(url).call())
    }

    fn post_form(&self, url: &str, fields: &[(&str, &str)]) -> Result<HttpResponse, HttpError> {
        let owned: Vec<(String, String)> = fields
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect();
        into_response(self.agent.post(url).send_form(owned))
    }
}

#[cfg(test)]
pub struct FakeHttp {
    routes: Vec<(String, u16, String)>,
    pub calls: std::sync::Mutex<Vec<String>>,
}

#[cfg(test)]
impl FakeHttp {
    pub fn new(routes: &[(&str, u16, &str)]) -> Self {
        Self {
            routes: routes
                .iter()
                .map(|(pattern, status, body)| {
                    ((*pattern).to_string(), *status, (*body).to_string())
                })
                .collect(),
            calls: std::sync::Mutex::new(Vec::new()),
        }
    }

    fn respond(&self, url: &str) -> Result<HttpResponse, HttpError> {
        self.calls.lock().unwrap().push(url.to_string());
        for (pattern, status, body) in &self.routes {
            if url.contains(pattern.as_str()) {
                return Ok(HttpResponse {
                    status: *status,
                    body: body.clone(),
                });
            }
        }
        Err(HttpError(format!("준비되지 않은 요청: {url}")))
    }
}

#[cfg(test)]
impl Http for FakeHttp {
    fn get(&self, url: &str) -> Result<HttpResponse, HttpError> {
        self.respond(url)
    }

    fn post_form(&self, url: &str, _fields: &[(&str, &str)]) -> Result<HttpResponse, HttpError> {
        self.respond(url)
    }
}
