use bevy_app::prelude::*;
use crate::storage::StorageManager;

/// 存储插件
pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        let storage = StorageManager::new();

        app.insert_resource(storage);
    }
}
