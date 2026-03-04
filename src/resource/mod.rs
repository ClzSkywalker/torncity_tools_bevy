use crate::game::GameState;
use crate::model::items::{CsvItemInfo, ItemInfo};
use bevy::asset::AssetLoader;
use bevy::asset::io::Reader;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use thiserror::Error;

pub mod state;
pub mod items_data;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemsCsvAsset>()
            .register_asset_loader(ItemsCsvAssetLoader)
            .add_loading_state(
                LoadingState::new(GameState::Asset)
                    .continue_to_state(GameState::InitConfig)
                    .load_collection::<DataAssets>()
                    .load_collection::<TextureAssets>()
                    .load_collection::<FontAssets>(),
            );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)
#[derive(AssetCollection, Resource)]
pub struct DataAssets {
    #[asset(path = "data/torncity_items.csv")]
    pub items: Handle<ItemsCsvAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "icons/loading.png")]
    pub loading: Handle<Image>,
}

#[derive(Asset, TypePath, Debug, Clone)]
pub struct ItemsCsvAsset {
    pub items: Vec<ItemInfo>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/NotoSansSC-Medium.ttf")]
    pub font: Handle<Font>,
}

#[derive(Default, TypePath)]
pub struct ItemsCsvAssetLoader;

#[derive(Debug, Error)]
pub enum ItemsCsvAssetLoaderError {
    #[error("Failed to read csv file bytes: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse csv row: {0}")]
    Csv(#[from] csv::Error),
}

impl AssetLoader for ItemsCsvAssetLoader {
    type Asset = ItemsCsvAsset;
    type Settings = ();
    type Error = ItemsCsvAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(bytes.as_slice());
        let mut items = Vec::new();

        for record in csv_reader.deserialize::<CsvItemInfo>() {
            items.push(record?.into());
        }

        Ok(ItemsCsvAsset { items })
    }

    fn extensions(&self) -> &[&str] {
        &["csv"]
    }
}
