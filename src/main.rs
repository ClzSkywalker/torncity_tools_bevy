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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TornCity Tools".to_string(),
                resolution: (480, 300).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
