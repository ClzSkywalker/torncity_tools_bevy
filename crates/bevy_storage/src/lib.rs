mod error;
mod plugin;
mod storage;

pub use error::{Result, StorageError};
pub use plugin::StoragePlugin;
pub use storage::StorageManager;

// 预导出常用 traits
pub use serde::{Deserialize, Serialize};
