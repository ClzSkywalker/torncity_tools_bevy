use std::fs;
use std::path::Path;

use crate::error::{Result, StorageError};

pub struct FileStore;

impl FileStore {
    pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
        fs::read(path).map_err(|e| {
            StorageError::BackendError(format!("read bytes `{}` failed: {e}", path.display()))
        })
    }

    pub fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                StorageError::BackendError(format!("create dir `{}` failed: {e}", parent.display()))
            })?;
        }
        fs::write(path, bytes).map_err(|e| {
            StorageError::BackendError(format!("write bytes `{}` failed: {e}", path.display()))
        })
    }

    pub fn read_text(path: &Path) -> Result<String> {
        fs::read_to_string(path).map_err(|e| {
            StorageError::BackendError(format!("read text `{}` failed: {e}", path.display()))
        })
    }

    pub fn write_text(path: &Path, text: &str) -> Result<()> {
        Self::write_bytes(path, text.as_bytes())
    }

    pub fn exists(path: &Path) -> bool {
        path.exists()
    }
}
