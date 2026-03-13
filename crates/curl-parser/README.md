# curl-parser

一个用 Rust 编写的 curl 命令解析库，专门用于解析从浏览器复制的 curl 命令。

## 功能特性

- ✅ 解析浏览器复制的 curl 命令
- ✅ 支持单引号、双引号
- ✅ 支持转义字符
- ✅ 支持 curl 常用选项

## 支持的 curl 选项

### 方法
- `-X`, `--request`

### Header
- `-H`, `--header`
- `-A`, `--user-agent`
- `-e`, `--referer`
- `-b`, `--cookie`

### Body
- `-d`, `--data`
- `--data-raw`
- `--data-binary`
- `--data-urlencode`

### 表单
- `-F`, `--form`

### 认证
- `-u`, `--user`

### 其他
- `--compressed`

## 安装

```toml
[dependencies]
curl-parser = "0.1"
```

## 使用示例

```rust
use curl_parser::parse_curl;

fn main() {
    let curl_command = r#"curl -X POST 'https://api.example.com/data' \
        -H 'Content-Type: application/json' \
        -H 'Authorization: Bearer token123' \
        -d '{"key":"value"}'"#;

    let request = parse_curl(curl_command).unwrap();

    println!("URL: {}", request.url);
    println!("Method: {:?}", request.method);
    println!("Headers: {:?}", request.headers);
    println!("Body: {:?}", request.body);
}
```

## API

### `parse_curl`

从 curl 命令字符串解析出 `CurlRequest`。

```rust
pub fn parse_curl(curl_command: &str) -> Result<CurlRequest, CurlError>
```

### CurlRequest

解析结果结构体：

```rust
pub struct CurlRequest {
    pub url: String,              // 请求 URL
    pub method: HttpMethod,       // HTTP 方法
    pub headers: HashMap<String, String>,  // 请求头
    pub body: Option<Vec<u8>>,    // 请求体
    pub form_data: Vec<(String, String)>, // 表单数据
    pub auth: Option<Auth>,       // 认证信息
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub cookies: Option<String>,
    pub compressed: bool,
}
```

## 许可证

MIT
