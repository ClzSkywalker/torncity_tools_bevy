pub mod ast;
pub mod error;
pub mod parser;
pub mod tokenizer;

pub use ast::{Auth, CurlRequest, HttpMethod};
pub use error::{CurlError, Result};
pub use parser::parse;

pub fn parse_curl(curl_command: &str) -> Result<CurlRequest> {
    parser::parse(curl_command)
}
