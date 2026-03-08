use super::theme::ThemeColors;
use bevy_color::prelude::Color;
use bevy_ecs::prelude::{Bundle, Component};
use bevy_text::prelude::TextColor;
use bevy_ui::prelude::*;

#[derive(Debug, Component, Clone)]
pub struct ThemedBackground {
    pub layer: ThemedBackgroundLayer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ThemedBackgroundLayer {
    Deep,
    Primary,
    Secondary,
    Tertiary,
    Elevated,
}

impl ThemedBackground {
    pub fn deep() -> Self {
        Self {
            layer: ThemedBackgroundLayer::Deep,
        }
    }
    pub fn primary() -> Self {
        Self {
            layer: ThemedBackgroundLayer::Primary,
        }
    }
    pub fn secondary() -> Self {
        Self {
            layer: ThemedBackgroundLayer::Secondary,
        }
    }
    pub fn tertiary() -> Self {
        Self {
            layer: ThemedBackgroundLayer::Tertiary,
        }
    }
    pub fn elevated() -> Self {
        Self {
            layer: ThemedBackgroundLayer::Elevated,
        }
    }

    pub fn get_color(&self, colors: &ThemeColors) -> Color {
        match self.layer {
            ThemedBackgroundLayer::Deep => colors.bg_deep,
            ThemedBackgroundLayer::Primary => colors.bg_primary,
            ThemedBackgroundLayer::Secondary => colors.bg_secondary,
            ThemedBackgroundLayer::Tertiary => colors.bg_tertiary,
            ThemedBackgroundLayer::Elevated => colors.bg_elevated,
        }
    }

    pub fn bundle(self) -> impl Bundle {
        (self, BackgroundColor(Color::BLACK))
    }
}

#[derive(Component, Clone, Default)]
pub struct ThemedBorder {
    pub layer: ThemedBorderLayer,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ThemedBorderLayer {
    #[default]
    Default,
    Subtle,
    Active,
}

impl ThemedBorder {
    pub fn subtle() -> Self {
        Self {
            layer: ThemedBorderLayer::Subtle,
        }
    }
    pub fn active() -> Self {
        Self {
            layer: ThemedBorderLayer::Active,
        }
    }

    pub fn get_color(&self, colors: &ThemeColors) -> Color {
        match self.layer {
            ThemedBorderLayer::Subtle => colors.border_subtle,
            ThemedBorderLayer::Default => colors.border_default,
            ThemedBorderLayer::Active => colors.border_active,
        }
    }

    pub fn bundle(self) -> impl Bundle {
        (self, BorderColor::all(Color::BLACK))
    }
}

#[derive(Component, Clone)]
pub struct ThemedText {
    pub layer: ThemedTextLayer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ThemedTextLayer {
    Primary,
    Secondary,
    Muted,
}

impl ThemedText {
    pub fn primary() -> Self {
        Self {
            layer: ThemedTextLayer::Primary,
        }
    }
    pub fn secondary() -> Self {
        Self {
            layer: ThemedTextLayer::Secondary,
        }
    }
    pub fn muted() -> Self {
        Self {
            layer: ThemedTextLayer::Muted,
        }
    }

    pub fn get_color(&self, colors: &ThemeColors) -> Color {
        match self.layer {
            ThemedTextLayer::Primary => colors.text_primary,
            ThemedTextLayer::Secondary => colors.text_secondary,
            ThemedTextLayer::Muted => colors.text_muted,
        }
    }

    pub fn bundle(self) -> impl Bundle {
        (self, TextColor(Color::BLACK))
    }
}

#[derive(Component, Clone, Default)]
pub struct ThemedState {
    pub state_type: ThemedStateType,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ThemedStateType {
    #[default]
    Success,
    Warning,
    Error,
    Info,
    Primary,
}

impl ThemedState {
    pub fn success() -> Self {
        Self {
            state_type: ThemedStateType::Success,
        }
    }

    pub fn warning() -> Self {
        Self {
            state_type: ThemedStateType::Warning,
        }
    }

    pub fn error() -> Self {
        Self {
            state_type: ThemedStateType::Error,
        }
    }

    pub fn info() -> Self {
        Self {
            state_type: ThemedStateType::Info,
        }
    }

    pub fn primary() -> Self {
        Self {
            state_type: ThemedStateType::Primary,
        }
    }

    pub fn get_color(&self, colors: &ThemeColors) -> Color {
        match self.state_type {
            ThemedStateType::Success => colors.success,
            ThemedStateType::Warning => colors.warning,
            ThemedStateType::Error => colors.error,
            ThemedStateType::Info => colors.info,
            ThemedStateType::Primary => colors.primary,
        }
    }
}

#[derive(Component, Clone)]
pub struct ThemedPrimaryButton;

#[derive(Component, Clone)]
pub struct ThemedSecondaryButton;
