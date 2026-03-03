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
        let parser = CurlParser::new(curl_command)?;
        parser.parse()
    }
}

/// Curl 命令解析器
struct CurlParser {
    tokens: Vec<String>,
    position: usize,
}

impl CurlParser {
    /// 创建新的解析器
    fn new(curl_command: &str) -> Result<Self, String> {
        let tokens = tokenize_curl(curl_command);

        // 验证命令是否以 curl 开头
        if tokens.is_empty() {
            return Err("Empty command".to_string());
        }

        if tokens[0] != "curl" && tokens[0] != "curl.exe" {
            return Err(format!(
                "Not a curl command. Expected 'curl' or 'curl.exe', got '{}'",
                tokens[0]
            ));
        }

        Ok(Self {
            tokens,
            position: 0,
        })
    }

    /// 解析 curl 命令
    fn parse(mut self) -> Result<HttpTool, String> {
        let mut method: Option<HttpMethod> = None;
        let mut url: Option<String> = None;
        let mut headers = HashMap::new();
        let mut data: Option<Vec<u8>> = None;
        let mut form_data: Vec<(String, String)> = Vec::new();

        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position].clone();

            match token.as_str() {
                "curl" | "curl.exe" => {
                    self.position += 1;
                }
                "-X" | "--request" => {
                    method = Some(self.parse_method()?);
                }
                "-H" | "--header" => {
                    let (key, value) = self.parse_header()?;
                    headers.insert(key, value);
                }
                "-d" | "--data" | "--data-raw" => {
                    data = Some(self.parse_data()?);
                }
                "--data-binary" => {
                    data = Some(self.parse_data()?);
                }
                "--data-urlencode" => {
                    let encoded = self.parse_data_urlencode()?;
                    data = Some(encoded);
                }
                "-F" | "--form" => {
                    let (key, value) = self.parse_form_data()?;
                    form_data.push((key, value));
                }
                "-u" | "--user" => {
                    let (key, value) = self.parse_auth()?;
                    headers.insert(key, value);
                }
                "-A" | "--user-agent" => {
                    let agent = self.consume_next_token("user agent")?;
                    headers.insert("user-agent".to_string(), agent);
                }
                "-b" | "--cookie" => {
                    let cookie = self.consume_next_token("cookie")?;
                    headers.insert("cookie".to_string(), cookie);
                }
                "-e" | "--referer" => {
                    let referer = self.consume_next_token("referer")?;
                    headers.insert("referer".to_string(), referer);
                }
                "--compressed" => {
                    headers.insert("accept-encoding".to_string(), "gzip, deflate, br".to_string());
                    self.position += 1;
                }
                // 忽略的选项
                "-k" | "--insecure" | "-s" | "--silent" | "-v" | "--verbose"
                | "-i" | "--include" | "-L" | "--location" | "-f" | "--fail" => {
                    self.position += 1;
                }
                // 需要参数但忽略的选项
                "--max-time" | "--connect-timeout" | "-m" | "--max-redirs" => {
                    self.position += 1;
                    if self.position < self.tokens.len() {
                        self.position += 1;
                    }
                }
                _ => {
                    // 处理组合的短选项 (如 -sSL)
                    if token.starts_with('-') && !token.starts_with("--") && token.len() > 2 {
                        self.parse_combined_flags(&mut method, &mut headers, &mut data)?;
                    }
                    // URL (不以 - 开头的参数)
                    else if !token.starts_with('-') {
                        if url.is_none() {
                            let potential_url = self.clean_url(token);
                            if is_valid_url(&potential_url) {
                                url = Some(potential_url);
                            } else {
                                return Err(format!("Invalid URL: '{}'", potential_url));
                            }
                        }
                        self.position += 1;
                    }
                    // 未知选项
                    else {
                        self.position += 1;
                    }
                }
            }
        }

        // 验证必需字段
        let url = url.ok_or_else(|| "URL not found in curl command".to_string())?;

        // 构建 HttpTool
        let mut tool = HttpTool {
            url,
            headers,
            method: HttpMethod::GET,
            body: Vec::new(),
            params: HashMap::new(),
        };

        // 处理表单数据
        if !form_data.is_empty() {
            let body_str = form_data
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
        } else if let Some(body_data) = data {
            tool.body = body_data;
        }

        // 确定 HTTP 方法
        tool.method = if let Some(m) = method {
            m
        } else if !tool.body.is_empty() {
            HttpMethod::POST
        } else {
            HttpMethod::GET
        };

        Ok(tool)
    }

    /// 解析 HTTP 方法
    fn parse_method(&mut self) -> Result<HttpMethod, String> {
        let method_str = self.consume_next_token("HTTP method")?;
        Ok(HttpMethod::from_str(&method_str))
    }

    /// 解析 header
    fn parse_header(&mut self) -> Result<(String, String), String> {
        let header_str = self.consume_next_token("header")?;
        parse_header(&header_str)
            .ok_or_else(|| format!("Invalid header format: '{}'", header_str))
    }

    /// 解析普通数据
    fn parse_data(&mut self) -> Result<Vec<u8>, String> {
        let data = self.consume_next_token("data")?;
        Ok(data.into_bytes())
    }

    /// 解析 URL 编码的数据
    fn parse_data_urlencode(&mut self) -> Result<Vec<u8>, String> {
        let data = self.consume_next_token("data")?;
        // 简单处理，实际可能需要更复杂的 URL 编码逻辑
        if let Some((key, value)) = data.split_once('=') {
            let encoded = format!(
                "{}={}",
                urlencoding::encode(key),
                urlencoding::encode(value)
            );
            Ok(encoded.into_bytes())
        } else {
            Ok(urlencoding::encode(&data).as_bytes().to_vec())
        }
    }

    /// 解析表单数据
    fn parse_form_data(&mut self) -> Result<(String, String), String> {
        let form = self.consume_next_token("form data")?;
        if let Some((key, value)) = form.split_once('=') {
            Ok((key.to_string(), value.to_string()))
        } else {
            Err(format!("Invalid form data format: '{}'", form))
        }
    }

    /// 解析认证信息
    fn parse_auth(&mut self) -> Result<(String, String), String> {
        let auth = self.consume_next_token("auth credentials")?;
        let encoded = base64_encode(&auth);
        Ok(("authorization".to_string(), format!("Basic {}", encoded)))
    }

    /// 解析组合的短选项 (如 -sSL)
    fn parse_combined_flags(
        &mut self,
        method: &mut Option<HttpMethod>,
        headers: &mut HashMap<String, String>,
        data: &mut Option<Vec<u8>>,
    ) -> Result<(), String> {
        let token = self.tokens[self.position].clone();
        let flags = token.strip_prefix('-').unwrap_or("");
        self.position += 1;

        for flag in flags.chars() {
            match flag {
                'X' => {
                    *method = Some(self.parse_method()?);
                }
                'H' => {
                    let (key, value) = self.parse_header()?;
                    headers.insert(key, value);
                }
                'd' => {
                    *data = Some(self.parse_data()?);
                }
                // 忽略的标志
                's' | 'S' | 'L' | 'v' | 'i' | 'k' | 'f' => {}
                _ => {
                    // 未知标志，忽略
                }
            }
        }

        Ok(())
    }

    /// 消费下一个 token
    fn consume_next_token(&mut self, expected: &str) -> Result<String, String> {
        self.position += 1;
        if self.position >= self.tokens.len() {
            return Err(format!("Expected {} but reached end of command", expected));
        }
        let token = self.tokens[self.position].clone();
        self.position += 1;
        Ok(token)
    }

    /// 清理 URL (移除反引号等)
    fn clean_url(&self, url: &str) -> String {
        url.trim().trim_matches('`').trim_matches('\'').trim_matches('"').to_string()
    }
}

