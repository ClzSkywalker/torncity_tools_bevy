use std::{
    collections::{HashMap, HashSet},
    path::Path,
    time::Duration,
};

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
use bevy_storage::StorageManager;

use bevy_app::prelude::*;
use bevy_asset::{RenderAssetUsages, prelude::*};
use bevy_ecs::prelude::*;
use bevy_image::{CompressedImageFormats, ImageSampler, ImageType, prelude::*};
use bevy_log::prelude::*;
use bevy_tasks::{IoTaskPool, Task, futures_lite::future};
use bevy_ui::prelude::*;

const REMOTE_IMAGE_CACHE_NAMESPACE: &str = "image";

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

#[derive(Resource, Debug, Default)]
struct RemoteImageCache {
    in_flight: HashSet<String>,
    url_to_handle: HashMap<String, Handle<Image>>,
}

#[derive(Debug, Default)]
struct ImageDownloadResult {
    url: String,
    bytes: Option<Vec<u8>>,
    content_type: Option<String>,
}

fn queue_remote_images(
    mut commands: Commands,
    #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))] mut images: ResMut<
        Assets<Image>,
    >,
    #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))] storage: Res<StorageManager>,
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
        if let Some(image) = load_cached_image(&target.url, &storage) {
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
    #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))] storage: Res<StorageManager>,
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
                let _ = storage.save_cache_bytes(
                    REMOTE_IMAGE_CACHE_NAMESPACE,
                    &result.url,
                    &bytes,
                    extension_from_url(&result.url),
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
    request.timeout = Some(Duration::from_secs(5));

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
    Path::new(url_no_query)
        .extension()
        .and_then(|value| value.to_str())
}

#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
fn load_cached_image(url: &str, storage: &StorageManager) -> Option<Image> {
    let cached = storage
        .load_cache_bytes(REMOTE_IMAGE_CACHE_NAMESPACE, url)
        .ok()??;
    decode_image_from_response(url, &cached.bytes, cached.content_type.as_deref()).or_else(|| {
        let ext = cached.ext.as_deref()?;
        Image::from_buffer(
            &cached.bytes,
            ImageType::Extension(ext),
            CompressedImageFormats::all(),
            true,
            ImageSampler::default(),
            RenderAssetUsages::default(),
        )
        .ok()
    })
}
