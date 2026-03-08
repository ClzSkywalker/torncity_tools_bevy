pub mod component;
pub mod resource;
pub mod systems;
pub mod theme;

pub mod prelude;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_feathers::theme::UiTheme;

#[derive(Default)]
pub struct BevyThemePlugin {
    pub initial_theme: theme::ThemePreset,
}

impl BevyThemePlugin {
    pub fn new(preset: theme::ThemePreset) -> Self {
        Self {
            initial_theme: preset,
        }
    }
}

impl Plugin for BevyThemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>()
            .init_resource::<resource::Theme>()
            .insert_resource(resource::Theme::from_preset(self.initial_theme))
            .add_observer(systems::on_insert_background)
            .add_observer(systems::on_insert_border)
            .add_observer(systems::on_insert_text)
            .add_observer(systems::on_insert_state)
            .add_observer(systems::on_insert_primary_button)
            .add_systems(
                Update,
                (
                    systems::apply_theme_to_components,
                    sync_theme_to_feathers,
                )
                    .run_if(resource_changed::<resource::Theme>),
            );
    }
}

fn sync_theme_to_feathers(theme: Res<resource::Theme>, mut ui_theme: ResMut<UiTheme>) {
    ui_theme.0 = theme.to_feathers_props();
}

pub trait ThemeAppExt {
    fn set_theme(&mut self, preset: theme::ThemePreset);
    fn get_theme(&self) -> &resource::Theme;
    fn get_theme_mut(&mut self) -> Mut<'_, resource::Theme>;
}

impl ThemeAppExt for App {
    fn set_theme(&mut self, preset: theme::ThemePreset) {
        self.world_mut()
            .resource_mut::<resource::Theme>()
            .set_preset(preset);
    }

    fn get_theme(&self) -> &resource::Theme {
        self.world().resource::<resource::Theme>()
    }

    fn get_theme_mut(&mut self) -> Mut<'_, resource::Theme> {
        self.world_mut().resource_mut::<resource::Theme>()
    }
}

pub use component::{
    ThemedBackground, ThemedBackgroundLayer, ThemedBorder, ThemedBorderLayer, ThemedPrimaryButton,
    ThemedSecondaryButton, ThemedState, ThemedStateType, ThemedText, ThemedTextLayer,
};
pub use resource::Theme;
pub use systems::get_theme_colors;
pub use theme::ThemePreset;
pub use theme::{CustomTheme, CustomThemeBuilder, ThemeColors, ThemeMode};
