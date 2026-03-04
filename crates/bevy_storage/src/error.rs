use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to serialize data: {0}")]
    SerializationError(String),

    #[error("Failed to deserialize data: {0}")]
    DeserializationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Storage backend error: {0}")]
    BackendError(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;
