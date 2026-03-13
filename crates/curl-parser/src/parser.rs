use crate::ast::{Auth, CurlRequest, HttpMethod};
use crate::error::{CurlError, Result};
use crate::tokenizer::{Token, Tokenizer};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(curl_command: &str) -> Result<Self> {
        let tokens = Tokenizer::new(curl_command).tokenize()?;

        if tokens.is_empty() {
            return Err(CurlError::EmptyCommand);
        }

        let first = &tokens[0];
        match first {
            Token::Word(w) => {
                let is_curl = w == "curl"
                    || w == "curl.exe"
                    || w.starts_with("$curl")
                    || w.starts_with("%curl")
                    || w.starts_with("curl");
                if !is_curl {
                    if w.starts_with('-') {
                    } else {
                        return Err(CurlError::NotCurlCommand(w.clone()));
                    }
                }
            }
            Token::Flag(_) => {}
            Token::Eof => {
                return Err(CurlError::NotCurlCommand("unknown".to_string()));
            }
        }

        Ok(Self { tokens, position: 0 })
    }

    pub fn parse(&mut self) -> Result<CurlRequest> {
        let mut request = CurlRequest::default();

        while self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();

            match token {
                Token::Word(w) if w == "curl" || w == "curl.exe" || w.starts_with("$curl") || w.starts_with("%curl") || w.starts_with("curl") => {
                    self.position += 1;
                }
                Token::Flag(flag) => {
                    self.parse_flag(&flag, &mut request)?;
                }
                Token::Word(word) => {
                    if request.url.is_empty() && is_valid_url(&word) {
                        request.url = word;
                    }
                    self.position += 1;
                }
                Token::Eof => {
                    break;
                }
            }
        }

        if request.url.is_empty() {
            return Err(CurlError::UrlNotFound);
        }

        Ok(request)
    }

    fn parse_flag(&mut self, flag: &str, request: &mut CurlRequest) -> Result<()> {
        match flag {
            "-X" | "--request" => {
                let method = self.next_word("HTTP method")?;
                request.method = HttpMethod::from_str(&method);
            }
            "-H" | "--header" => {
                let header = self.next_word("header")?;
                if let Some((key, value)) = parse_header(&header) {
                    request.headers.insert(key, value);
                } else {
                    return Err(CurlError::InvalidHeader(header));
                }
            }
            "-d" | "--data" | "--data-raw" => {
                let data = self.next_word("data")?;
                request.body = Some(data.into_bytes());
                if request.method == HttpMethod::GET {
                    request.method = HttpMethod::POST;
                }
            }
            "--data-binary" => {
                let data = self.next_word("data")?;
                request.body = Some(data.into_bytes());
                if request.method == HttpMethod::GET {
                    request.method = HttpMethod::POST;
                }
            }
            "--data-urlencode" => {
                let data = self.next_word("data")?;
                let encoded = url_encode(&data);
                request.body = Some(encoded.into_bytes());
                if request.method == HttpMethod::GET {
                    request.method = HttpMethod::POST;
                }
            }
            "-F" | "--form" => {
                let form = self.next_word("form data")?;
                if let Some((key, value)) = form.split_once('=') {
                    request.form_data.push((key.to_string(), value.to_string()));
                } else {
                    return Err(CurlError::InvalidFormData(form));
                }
            }
            "-u" | "--user" => {
                let auth_str = self.next_word("auth credentials")?;
                if let Some((username, password)) = auth_str.split_once(':') {
                    request.auth = Some(Auth {
                        username: username.to_string(),
                        password: password.to_string(),
                    });
                    let encoded = base64_encode(&auth_str);
                    request.headers.insert(
                        "authorization".to_string(),
                        format!("Basic {}", encoded),
                    );
                }
            }
            "-A" | "--user-agent" => {
                let agent = self.next_word("user agent")?;
                request.headers.insert("user-agent".to_string(), agent);
            }
            "-e" | "--referer" => {
                let referer = self.next_word("referer")?;
                request.headers.insert("referer".to_string(), referer);
            }
            "-b" | "--cookie" => {
                let cookie = self.next_word("cookie")?;
                request.headers.insert("cookie".to_string(), cookie);
            }
            "--compressed" => {
                request
                    .headers
                    .insert("accept-encoding".to_string(), "gzip, deflate".to_string());
                request.compressed = true;
                self.position += 1;
            }
            "-k" | "--insecure" | "-s" | "--silent" | "-v" | "--verbose" | "-i" | "--include"
            | "-L" | "--location" | "-f" | "--fail" => {
                self.position += 1;
            }
            "-o" | "--output" | "-O" | "--remote-name" => {
                self.next_word("output file")?;
            }
            "--max-time" | "--connect-timeout" | "-m" | "--max-redirs" => {
                self.position += 1;
                if self.position < self.tokens.len() {
                    self.position += 1;
                }
            }
            _ => {
                if flag.starts_with('-') && !flag.starts_with("--") && flag.len() > 2 {
                    self.parse_combined_flags(flag, request)?;
                    self.position += 1;
                } else {
                    self.position += 1;
                }
            }
        }

        Ok(())
    }

    fn parse_combined_flags(&mut self, flags: &str, request: &mut CurlRequest) -> Result<()> {
        let chars: Vec<char> = flags.strip_prefix('-').unwrap_or("").chars().collect();
        let mut args_consumed = 0;

        for c in chars {
            match c {
                'X' => {
                    let method = self.next_word("HTTP method")?;
                    request.method = HttpMethod::from_str(&method);
                    args_consumed += 1;
                }
                'H' => {
                    let header = self.next_word("header")?;
                    if let Some((key, value)) = parse_header(&header) {
                        request.headers.insert(key, value);
                    }
                    args_consumed += 1;
                }
                'd' => {
                    let data = self.next_word("data")?;
                    request.body = Some(data.into_bytes());
                    if request.method == HttpMethod::GET {
                        request.method = HttpMethod::POST;
                    }
                    args_consumed += 1;
                }
                's' | 'S' | 'L' | 'v' | 'i' | 'k' | 'f' | 'o' | 'O' => {}
                _ => {}
            }
        }

        self.position += args_consumed;
        Ok(())
    }

    fn next_word(&mut self, expected: &str) -> Result<String> {
        self.position += 1;
        if self.position >= self.tokens.len() {
            return Err(CurlError::MissingValue(expected.to_string()));
        }

        match &self.tokens[self.position] {
            Token::Word(w) => {
                self.position += 1;
                Ok(w.clone())
            }
            _ => Err(CurlError::MissingValue(expected.to_string())),
        }
    }
}

