use crate::error::{Result, StorageError};
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// 配置存储管理器 (Bevy Resource)
#[derive(Resource)]
pub struct StorageManager {
    file_path: PathBuf,
    cache: Mutex<HashMap<String, serde_json::Value>>,
}

impl StorageManager {
    /// 创建新的存储管理器
    pub fn new(organization: &str, application: &str) -> Self {
        let file_path = Self::resolve_storage_file_path(organization, application);
        let cache = match Self::read_store_map(&file_path) {
            Ok(cache) => cache,
            Err(err) => {
                bevy_log::warn!("Failed to load storage file `{}`: {err}", file_path.display());
                HashMap::new()
            }
        };

        Self {
            file_path,
            cache: Mutex::new(cache),
        }
    }

    /// 保存配置
    pub fn save<T: Serialize>(&mut self, key: &str, value: &T) -> Result<()> {
        let value =
            serde_json::to_value(value).map_err(|e| StorageError::SerializationError(e.to_string()))?;
        let mut cache = self
            .cache
            .lock()
            .map_err(|e| StorageError::BackendError(e.to_string()))?;
        cache.insert(key.to_string(), value);
        self.persist_store_map(&cache)
    }

    /// 加载配置
    pub fn load<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T> {
        let cache = self
            .cache
            .lock()
            .map_err(|e| StorageError::BackendError(e.to_string()))?;
        let value = cache
            .get(key)
            .cloned()
            .ok_or_else(|| StorageError::KeyNotFound(key.to_string()))?;
        serde_json::from_value(value).map_err(|e| StorageError::DeserializationError(e.to_string()))
    }

    /// 检查键是否存在
    pub fn exists(&self, key: &str) -> bool {
        self.cache
            .lock()
            .map(|cache| cache.contains_key(key))
            .unwrap_or(false)
    }

    fn resolve_storage_file_path(organization: &str, application: &str) -> PathBuf {
        let base_dir = sysdirs::config_dir()
            .or_else(sysdirs::data_local_dir)
            .or_else(sysdirs::data_dir)
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));

        base_dir
            .join(organization)
            .join(application)
            .join("storage.json")
    }

    fn read_store_map(path: &Path) -> Result<HashMap<String, serde_json::Value>> {
        if !path.exists() {
            return Ok(HashMap::new());
        }

        let content =
            fs::read_to_string(path).map_err(|e| StorageError::BackendError(e.to_string()))?;
        if content.trim().is_empty() {
            return Ok(HashMap::new());
        }

        serde_json::from_str(&content).map_err(|e| StorageError::DeserializationError(e.to_string()))
    }

    fn persist_store_map(&self, cache: &HashMap<String, serde_json::Value>) -> Result<()> {
        if let Some(parent_dir) = self.file_path.parent() {
            fs::create_dir_all(parent_dir).map_err(|e| StorageError::BackendError(e.to_string()))?;
        }

        let content =
            serde_json::to_string_pretty(cache).map_err(|e| StorageError::SerializationError(e.to_string()))?;
        fs::write(&self.file_path, content).map_err(|e| StorageError::BackendError(e.to_string()))
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new("bevy_storage", "dev")
    }
}
