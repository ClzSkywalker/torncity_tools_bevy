use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurlError {
    #[error("Empty command")]
    EmptyCommand,

    #[error("Not a curl command: expected 'curl', got '{0}'")]
    NotCurlCommand(String),

    #[error("URL not found in curl command")]
    UrlNotFound,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid header format: {0}")]
    InvalidHeader(String),

    #[error("Invalid method: {0}")]
    InvalidMethod(String),

    #[error("Invalid form data: {0}")]
    InvalidFormData(String),

    #[error("Missing value for option: {0}")]
    MissingValue(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Tokenizer error: {0}")]
    TokenizerError(String),
}

pub type Result<T> = std::result::Result<T, CurlError>;
