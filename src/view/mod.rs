use bevy::prelude::*;
use bevy_clipboard::ClipboardPlugin;
use bevy_storage::StoragePlugin;
use bevy_tab::{BevyTabPlugin, tab::*};
use bevy_ui_fonts::{DEFAULT_TEXT_FONT_PATH, GlobalUiFontPlugin};

use crate::{
    components::ComponentsPlugin, http::favorites::Weav3rFavoriteHttpPlugin, resource::items_data::ItemsDataPlugin, view::res::SettingConfigRes
};

mod home;
mod res;
mod setting;
pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ItemsDataPlugin)
            .add_plugins(ComponentsPlugin)
            .add_plugins(Weav3rFavoriteHttpPlugin)
            .add_plugins(BevyTabPlugin::new(tab_config()))
            .add_plugins(ClipboardPlugin)
            .add_plugins(StoragePlugin::new(
                "SMTH".to_string(),
                "torncity_tool".to_string(),
            ))
            .add_plugins(GlobalUiFontPlugin {
                main_font_path: Some(DEFAULT_TEXT_FONT_PATH.to_string()),
            })
            .add_plugins((setting::SettingPlugin, home::Weav3rHomePlugin))
            .insert_resource(SettingConfigRes::default())
            .add_systems(Startup, setup);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
enum TabId {
    #[default]
    Home,
    Setting,
}

impl TabId {
    pub fn name(&self) -> &str {
        match self {
            TabId::Home => "Home",
            TabId::Setting => "Setting",
        }
    }
}

#[cfg(target_os = "android")]
const TOP_SAFE_INSET_PX: f32 = 28.0;
#[cfg(not(target_os = "android"))]
const TOP_SAFE_INSET_PX: f32 = 0.0;

fn tab_config() -> ViewTabConfig {
    let tabs = vec![
        TabItemConfig::new(TabId::Home.name(), TabId::Home.name()),
        TabItemConfig::new(TabId::Setting.name(), TabId::Setting.name()),
    ];
    let mut style = TabStyleConfig::default();
    style.content_padding = UiRect::new(px(5.0), px(5.0), px(5.0 + TOP_SAFE_INSET_PX), px(5.0));

    ViewTabConfig {
        tabs,
        initial_tab: TabId::Setting.name().to_string(),
        style,
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
