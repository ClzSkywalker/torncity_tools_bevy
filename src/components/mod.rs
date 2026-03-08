use bevy::prelude::*;

pub mod browser;
pub mod button_click_effect;
pub mod number_stepper;
pub mod scroll;
pub mod tick;
pub mod trader_card;
pub mod theme_ext;

use bevy_feathers::FeathersPlugins;
pub use number_stepper::handle_stepper_buttons;

use crate::components::{
    button_click_effect::ButtonClickEffectPlugin, scroll::ScrollXPlugin,
    trader_card::TraderCardPlugin,
};

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FeathersPlugins)
            .add_plugins(ButtonClickEffectPlugin)
            .add_plugins(ScrollXPlugin)
            .add_plugins(TraderCardPlugin)
            .add_systems(Update, handle_stepper_buttons);
    }
}
