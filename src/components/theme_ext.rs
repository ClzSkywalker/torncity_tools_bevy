use bevy::prelude::*;
use bevy_theme::prelude::*;

#[derive(Clone)]
pub struct CardThemeConfig {
    pub card_width: f32,
    pub card_border_width: f32,
    pub section_border_width: f32,
    pub image_width: f32,
    pub image_height: f32,
    pub row_gap: f32,
    pub section_padding: f32,
    pub title_font_size: f32,
    pub value_font_size: f32,
    pub placeholder_image_path: String,
}

impl Default for CardThemeConfig {
    fn default() -> Self {
        Self {
            card_width: 300.0,
            card_border_width: 1.0,
            section_border_width: 1.0,
            image_width: 50.0,
            image_height: 30.0,
            row_gap: 6.0,
            section_padding: 6.0,
            title_font_size: 20.0,
            value_font_size: 14.0,
            placeholder_image_path: "icons/loading.png".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct CardTheme {
    pub config: CardThemeConfig,
    pub colors: CardColors,
}

#[derive(Clone, Default)]
pub struct CardColors {
    pub card_background: Color,
    pub card_border_color: Color,
    pub section_border_color: Color,
    pub text_color: Color,
    pub muted_text_color: Color,
}

impl CardTheme {
    pub fn from_theme(theme: &Theme) -> Self {
        let colors = theme.colors();
        Self {
            config: CardThemeConfig::default(),
            colors: CardColors {
                card_background: colors.bg_secondary,
                card_border_color: colors.border_active,
                section_border_color: colors.border_active,
                text_color: colors.text_primary,
                muted_text_color: colors.text_secondary,
            },
        }
    }

    pub fn default_dark() -> Self {
        Self {
            config: CardThemeConfig::default(),
            colors: CardColors {
                card_background: Color::srgb(0.17, 0.17, 0.17),
                card_border_color: Color::srgb(0.20, 0.65, 0.22),
                section_border_color: Color::srgb(0.20, 0.65, 0.22),
                text_color: Color::WHITE,
                muted_text_color: Color::srgb(0.80, 0.80, 0.80),
            },
        }
    }
}

impl Default for CardTheme {
    fn default() -> Self {
        Self::default_dark()
    }
}
