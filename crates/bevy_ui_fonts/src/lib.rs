use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_text::prelude::*;
use bevy_ui::prelude::*;

pub const DEFAULT_TEXT_FONT_PATH: &str = "fonts/NotoSansSC-Medium.ttf";

#[derive(Resource, Clone)]
pub struct UiFonts {
    pub main: Handle<Font>,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct GlobalUiFontPath(pub Option<String>);

#[derive(Debug, Clone)]
pub struct GlobalUiFontPlugin {
    pub main_font_path: Option<String>,
}

impl Default for GlobalUiFontPlugin {
    fn default() -> Self {
        Self {
            // Use Bevy built-in fallback font by default.
            // This avoids startup errors when external font assets are absent.
            main_font_path: None,
        }
    }
}

impl Plugin for GlobalUiFontPlugin {
    fn build(&self, app: &mut App) {
        if self.main_font_path.is_some() {
            app.insert_resource(GlobalUiFontPath(self.main_font_path.clone()))
                .add_systems(Startup, init_ui_fonts)
                .add_systems(Update, apply_global_font_to_new_text);
        }
    }
}

fn init_ui_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_path: Res<GlobalUiFontPath>,
) {
    let Some(path) = font_path.0.clone() else {
        return;
    };
    commands.insert_resource(UiFonts {
        main: asset_server.load(path),
    });
}

fn apply_global_font_to_new_text(
    mut commands: Commands,
    fonts: Res<UiFonts>,
    query: Query<(Entity, Option<&TextFont>), Added<Text>>,
) {
    for (entity, existing_text_font) in &query {
        let mut text_font = existing_text_font.cloned().unwrap_or_default();
        text_font.font = fonts.main.clone();
        commands.entity(entity).insert(text_font);
    }
}
