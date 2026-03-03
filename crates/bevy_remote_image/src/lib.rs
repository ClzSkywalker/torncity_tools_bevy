use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
use std::{
    fs,
    path::{Path, PathBuf},
};

use bevy_app::prelude::*;
use bevy_asset::{RenderAssetUsages, prelude::*};
use bevy_ecs::prelude::*;
use bevy_image::{CompressedImageFormats, ImageSampler, ImageType, prelude::*};
use bevy_log::prelude::*;
use bevy_tasks::{IoTaskPool, Task, futures_lite::future};
use bevy_ui::prelude::*;
#[cfg(all(feature = "disk-cache", target_os = "android"))]
use jni::objects::{JObject, JString};

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
use sha2::{Digest, Sha256};

pub struct RemoteImagePlugin;

impl Plugin for RemoteImagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RemoteImageCache>()
            .add_systems(Update, queue_remote_images)
            .add_systems(Update, apply_downloaded_images);
    }
}

#[derive(Component, Debug, Clone)]
pub struct RemoteImageTarget {
    pub url: String,
}

#[derive(Component)]
struct ImageDownloadTask(Task<ImageDownloadResult>);

#[derive(Resource, Debug)]
struct RemoteImageCache {
    in_flight: HashSet<String>,
    url_to_handle: HashMap<String, Handle<Image>>,
    #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
    cache_absolute_dir: PathBuf,
}

#[derive(Debug)]
struct ImageDownloadResult {
    url: String,
    bytes: Option<Vec<u8>>,
    content_type: Option<String>,
}

impl Default for RemoteImageCache {
    fn default() -> Self {
        #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
        let cache_absolute_dir = {
            let absolute = resolve_cache_dir();
            let _ = fs::create_dir_all(&absolute);
            absolute
        };

        Self {
            in_flight: HashSet::new(),
            url_to_handle: HashMap::new(),
            #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
            cache_absolute_dir,
        }
    }
}

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32"), not(target_os = "android")))]
fn resolve_cache_dir() -> PathBuf {
    PathBuf::from("assets").join(".http_cache")
}

#[cfg(all(feature = "disk-cache", target_os = "android"))]
fn resolve_cache_dir() -> PathBuf {
    // Android APK assets are read-only. Use app cache dir instead.
    android_cache_dir().unwrap_or_else(|| std::env::temp_dir().join("bevy_remote_image_cache"))
}

#[cfg(all(feature = "disk-cache", target_os = "android"))]
fn android_cache_dir() -> Option<PathBuf> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.ok()?;
    let mut env = vm.attach_current_thread().ok()?;

    let activity = unsafe { JObject::from_raw(ctx.context().cast()) };
    let cache_dir = env
        .call_method(activity, "getCacheDir", "()Ljava/io/File;", &[])
        .ok()?
        .l()
        .ok()?;
    let path = env
        .call_method(cache_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
        .ok()?
        .l()
        .ok()?;
    let path: JString = path.into();
    let path: String = env.get_string(&path).ok()?.into();
    Some(PathBuf::from(path))
}

fn queue_remote_images(
    mut commands: Commands,
    #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))] mut images: ResMut<
        Assets<Image>,
    >,
    mut cache: ResMut<RemoteImageCache>,
    query: Query<(Entity, &RemoteImageTarget), Added<RemoteImageTarget>>,
) {
    for (entity, target) in &query {
        if let Some(handle) = cache.url_to_handle.get(&target.url) {
            commands
                .entity(entity)
                .insert(ImageNode::new(handle.clone()));
            continue;
        }

        #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
        if let Some(image) = load_cached_image(&target.url, &cache) {
            let handle = images.add(image);
            cache
                .url_to_handle
                .insert(target.url.clone(), handle.clone());
            commands.entity(entity).insert(ImageNode::new(handle));
            continue;
        }

        if cache.in_flight.contains(&target.url) {
            continue;
        }

        cache.in_flight.insert(target.url.clone());
        let url = target.url.clone();

        let task_pool = IoTaskPool::get();
        let task = task_pool.spawn(async move { download_image(&url).await });
        commands.spawn(ImageDownloadTask(task));
    }
}

fn apply_downloaded_images(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut cache: ResMut<RemoteImageCache>,
    mut task_query: Query<(Entity, &mut ImageDownloadTask)>,
    target_query: Query<(Entity, &RemoteImageTarget)>,
) {
    for (task_entity, mut task) in &mut task_query {
        let Some(result) = future::block_on(future::poll_once(&mut task.0)) else {
            continue;
        };

        cache.in_flight.remove(&result.url);

        if let Some(bytes) = result.bytes {
            if let Some(image) =
                decode_image_from_response(&result.url, &bytes, result.content_type.as_deref())
            {
                #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
                persist_downloaded_image(
                    &result.url,
                    &bytes,
                    result.content_type.as_deref(),
                    &cache.cache_absolute_dir,
                );

                let handle = images.add(image);
                cache
                    .url_to_handle
                    .insert(result.url.clone(), handle.clone());

                for (target_entity, target) in &target_query {
                    if target.url == result.url {
                        commands
                            .entity(target_entity)
                            .insert(ImageNode::new(handle.clone()));
                    }
                }
            } else {
                warn!("decode image failed for {}", result.url);
            }
        } else {
            warn!("download image failed for {}", result.url);
        }

        commands.entity(task_entity).despawn();
    }
}

