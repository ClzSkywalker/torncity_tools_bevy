use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, StorageError};
use crate::file_store::FileStore;

#[derive(Debug, Clone)]
pub struct CachedBinary {
    pub bytes: Vec<u8>,
    pub ext: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CacheStore {
    root_dir: PathBuf,
}

impl CacheStore {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }

    pub fn ensure_cache_dir(&self, namespace: &str) -> Result<PathBuf> {
        let cache_dir = self.root_dir.join(namespace);
        fs::create_dir_all(&cache_dir)
            .map_err(|e| Self::backend_error(&format!("create cache dir `{}`", cache_dir.display()), e))?;
        Ok(cache_dir)
    }

    pub fn load_cache_bytes(&self, namespace: &str, key: &str) -> Result<Option<CachedBinary>> {
        let cache_dir = self.ensure_cache_dir(namespace)?;
        let hash = Self::sha256_base64url(key);
        let Some(path) = self.resolve_cached_file_path(&cache_dir, &hash)? else {
            return Ok(None);
        };
        
        let bytes = FileStore::read_bytes(&path)?;
        let ext = path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.to_ascii_lowercase());
        let content_type = ext
            .as_deref()
            .and_then(Self::content_type_from_ext)
            .map(str::to_string);

        Ok(Some(CachedBinary {
            bytes,
            ext,
            content_type,
        }))
    }

    pub fn save_cache_bytes(
        &self,
        namespace: &str,
        key: &str,
        bytes: &[u8],
        hint_ext: Option<&str>,
    ) -> Result<()> {
        let cache_dir = self.ensure_cache_dir(namespace)?;
        let hash = Self::sha256_base64url(key);
        let ext = Self::infer_cache_extension(key, hint_ext);
        let path = cache_dir.join(format!("{hash}.{ext}"));
        bevy_log::info!("bevy_storage: save cache bytes path: {}", path.display());

        self.cleanup_old_cache_entries(&cache_dir, &hash, &path)?;
        FileStore::write_bytes(&path, bytes)
    }

    fn resolve_cached_file_path(&self, cache_dir: &Path, hash: &str) -> Result<Option<PathBuf>> {
        let entries = fs::read_dir(cache_dir).map_err(|e| {
            Self::backend_error(
                &format!("read cache dir `{}`", cache_dir.display()),
                e,
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                Self::backend_error(
                    &format!("iterate cache dir `{}`", cache_dir.display()),
                    e,
                )
            })?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.starts_with(hash) {
                return Ok(Some(entry.path()));
            }
        }
        Ok(None)
    }

    fn cleanup_old_cache_entries(&self, cache_dir: &Path, hash: &str, keep_path: &Path) -> Result<()> {
        let entries = fs::read_dir(cache_dir).map_err(|e| {
            Self::backend_error(
                &format!("read cache dir `{}`", cache_dir.display()),
                e,
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                Self::backend_error(
                    &format!("iterate cache dir `{}`", cache_dir.display()),
                    e,
                )
            })?;
            let entry_path = entry.path();
            if entry_path == keep_path {
                continue;
            }

            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.starts_with(hash) {
                fs::remove_file(&entry_path).map_err(|e| {
                    Self::backend_error(
                        &format!("remove stale cache file `{}`", entry_path.display()),
                        e,
                    )
                })?;
            }
        }

        Ok(())
    }

    fn backend_error(context: &str, err: impl std::fmt::Display) -> StorageError {
        bevy_log::warn!("bevy_storage: {context} failed: {err}");
        StorageError::BackendError(format!("{context} failed: {err}"))
    }

    fn sha256_base64url(value: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        URL_SAFE_NO_PAD.encode(hasher.finalize())
    }

    fn infer_cache_extension(key: &str, hint_ext: Option<&str>) -> String {
        if let Some(ext) = Self::extension_from_key(key).and_then(Self::normalize_extension) {
            return ext;
        }
        if let Some(ext) = hint_ext.and_then(Self::normalize_extension) {
            return ext;
        }
        "png".to_string()
    }

    fn extension_from_key(key: &str) -> Option<&str> {
        let key_no_query = key.split('?').next().unwrap_or(key);
        Path::new(key_no_query)
            .extension()
            .and_then(|value| value.to_str())
    }

    fn normalize_extension(ext: &str) -> Option<String> {
        let lower = ext.to_ascii_lowercase();
        match lower.as_str() {
            "png" => Some("png".to_string()),
            "jpg" | "jpeg" => Some("jpeg".to_string()),
            "webp" => Some("webp".to_string()),
            "gif" => Some("gif".to_string()),
            "bmp" => Some("bmp".to_string()),
            _ => None,
        }
    }

    fn content_type_from_ext(ext: &str) -> Option<&'static str> {
        match ext {
            "png" => Some("image/png"),
            "jpeg" => Some("image/jpeg"),
            "webp" => Some("image/webp"),
            "gif" => Some("image/gif"),
            "bmp" => Some("image/bmp"),
            _ => None,
        }
    }
}
