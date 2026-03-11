use bevy::prelude::*;
use bevy_theme::prelude::*;
use bevy_toast::ToastPlugin;

use crate::{resource::LoadingPlugin, view::ViewPlugin};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Asset,
    // During this State the actual game logic is executed
    InitConfig,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(BevyThemePlugin::new(ThemePreset::default()))
            .add_plugins((LoadingPlugin, ToastPlugin, ViewPlugin));

        // #[cfg(debug_assertions)]
        // {
        //     use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

        //     app.add_plugins((
        //         FrameTimeDiagnosticsPlugin::default(),
        //         LogDiagnosticsPlugin::default(),
        //     ));
        // }
    }
}
