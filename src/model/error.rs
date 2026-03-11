use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum MyError {
    #[error("Network Request Failed, code: {0}, url: {1}")]
    NetworkCode(i64, String),
    #[error("Response text is None")]
    ResponseTextIsNone,
    #[error("Json Parse Failed, error: {0}")]
    JsonParse(String),
    #[error("Channel Error, error: {0}")]
    ChannelError(String),
}

impl Into<String> for MyError {
    fn into(self) -> String {
        self.to_string()
    }
}