fn parse_header(header: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = header.splitn(2, ':').collect();
    if parts.len() == 2 {
        let key = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();
        if key.is_empty() {
            return None;
        }
        Some((key, value))
    } else {
        None
    }
}

fn is_valid_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }

    let url = url.trim();

    if url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("ftp://")
        || url.starts_with("ftps://")
        || url.starts_with("ws://")
        || url.starts_with("wss://")
    {
        return true;
    }

    if url.starts_with('/') {
        return true;
    }

    if url.contains('.') && !url.starts_with('-') && !url.starts_with('.') {
        let parts: Vec<&str> = url.split('/').collect();
        if let Some(domain) = parts.first() {
            if !domain.contains(' ') && !domain.contains('@') {
                return true;
            }
        }
    }

    false
}

fn base64_encode(input: &str) -> String {
    const BASE64_CHARS: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let input_bytes = input.as_bytes();
    let mut encoded = String::new();
    let mut i = 0;

    while i + 3 <= input_bytes.len() {
        let chunk = &input_bytes[i..i + 3];
        let n = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);

        encoded.push(BASE64_CHARS[((n >> 18) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[((n >> 6) & 0x3F) as usize] as char);
        encoded.push(BASE64_CHARS[(n & 0x3F) as usize] as char);

        i += 3;
    }

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

fn url_encode(data: &str) -> String {
    if let Some((key, value)) = data.split_once('=') {
        format!(
            "{}={}",
            urlencoding_encode(key),
            urlencoding_encode(value)
        )
    } else {
        urlencoding_encode(data)
    }
}

fn urlencoding_encode(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}

pub fn parse(curl_command: &str) -> Result<CurlRequest> {
    let mut parser = Parser::new(curl_command)?;
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let curl = "curl https://example.com";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.method, HttpMethod::GET);
    }

    #[test]
    fn test_post() {
        let curl = r#"curl -X POST https://example.com -d '{"key":"value"}'"#;
        let result = parse(curl).unwrap();
        assert_eq!(result.method, HttpMethod::POST);
        assert_eq!(result.body, Some(r#"{"key":"value"}"#.as_bytes().to_vec()));
    }

    #[test]
    fn test_headers() {
        let curl =
            r#"curl https://example.com -H "Content-Type: application/json" -H "Authorization: Bearer token""#;
        let result = parse(curl).unwrap();
        assert_eq!(
            result.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(
            result.headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
    }

    #[test]
    fn test_auth() {
        let curl = "curl -u user:pass https://example.com";
        let result = parse(curl).unwrap();
        assert!(result.auth.is_some());
        assert!(result.headers.contains_key("authorization"));
    }

    #[test]
    fn test_cookie() {
        let curl = r#"curl -b "session=abc123" https://example.com"#;
        let result = parse(curl).unwrap();
        assert_eq!(
            result.headers.get("cookie"),
            Some(&"session=abc123".to_string())
        );
    }

    #[test]
    fn test_compressed() {
        let curl = "curl --compressed https://example.com";
        let result = parse(curl).unwrap();
        assert!(result.compressed);
        assert_eq!(
            result.headers.get("accept-encoding"),
            Some(&"gzip, deflate".to_string())
        );
    }

    #[test]
    fn test_curl_exe() {
        let curl = "curl.exe https://example.com";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.method, HttpMethod::GET);
    }

    #[test]
    fn test_cmd_curl_exe_uri() {
        let curl = r#"curl.exe -Uri https://example.com"#;
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
    }

    #[test]
    fn test_powershell_format() {
        let curl = r#"curl.exe -Uri "https://example.com" -Method POST"#;
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.method, HttpMethod::POST);
    }

    #[test]
    fn test_multiline_curl() {
        let curl = "curl https://example.com \\\n -X POST \\\n -H 'Content-Type: application/json'";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.method, HttpMethod::POST);
        assert_eq!(
            result.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_windows_cmd_variable() {
        let curl = r#"%curl% -X POST https://example.com"#;
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.method, HttpMethod::POST);
    }

    #[test]
    fn test_bash_variable() {
        let curl = "$curl -X POST https://example.com";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.method, HttpMethod::POST);
    }

    #[test]
    fn test_combined_short_flags() {
        let curl = "curl -X POST -H 'Content-Type: application/json' -d '{\"test\":1}' https://example.com";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.method, HttpMethod::POST);
        assert_eq!(
            result.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(result.body, Some(r#"{"test":1}"#.as_bytes().to_vec()));
    }

    #[test]
    fn test_verbose_and_silent() {
        let curl = "curl -sv https://example.com";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
    }

    #[test]
    fn test_url_with_query_string() {
        let curl = "curl 'https://example.com/api?param1=value1&param2=value2'";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com/api?param1=value1&param2=value2");
    }

    #[test]
    fn test_follow_redirects() {
        let curl = "curl -L https://example.com";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
    }

    #[test]
    fn test_output_file() {
        let curl = "curl -o output.html https://example.com";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
    }

    #[test]
    fn test_timeout_option() {
        let curl = "curl -m 30 https://example.com";
        let result = parse(curl).unwrap();
        assert_eq!(result.url, "https://example.com");
    }

    #[test]
    fn test_user_agent() {
        let curl = r#"curl -A "Mozilla/5.0" https://example.com"#;
        let result = parse(curl).unwrap();
        assert_eq!(
            result.headers.get("user-agent"),
            Some(&"Mozilla/5.0".to_string())
        );
    }

    #[test]
    fn test_referer() {
        let curl = "-e https://google.com curl https://example.com";
        let result = parse(curl).unwrap();
        assert_eq!(
            result.headers.get("referer"),
            Some(&"https://google.com".to_string())
        );
    }
}
