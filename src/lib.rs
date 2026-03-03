use bevy::prelude::*;

use crate::game::GamePlugin;

mod resource;
mod view;
mod game;
mod model;
mod components;
mod weav3r;
mod tools;
mod http;

#[cfg_attr(target_os = "android", bevy_main)]
pub fn main() {
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "TornCity Tools".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    });

    #[cfg(target_os = "android")]
    let default_plugins = default_plugins.set(bevy::render::RenderPlugin {
        // Some Mali drivers crash on Vulkan command encoding in wgpu.
        // Force GLES backend as a pragmatic mobile fallback.
        render_creation: bevy::render::settings::WgpuSettings {
            backends: Some(bevy::render::settings::Backends::GL),
            priority: bevy::render::settings::WgpuSettingsPriority::Compatibility,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });

    App::new()
            .add_plugins(default_plugins)
            .add_plugins(GamePlugin)
        .run();
}