/// 将 curl 命令分词
///
/// 正确处理单引号、双引号和转义字符
fn tokenize_curl(curl_command: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escape_next = false;

    let chars: Vec<char> = curl_command.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if escape_next {
            current_token.push(ch);
            escape_next = false;
            i += 1;
            continue;
        }

        match ch {
            '\\' => {
                // 在双引号内或未引用时，反斜杠作为转义字符
                if !(!in_double_quote && in_single_quote) {
                    escape_next = true;
                } else {
                    current_token.push(ch);
                }
            }
            '\'' => {
                if !in_double_quote {
                    in_single_quote = !in_single_quote;
                } else {
                    current_token.push(ch);
                }
            }
            '"' => {
                if !in_single_quote {
                    in_double_quote = !in_double_quote;
                } else {
                    current_token.push(ch);
                }
            }
            ' ' | '\t' | '\n' | '\r' => {
                if in_single_quote || in_double_quote {
                    current_token.push(ch);
                } else if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(ch);
            }
        }

        i += 1;
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

/// 解析 header 字符串
///
/// 格式: "Key: Value"
fn parse_header(header: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = header.splitn(2, ':').collect();
    if parts.len() == 2 {
        let key = parts[0].trim().to_string();  // 保持原始大小写
        let value = parts[1].trim().trim_matches('`').to_string();

        // 验证 key 不为空
        if key.is_empty() {
            return None;
        }

        Some((key, value))
    } else {
        None
    }
}

/// Base64 编码
fn base64_encode(input: &str) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let input_bytes = input.as_bytes();
    let mut encoded = String::new();
    let mut i = 0;

    // 处理完整的 3 字节组
    while i + 3 <= input_bytes.len() {
        let chunk = &input_bytes[i..i + 3];

        let n = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);

        encoded.push(BASE64_CHARS[((n >> 18) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[((n >> 6) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[(n & 0x3F) as usize] as char);

        i += 3;
    }

    // 处理剩余的 1-2 字节
    let remaining = input_bytes.len() - i;
    if remaining == 2 {
        let n = ((input_bytes[i] as u32) << 16) | ((input_bytes[i + 1] as u32) << 8);
        encoded.push(BASE64_CHARS[((n >> 18) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[((n >> 6) & 0x3F) as usize] as char);
        encoded.push('=');
    } else if remaining == 1 {
        let n = (input_bytes[i] as u32) << 16;
        encoded.push(BASE64_CHARS[((n >> 18) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize] as char);
        encoded.push('=');
        encoded.push('=');
    }

    encoded
}

/// 验证 URL 是否有效
fn is_valid_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }

    let url = url.trim();

    // 协议前缀
    if url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("ftp://")
        || url.starts_with("ftps://")
        || url.starts_with("ws://")
        || url.starts_with("wss://")
    {
        return true;
    }

    // 相对路径
    if url.starts_with('/') {
        return true;
    }

    // 域名格式 (包含至少一个点，且不以特殊字符开头)
    if url.contains('.') && !url.starts_with('-') && !url.starts_with('.') {
        // 简单验证：检查是否有合理的域名结构
        let parts: Vec<&str> = url.split('/').collect();
        if let Some(domain) = parts.first() {
            // 域名部分不应该包含空格或其他明显的无效字符
            if !domain.contains(' ') && !domain.contains('@') {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod http_test {
    use super::*;

    #[test]
    fn test_curl() {
        let curl_command = r#"curl --request POST 'https://weav3r.dev/favorites' --header 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0' --header 'Accept: text/x-component' --header 'accept-language: en' --header 'content-type: text/plain;charset=UTF-8' --header 'dnt: 1' --header 'next-action: 40b56cda62a77de9e1724496c1e9fdea42e89ab88a' --header 'next-router-state-tree: %5B%22%22%2C%7B%22children%22%3A%5B%22favorites%22%2C%7B%22children%22%3A%5B%22__PAGE__%22%2C%7B%7D%2Cnull%2Cnull%5D%7D%2Cnull%2Cnull%5D%7D%2Cnull%2Cnull%2Ctrue%5D' --header 'origin: https://weav3r.dev' --header 'priority: u=1, i' --header 'referer: https://weav3r.dev/favorites' --header 'sec-ch-ua: "Not(A:Brand";v="8", "Chromium";v="144", "Microsoft Edge";v="144"' --header 'sec-ch-ua-mobile: ?0' --header 'sec-ch-ua'sec-fetch-mode: cors' --header 'sec-fetch-site: same-origin' --header 'Cookie: __tf_verified=1770611010478.e74392adabbaf29579a9e4f62881c8e478c12fc35d286bd365b5a1ff8b5a8f3a; _ga=GA1.1.513731005.1770524621; cf_clearance=OZlKaHRTcyP1STPGJGSOv2As0_ojXe_Zs0yqOyJgVI0-1770524621-1.2.1.1-qbGhAh1vXpKnYedj03Ouv1UylV6Hwhd0IWvzbngnef5CUJaKZcw480VZFwhs6rpZrR0MPfycUDwD.VZuMzli1LGRcp.1JcQDGYEE_S3JuFp_ifx79H3ETkpsTHJVoKCNAmBsnm9xwmMDFKmGMrR0mV1adeCf2a58P9uXwgvN6b1CAVLKmH4p8yr7ASpnH9D1Qbc23MXTtAuYYJwGi5QV.Xe8BhlR0T.Bj7J7dpxJK9A; _ga_PF693NSPW1=GS2.1.s1770524620$o1$g1$t1770524626$j54$l0$h0' --data '[[206]]'"#;
        let tool = HttpTool::from_curl(curl_command);
        assert!(tool.is_ok());
        let tool = tool.unwrap();
        println!("tool info:{:?}", tool);
        assert_eq!(tool.method, HttpMethod::POST);
        assert_eq!(tool.url, "https://weav3r.dev/favorites");
        println!("next-action: {:?}", tool.headers.get("next-action"));
        println!("cookie: {:?}", tool.headers.get("cookie"));
    }

    #[test]
    fn test_curl_basic_get() {
        let curl = "curl https://api.example.com/users";
        let tool = HttpTool::from_curl(curl).unwrap();
        assert_eq!(tool.method, HttpMethod::GET);
        assert_eq!(tool.url, "https://api.example.com/users");
        assert!(tool.body.is_empty());
    }

    #[test]
    fn test_curl_post_with_data() {
        let curl = r#"curl -X POST https://api.example.com/users -d '{"name":"John"}'"#;
        let tool = HttpTool::from_curl(curl).unwrap();
        assert_eq!(tool.method, HttpMethod::POST);
        assert_eq!(tool.url, "https://api.example.com/users");
        assert_eq!(String::from_utf8(tool.body).unwrap(), r#"{"name":"John"}"#);
    }

    #[test]
    fn test_curl_with_headers() {
        let curl = r#"curl https://api.example.com -H "Content-Type: application/json" -H "Authorization: Bearer token123""#;
        let tool = HttpTool::from_curl(curl).unwrap();
        assert_eq!(tool.headers.get("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(tool.headers.get("Authorization"), Some(&"Bearer token123".to_string()));
    }

    #[test]
    fn test_curl_with_basic_auth() {
        let curl = "curl -u user:pass https://api.example.com";
        let tool = HttpTool::from_curl(curl).unwrap();
        assert!(tool.headers.contains_key("authorization"));
        let auth = tool.headers.get("authorization").unwrap();
        assert!(auth.starts_with("Basic "));
    }

    #[test]
    fn test_curl_with_form_data() {
        let curl = r#"curl -X POST https://api.example.com -F "name=John" -F "age=30""#;
        let tool = HttpTool::from_curl(curl).unwrap();
        assert_eq!(tool.method, HttpMethod::POST);
        let body_str = String::from_utf8(tool.body).unwrap();
        assert!(body_str.contains("name=John"));
        assert!(body_str.contains("age=30"));
        assert_eq!(tool.headers.get("content-type"), Some(&"application/x-www-form-urlencoded".to_string()));
    }

    #[test]
    fn test_curl_with_cookies() {
        let curl = r#"curl -b "session=abc123; user=john" https://api.example.com"#;
        let tool = HttpTool::from_curl(curl).unwrap();
        // -b 选项内部会设置为小写的 cookie
        assert_eq!(tool.headers.get("cookie"), Some(&"session=abc123; user=john".to_string()));
    }

    #[test]
    fn test_curl_with_cookie_header() {
        // 测试从浏览器复制的带有 Cookie header 的 curl 命令
        let curl = r#"curl https://api.example.com -H 'Cookie: session=abc123; user=john'"#;
        let tool = HttpTool::from_curl(curl).unwrap();
        // header key 保持原始大小写
        assert_eq!(tool.headers.get("Cookie"), Some(&"session=abc123; user=john".to_string()));
        // 小写的 cookie 不存在
        assert!(tool.headers.get("cookie").is_none());
    }

    #[test]
    fn test_real_browser_curl_with_cookie() {
        // 模拟从浏览器复制的真实 curl 命令,带有 Cookie header
        let curl = r#"curl 'https://weav3r.dev/api' -H 'Cookie: session_id=abc123; auth_token=xyz789' -H 'User-Agent: Mozilla/5.0' --data '{"key":"value"}'"#;
        let tool = HttpTool::from_curl(curl).unwrap();

        // 验证 Cookie 保持大写
        assert!(tool.headers.contains_key("Cookie"));
        let cookie = tool.headers.get("Cookie").unwrap();
        assert!(cookie.contains("session_id=abc123"));
        assert!(cookie.contains("auth_token=xyz789"));

        // 确保可以用大写 "Cookie" 访问
        assert!(tool.headers.get("Cookie").is_some());
        // 小写 "cookie" 不存在(因为保持原始大小写)
        assert!(tool.headers.get("cookie").is_none());
    }

    #[test]
    fn test_curl_with_user_agent() {
        let curl = r#"curl -A "MyBot/1.0" https://api.example.com"#;
        let tool = HttpTool::from_curl(curl).unwrap();
        assert_eq!(tool.headers.get("user-agent"), Some(&"MyBot/1.0".to_string()));
    }

    #[test]
    fn test_curl_with_referer() {
        let curl = r#"curl -e "https://example.com" https://api.example.com"#;
        let tool = HttpTool::from_curl(curl).unwrap();
        assert_eq!(tool.headers.get("referer"), Some(&"https://example.com".to_string()));
    }

    #[test]
    fn test_curl_compressed() {
        let curl = "curl --compressed https://api.example.com";
        let tool = HttpTool::from_curl(curl).unwrap();
        assert_eq!(tool.headers.get("accept-encoding"), Some(&"gzip, deflate, br".to_string()));
    }

    #[test]
    fn test_curl_combined_flags() {
        let curl = r#"curl -sSL https://api.example.com"#;
        let tool = HttpTool::from_curl(curl).unwrap();
        assert_eq!(tool.url, "https://api.example.com");
    }

    #[test]
    fn test_curl_with_backticks() {
        let curl_command = r#"curl --request POST '`https://weav3r.dev/favorites`' --header 'origin: `https://weav3r.dev`' --header 'Cookie: test=value' --data '[[206]]'"#;
        let tool = HttpTool::from_curl(curl_command);
        assert!(tool.is_ok());
        let tool = tool.unwrap();
        assert_eq!(tool.url, "https://weav3r.dev/favorites");
        assert_eq!(
            tool.headers.get("origin"),
            Some(&"https://weav3r.dev".to_string())
        );
        // Cookie 保持大写
        assert_eq!(tool.headers.get("Cookie"), Some(&"test=value".to_string()));
    }

    // 错误情况测试
    #[test]
    fn test_invalid_not_curl() {
        let invalid_commands = vec![
            "random text",
            "wget https://example.com",
            "just some words",
            "",
        ];

        for cmd in invalid_commands {
            let result = HttpTool::from_curl(cmd);
            assert!(result.is_err(), "Expected error for: '{}'", cmd);
        }
    }

    #[test]
    fn test_curl_without_url() {
        let cmd = "curl -X POST -H 'Content-Type: application/json'";
        let result = HttpTool::from_curl(cmd);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "URL not found in curl command");
    }

    #[test]
    fn test_curl_with_invalid_url() {
        let cmd = "curl -X POST randomtext";
        let result = HttpTool::from_curl(cmd);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Invalid URL") || err.contains("URL not found"));
    }

    #[test]
    fn test_valid_url_formats() {
        let valid_commands = vec![
            "curl https://example.com",
            "curl http://example.com",
            "curl ftp://example.com",
            "curl ws://example.com",
            "curl wss://example.com",
            "curl /api/endpoint",
            "curl example.com/path",
            "curl api.example.com",
        ];

        for cmd in valid_commands {
            let result = HttpTool::from_curl(cmd);
            assert!(result.is_ok(), "Expected success for: '{}'", cmd);
        }
    }

    #[test]
    fn test_automatic_post_detection() {
        // 当有 body 数据但没有指定方法时,应该自动使用 POST
        let curl = r#"curl https://api.example.com -d '{"key":"value"}'"#;
        let tool = HttpTool::from_curl(curl).unwrap();
        assert_eq!(tool.method, HttpMethod::POST);
    }

    #[test]
    fn test_method_variants() {
        let methods = vec![
            ("GET", HttpMethod::GET),
            ("POST", HttpMethod::POST),
            ("PUT", HttpMethod::PUT),
            ("DELETE", HttpMethod::DELETE),
            ("PATCH", HttpMethod::PATCH),
            ("HEAD", HttpMethod::HEAD),
            ("OPTIONS", HttpMethod::OPTIONS),
        ];

        for (method_str, expected_method) in methods {
            let curl = format!("curl -X {} https://api.example.com", method_str);
            let tool = HttpTool::from_curl(&curl).unwrap();
            assert_eq!(tool.method, expected_method);
        }
    }

    // 辅助函数测试
    #[test]
    fn test_tokenize_curl() {
        let cmd = r#"curl -X POST "https://example.com" -H 'Content-Type: application/json'"#;
        let tokens = tokenize_curl(cmd);
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], "curl");
        assert_eq!(tokens[1], "-X");
        assert_eq!(tokens[2], "POST");
        assert_eq!(tokens[3], "https://example.com");
        assert_eq!(tokens[4], "-H");
        assert_eq!(tokens[5], "Content-Type: application/json");
    }

    #[test]
    fn test_parse_header() {
        // header key 保持原始大小写
        assert_eq!(
            parse_header("Content-Type: application/json"),
            Some(("Content-Type".to_string(), "application/json".to_string()))
        );
        assert_eq!(
            parse_header("Authorization: Bearer token"),
            Some(("Authorization".to_string(), "Bearer token".to_string()))
        );
        assert_eq!(
            parse_header("cookie: value"),
            Some(("cookie".to_string(), "value".to_string()))
        );
        assert_eq!(parse_header("Invalid"), None);
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("hello"), "aGVsbG8=");
        assert_eq!(base64_encode("user:pass"), "dXNlcjpwYXNz");
        assert_eq!(base64_encode("a"), "YQ==");
        assert_eq!(base64_encode("ab"), "YWI=");
    }

    #[test]
    fn test_is_valid_url() {
        // 有效的 URL
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com"));
        assert!(is_valid_url("ftp://example.com"));
        assert!(is_valid_url("/api/endpoint"));
        assert!(is_valid_url("example.com"));
        assert!(is_valid_url("api.example.com/path"));

        // 无效的 URL
        assert!(!is_valid_url(""));
        assert!(!is_valid_url("randomtext"));
        assert!(!is_valid_url("-something"));
        assert!(!is_valid_url("--flag"));
    }
}

