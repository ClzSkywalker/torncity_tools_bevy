use std::fs;
use std::path::PathBuf;

use crate::error::{Result, StorageError};

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub temp_dir: PathBuf,
}

impl AppPaths {
    pub fn detect() -> Self {
        let app_name = Self::detect_app_name();
        let config_base = sysdirs::config_dir()
            .or_else(sysdirs::data_local_dir)
            .or_else(sysdirs::data_dir)
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));
        let data_base = sysdirs::data_local_dir()
            .or_else(sysdirs::data_dir)
            .or_else(|| Some(config_base.clone()))
            .unwrap_or_else(|| PathBuf::from("."));
        let cache_base = sysdirs::cache_dir()
            .or_else(sysdirs::data_local_dir)
            .or_else(|| Some(config_base.clone()))
            .unwrap_or_else(|| PathBuf::from("."));

        let config_dir = config_base.join(&app_name);
        let data_dir = data_base.join(&app_name);
        let cache_dir = cache_base.join(&app_name);
        let temp_dir = std::env::temp_dir();

        Self {
            config_dir,
            data_dir,
            cache_dir,
            temp_dir,
        }
    }

    fn detect_app_name() -> String {
        let from_exe = std::env::current_exe()
            .ok()
            .as_ref()
            .and_then(|path| path.file_stem())
            .and_then(|stem| stem.to_str())
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .map(ToOwned::to_owned);

        from_exe.unwrap_or_else(|| "app".to_string())
    }

    pub fn ensure_all(&self) -> Result<()> {
        self.ensure_dir(&self.config_dir)?;
        self.ensure_dir(&self.data_dir)?;
        self.ensure_dir(&self.cache_dir)?;
        self.ensure_dir(&self.temp_dir)?;
        Ok(())
    }

    pub fn ensure_dir(&self, dir: &PathBuf) -> Result<()> {
        fs::create_dir_all(dir).map_err(|e| {
            StorageError::BackendError(format!("create dir `{}` failed: {e}", dir.display()))
        })
    }
}
