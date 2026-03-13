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
    fn from_str(method: &str) -> Self {
        match method.to_uppercase().as_str() {
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

#[derive(Clone)]
pub struct HttpTool {
    pub params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub method: HttpMethod,
    pub url: String,
}

impl std::fmt::Debug for HttpTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body_str = if self.body.is_empty() {
            String::from("<empty>")
        } else if let Ok(s) = String::from_utf8(self.body.clone()) {
            s
        } else {
            format!("<binary data, {} bytes>", self.body.len())
        };

        writeln!(f, "HttpTool {{")?;
        writeln!(f, "  params: {:?},", self.params)?;
        writeln!(f, "  headers: {:?},", self.headers)?;
        writeln!(f, "  body: {},", body_str)?;
        writeln!(f, "  method: {:?},", self.method)?;
        writeln!(f, "  url: {:?}", self.url)?;
        write!(f, "}}")
    }
}

impl Default for HttpTool {
    fn default() -> Self {
        Self {
            params: HashMap::new(),
            headers: HashMap::new(),
            body: Vec::new(),
            method: HttpMethod::GET,
            url: String::new(),
        }
    }
}

impl From<HttpTool> for ehttp::Request {
    fn from(tool: HttpTool) -> Self {
        let url = if !tool.params.is_empty() {
            let separator = if tool.url.contains('?') { '&' } else { '?' };
            let query_string: Vec<String> = tool
                .params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect();
            format!("{}{}{}", tool.url, separator, query_string.join("&"))
        } else {
            tool.url
        };

        let body = tool.body;
        let mut req = match tool.method {
            HttpMethod::GET => {
                let mut r = ehttp::Request::get(url);
                r.body = body;
                r
            }
            HttpMethod::POST => ehttp::Request::post(url, body),
            HttpMethod::PUT => {
                let mut r = ehttp::Request::post(url, body);
                r.method = "PUT".to_string();
                r
            }
            HttpMethod::DELETE => {
                let mut r = ehttp::Request::post(url, body);
                r.method = "DELETE".to_string();
                r
            }
            HttpMethod::PATCH => {
                let mut r = ehttp::Request::post(url, body);
                r.method = "PATCH".to_string();
                r
            }
            HttpMethod::HEAD => {
                let mut r = ehttp::Request::head(url);
                r.body = body;
                r
            }
            HttpMethod::OPTIONS => {
                let mut r = ehttp::Request::post(url, body);
                r.method = "OPTIONS".to_string();
                r
            }
        };

        for (key, value) in tool.headers {
            req.headers.insert(key, value);
        }

        req
    }
}

impl HttpTool {
    pub fn add_param(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }
    pub fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    pub fn set_method(&mut self, method: HttpMethod) {
        self.method = method;
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    /// 从 curl 命令解析出 HTTP 请求配置
    ///
    /// # 示例
    /// ```
    /// use bevy_http::HttpTool;
    ///
    /// let curl = "curl -X POST https://api.example.com/data -H 'Content-Type: application/json' -d '{\"key\":\"value\"}'";
    /// let tool = HttpTool::from_curl(curl).unwrap();
    /// ```
    pub fn from_curl(curl_command: &str) -> Result<Self, String> {
        from_curl_with_parser(curl_command)
    }
}

/// 使用 curl-parser crate 解析 curl 命令
fn from_curl_with_parser(curl_command: &str) -> Result<HttpTool, String> {
    let request = curl_parser::parse_curl(curl_command).map_err(|e| e.to_string())?;

    let method = match request.method {
        curl_parser::HttpMethod::GET => HttpMethod::GET,
        curl_parser::HttpMethod::POST => HttpMethod::POST,
        curl_parser::HttpMethod::PUT => HttpMethod::PUT,
        curl_parser::HttpMethod::DELETE => HttpMethod::DELETE,
        curl_parser::HttpMethod::PATCH => HttpMethod::PATCH,
        curl_parser::HttpMethod::HEAD => HttpMethod::HEAD,
        curl_parser::HttpMethod::OPTIONS => HttpMethod::OPTIONS,
    };

    let body = request.body.unwrap_or_default();

    let mut tool = HttpTool {
        url: request.url,
        headers: request.headers,
        body: body.clone(),
        method,
        params: HashMap::new(),
    };

    if !request.form_data.is_empty() {
        let body_str = request
            .form_data
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        tool.body = body_str.into_bytes();

        if !tool.headers.contains_key("content-type") {
            tool.headers.insert(
                "content-type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            );
        }
    }

    Ok(tool)
}