async fn download_image(url: &str) -> ImageDownloadResult {
    let mut request = ehttp::Request::get(url);
    request.timeout = Some(Duration::from_secs(15));

    let response = match ehttp::fetch_async(request).await {
        Ok(resp) => resp,
        Err(err) => {
            warn!("image download failed for {url}: {err}");
            return ImageDownloadResult {
                url: url.to_string(),
                bytes: None,
                content_type: None,
            };
        }
    };

    if !response.ok {
        warn!(
            "image response status failed for {url}: {} {}",
            response.status, response.status_text
        );
        return ImageDownloadResult {
            url: url.to_string(),
            bytes: None,
            content_type: None,
        };
    }

    let content_type = response.content_type().map(|value| value.to_string());
    let bytes = response.bytes;
    ImageDownloadResult {
        url: url.to_string(),
        bytes: Some(bytes),
        content_type,
    }
}

fn decode_image_from_response(
    url: &str,
    bytes: &[u8],
    content_type: Option<&str>,
) -> Option<Image> {
    let image_type = if let Some(content_type) = content_type {
        ImageType::MimeType(content_type)
    } else if let Some(ext) = extension_from_url(url) {
        ImageType::Extension(ext)
    } else {
        ImageType::Extension("png")
    };

    match Image::from_buffer(
        bytes,
        image_type,
        CompressedImageFormats::all(),
        true,
        ImageSampler::default(),
        RenderAssetUsages::default(),
    ) {
        Ok(image) => Some(image),
        Err(err) => {
            warn!("decode image bytes failed for {url}: {err}");
            None
        }
    }
}

fn extension_from_url(url: &str) -> Option<&str> {
    let url_no_query = url.split('?').next().unwrap_or(url);
    std::path::Path::new(url_no_query)
        .extension()
        .and_then(|value| value.to_str())
}

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
fn load_cached_image(url: &str, cache: &RemoteImageCache) -> Option<Image> {
    let path = resolve_cached_file_path(url, &cache.cache_absolute_dir)?;
    let bytes = fs::read(&path).ok()?;

    let ext = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("png");
    let image_type = ImageType::Extension(ext);

    Image::from_buffer(
        &bytes,
        image_type,
        CompressedImageFormats::all(),
        true,
        ImageSampler::default(),
        RenderAssetUsages::default(),
    )
    .ok()
}

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
fn persist_downloaded_image(
    url: &str,
    bytes: &[u8],
    content_type: Option<&str>,
    cache_absolute_dir: &Path,
) {
    let hash = sha256_base64url(url);
    let ext = infer_extension(url, content_type);
    let file_name = format!("{hash}.{ext}");
    let absolute_path = cache_absolute_dir.join(file_name);

    if let Err(err) = fs::create_dir_all(cache_absolute_dir) {
        warn!("create cache dir failed for {url}: {err}");
        return;
    }

    if let Err(err) = fs::write(absolute_path, bytes) {
        warn!("write cache file failed for {url}: {err}");
    }
}

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
fn resolve_cached_file_path(url: &str, cache_absolute_dir: &Path) -> Option<PathBuf> {
    let hash = sha256_base64url(url);
    let entries = fs::read_dir(cache_absolute_dir).ok()?;

    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name.starts_with(&hash) {
            return Some(entry.path());
        }
    }

    None
}

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
fn sha256_base64url(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let bytes = hasher.finalize();
    URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
fn infer_extension(url: &str, content_type: Option<&str>) -> String {
    let url_no_query = url.split('?').next().unwrap_or(url);
    if let Some(ext) = Path::new(url_no_query).extension().and_then(|s| s.to_str()) {
        let lower = ext.to_ascii_lowercase();
        if matches!(
            lower.as_str(),
            "png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp"
        ) {
            return if lower == "jpg" {
                "jpeg".to_string()
            } else {
                lower
            };
        }
    }

    let Some(content_type) = content_type else {
        return "png".to_string();
    };

    if content_type.contains("jpeg") || content_type.contains("jpg") {
        "jpeg".to_string()
    } else if content_type.contains("webp") {
        "webp".to_string()
    } else if content_type.contains("gif") {
        "gif".to_string()
    } else if content_type.contains("bmp") {
        "bmp".to_string()
    } else {
        "png".to_string()
    }
}
