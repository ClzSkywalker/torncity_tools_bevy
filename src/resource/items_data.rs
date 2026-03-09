use crate::game::GameState;
use crate::model::items::ItemInfo;
use crate::resource::{DataAssets, ItemsCsvAsset};
use bevy::prelude::*;

/// ItemsData 插件
pub struct ItemsDataPlugin;

impl Plugin for ItemsDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InitConfig), office_item_startup);
    }
}

/// 存储所有物品数据的 Resource
#[derive(Resource, Debug)]
pub struct OfficeItemsDbRes {
    pub items: Vec<ItemInfo>,
}

impl OfficeItemsDbRes {
    /// 根据 ID 查找物品
    #[allow(dead_code)]
    pub fn get_by_id(&self, id: i32) -> Option<&ItemInfo> {
        self.items.iter().find(|item| item.id == id)
    }

    /// 根据名称查找物品
    #[allow(dead_code)]
    pub fn get_by_name(&self, name: &str) -> Option<&ItemInfo> {
        self.items.iter().find(|item| item.name == name)
    }
}

/// 在加载状态完成后，从 DataAssets 中构建 OfficeItemsDbRes
pub fn office_item_startup(
    mut commands: Commands,
    data_assets: Res<DataAssets>,
    items_csv_assets: Res<Assets<ItemsCsvAsset>>,
) {
    let Some(csv_asset) = items_csv_assets.get(&data_assets.items) else {
        bevy::log::error!("ItemsCsvAsset not loaded when entering menu");
        return;
    };

    let database = OfficeItemsDbRes {
        items: csv_asset.items.clone(),
    };
    bevy::log::info!(
        "office_item_startup: Loaded {} items from DataAssets",
        database.items.len()
    );
    commands.insert_resource(database);
}
