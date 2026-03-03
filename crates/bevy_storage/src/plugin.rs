use bevy_app::prelude::*;
use crate::storage::StorageManager;

/// 存储插件
pub struct StoragePlugin {
    pub organization: String,
    pub application: String,
}

impl Default for StoragePlugin {
    fn default() -> Self {
        Self {
            organization: "bevy_storage".to_string(),
            application: "dev".to_string(),
        }
    }
}

impl StoragePlugin {
    pub fn new(organization: String, application: String) -> Self {
        Self { organization, application }
    }
}

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        let storage = StorageManager::new(
            &self.organization,
            &self.application,
        );

        app.insert_resource(storage);
    }
}
