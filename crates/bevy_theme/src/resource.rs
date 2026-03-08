use bevy_ecs::prelude::*;
use bevy_feathers::theme::ThemeProps;
use super::theme::{ThemeColors, ThemeMode, ThemePreset, CustomTheme};

#[derive(Resource, Clone, Debug)]
pub struct Theme {
    pub mode: ThemeMode,
    pub preset: ThemePreset,
    pub custom: CustomTheme,
    pub current_colors: ThemeColors,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Preset,
            preset: ThemePreset::GitHubDark,
            custom: CustomTheme::default(),
            current_colors: ThemeColors::github_dark(),
        }
    }
}

impl Theme {
    pub fn from_preset(preset: ThemePreset) -> Self {
        let colors = match preset {
            ThemePreset::MaterialDesign3 => ThemeColors::material_design3(),
            ThemePreset::GitHubDark => ThemeColors::github_dark(),
        };
        Self {
            mode: ThemeMode::Preset,
            preset,
            custom: CustomTheme::default(),
            current_colors: colors,
        }
    }

    pub fn colors(&self) -> &ThemeColors {
        &self.current_colors
    }

    pub fn colors_mut(&mut self) -> &mut ThemeColors {
        &mut self.current_colors
    }

    pub fn set_preset(&mut self, preset: ThemePreset) {
        self.mode = ThemeMode::Preset;
        self.preset = preset;
        self.current_colors = match preset {
            ThemePreset::MaterialDesign3 => ThemeColors::material_design3(),
            ThemePreset::GitHubDark => ThemeColors::github_dark(),
        };
    }

    pub fn set_custom(&mut self, custom: CustomTheme) {
        let colors = custom.colors.clone();
        self.mode = ThemeMode::Custom;
        self.current_colors = colors;
        self.custom = custom;
    }

    pub fn update_custom_color(&mut self, update: impl FnOnce(&mut ThemeColors)) {
        if self.mode == ThemeMode::Preset {
            self.custom.colors = self.current_colors.clone();
            self.mode = ThemeMode::Custom;
        }
        update(&mut self.custom.colors);
        self.current_colors = self.custom.colors.clone();
    }

    pub fn is_dark_mode(&self) -> bool {
        match self.mode {
            ThemeMode::Preset => true,
            ThemeMode::Custom => self.custom.is_dark,
        }
    }

    pub fn to_feathers_props(&self) -> ThemeProps {
        self.current_colors.to_feathers_props()
    }
}
