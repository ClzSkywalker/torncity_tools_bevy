use bevy_app::prelude::*;
use bevy_asset::{io::embedded::GetAssetServer, prelude::*};
use bevy_ecs::prelude::*;
use bevy_text::prelude::*;
use bevy_ui::prelude::*;

#[derive(Resource, Clone)]
pub struct UiFonts(pub Option<Handle<Font>>);

#[derive(Debug, Clone, Default)]
pub struct GlobalUiFontPlugin {
    pub path: Option<String>,
}

impl GlobalUiFontPlugin {
    pub fn new(path: Option<String>) -> Self {
        Self { path }
    }
}

impl Plugin for GlobalUiFontPlugin {
    fn build(&self, app: &mut App) {
        let font_handle = if self.path.is_some() {
            Some(app.get_asset_server().load(self.path.as_ref().unwrap()))
        } else {
            None
        };
        app.insert_resource(UiFonts(font_handle))
            .add_systems(Update, text_change_add)
            .add_systems(Update, font_change_ob.run_if(resource_changed::<UiFonts>));
    }
}

fn text_change_add(
    mut commands: Commands,
    fonts: Res<UiFonts>,
    query: Query<(Entity, Option<&TextFont>), Or<(Added<Text>, Changed<Text>)>>,
) {
    let Some(font_handle) = fonts.0.clone() else {
        return;
    };
    for (entity, existing_text_font) in &query {
        let mut text_font = existing_text_font.cloned().unwrap_or_default();
        text_font.font = font_handle.clone();
        commands.entity(entity).insert(text_font);
    }
}

fn font_change_ob(
    fonts: Res<UiFonts>,
    mut query: Query<&mut TextFont, Or<(Added<Text>, Changed<Text>)>>,
) {
    let Some(font_handle) = fonts.0.clone() else {
        return;
    };
    for mut text_font in &mut query {
        text_font.font = font_handle.clone();
    }
}
