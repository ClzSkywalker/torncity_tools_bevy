use std::path::PathBuf;

use serde::{Serialize, de::DeserializeOwned};

use crate::app_paths::AppPaths;
use crate::error::{Result, StorageError};
use crate::file_store::FileStore;

const APP_CONFIG_FILE_NAME: &str = "app_config.json";

#[derive(Debug, Clone)]
pub struct AppConfigStore {
    path: PathBuf,
}

impl AppConfigStore {
    pub fn new(paths: &AppPaths) -> Self {
        Self {
            path: paths.config_dir.join(APP_CONFIG_FILE_NAME),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn load<T: DeserializeOwned>(&self) -> Result<T> {
        if !FileStore::exists(&self.path) {
            return Err(StorageError::NotFound(self.path.display().to_string()));
        }
        let text = FileStore::read_text(&self.path)?;
        serde_json::from_str(&text).map_err(|e| StorageError::DeserializationError(e.to_string()))
    }

    pub fn load_or_default<T: DeserializeOwned + Default>(&self) -> Result<T> {
        match self.load::<T>() {
            Ok(value) => Ok(value),
            Err(StorageError::NotFound(_)) => Ok(T::default()),
            Err(StorageError::DeserializationError(_)) => Ok(T::default()),
            Err(err) => Err(err),
        }
    }

    pub fn save<T: Serialize>(&self, value: &T) -> Result<()> {
        bevy_log::info!("bevy_storage: save app config path: {}", self.path.display());
        let text = serde_json::to_string_pretty(value)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        FileStore::write_text(&self.path, &text)
    }
}
