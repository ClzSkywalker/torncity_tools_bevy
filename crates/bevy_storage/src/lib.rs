mod app_config;
mod app_paths;
mod cache_store;
mod error;
mod file_store;
mod plugin;
mod storage;

pub use app_config::AppConfigStore;
pub use app_paths::AppPaths;
pub use cache_store::{CacheStore, CachedBinary};
pub use error::{Result, StorageError};
pub use file_store::FileStore;
pub use plugin::StoragePlugin;
pub use storage::StorageManager;

// 预导出常用 traits
pub use serde::{Deserialize, Serialize};
