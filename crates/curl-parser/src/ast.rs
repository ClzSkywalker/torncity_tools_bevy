use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            _ => HttpMethod::GET,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct CurlRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub form_data: Vec<(String, String)>,
    pub auth: Option<Auth>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub cookies: Option<String>,
    pub compressed: bool,
}

impl Default for CurlRequest {
    fn default() -> Self {
        Self {
            url: String::new(),
            method: HttpMethod::GET,
            headers: HashMap::new(),
            body: None,
            form_data: Vec::new(),
            auth: None,
            user_agent: None,
            referer: None,
            cookies: None,
            compressed: false,
        }
    }
}

impl CurlRequest {
    pub fn into_http_request(self) -> crate::Result<ehttp::Request> {
        use ehttp::Request;

        let url = if self.url.is_empty() {
            return Err(crate::CurlError::UrlNotFound);
        } else {
            self.url.clone()
        };

        let body = self.body.clone().unwrap_or_default();

        let mut req = match self.method {
            HttpMethod::GET => {
                let mut r = Request::get(url);
                r.body = body;
                r
            }
            HttpMethod::POST => Request::post(url, body),
            HttpMethod::PUT => {
                let mut r = Request::post(url, body);
                r.method = "PUT".to_string();
                r
            }
            HttpMethod::DELETE => {
                let mut r = Request::post(url, body);
                r.method = "DELETE".to_string();
                r
            }
            HttpMethod::PATCH => {
                let mut r = Request::post(url, body);
                r.method = "PATCH".to_string();
                r
            }
            HttpMethod::HEAD => {
                let mut r = Request::head(url);
                r.body = body;
                r
            }
            HttpMethod::OPTIONS => {
                let mut r = Request::post(url, body);
                r.method = "OPTIONS".to_string();
                r
            }
        };

        for (key, value) in self.headers {
            req.headers.insert(key, value);
        }

        Ok(req)
    }
}
