use std::path::{Path, PathBuf};

use bevy_ecs::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

use crate::app_config::AppConfigStore;
use crate::app_paths::AppPaths;
use crate::cache_store::{CacheStore, CachedBinary};
use crate::error::Result;
use crate::file_store::FileStore;

/// 存储管理器：统一提供应用配置、目录路径、通用文件读写、缓存文件读写能力
#[derive(Resource, Debug, Clone)]
pub struct StorageManager {
    paths: AppPaths,
    app_config: AppConfigStore,
    cache_store: CacheStore,
}

impl StorageManager {
    pub fn new() -> Self {
        let paths = AppPaths::detect();
        let _ = paths.ensure_all();
        let app_config = AppConfigStore::new(&paths);
        let cache_store = CacheStore::new(paths.cache_dir.clone());
        Self {
            paths,
            app_config,
            cache_store,
        }
    }

    pub fn app_paths(&self) -> &AppPaths {
        &self.paths
    }

    pub fn app_config_path(&self) -> &PathBuf {
        self.app_config.path()
    }

    pub fn save_app_config<T: Serialize>(&self, value: &T) -> Result<()> {
        self.app_config.save(value)
    }

    pub fn load_app_config<T: DeserializeOwned>(&self) -> Result<T> {
        self.app_config.load()
    }

    pub fn load_app_config_or_default<T: DeserializeOwned + Default>(&self) -> Result<T> {
        self.app_config.load_or_default()
    }

    pub fn read_file_bytes(&self, path: &Path) -> Result<Vec<u8>> {
        FileStore::read_bytes(path)
    }

    pub fn write_file_bytes(&self, path: &Path, bytes: &[u8]) -> Result<()> {
        FileStore::write_bytes(path, bytes)
    }

    pub fn read_file_text(&self, path: &Path) -> Result<String> {
        FileStore::read_text(path)
    }

    pub fn write_file_text(&self, path: &Path, text: &str) -> Result<()> {
        FileStore::write_text(path, text)
    }

    pub fn ensure_cache_dir(&self, namespace: &str) -> Result<PathBuf> {
        self.cache_store.ensure_cache_dir(namespace)
    }

    pub fn load_cache_bytes(&self, namespace: &str, key: &str) -> Result<Option<CachedBinary>> {
        self.cache_store.load_cache_bytes(namespace, key)
    }

    pub fn save_cache_bytes(
        &self,
        namespace: &str,
        key: &str,
        bytes: &[u8],
        hint_ext: Option<&str>,
    ) -> Result<()> {
        self.cache_store
            .save_cache_bytes(namespace, key, bytes, hint_ext)
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new()
    }
}
