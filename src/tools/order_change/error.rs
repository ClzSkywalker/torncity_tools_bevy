use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderChangeError {
    LengthMismatch { current: usize, target: usize },
    ContentMismatch,
}

impl fmt::Display for OrderChangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthMismatch { current, target } => {
                write!(f, "Length mismatch: current={} target={}", current, target)
            }
            Self::ContentMismatch => {
                write!(f, "Content mismatch between current and target data")
            }
        }
    }
}

impl std::error::Error for OrderChangeError {}

pub type Result<T> = std::result::Result<T, OrderChangeError>;
