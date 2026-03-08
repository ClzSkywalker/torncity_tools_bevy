use bevy_color::prelude::*;
use bevy_feathers::{theme::ThemeProps, tokens};
use bevy_platform::collections::HashMap;
use bevy_state::prelude::*;

fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
    let a = if hex.len() >= 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f32 / 255.0
    } else {
        1.0
    };
    Color::srgba(r, g, b, a)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum ThemeMode {
    #[default]
    Preset,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum ThemePreset {
    #[default]
    MaterialDesign3,
    GitHubDark,
}

#[derive(Clone, Debug)]
pub struct ThemeColors {
    pub primary: Color,
    pub primary_hover: Color,
    pub primary_active: Color,
    pub secondary: Color,

    pub bg_deep: Color,
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_tertiary: Color,
    pub bg_elevated: Color,

    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,

    pub border_subtle: Color,
    pub border_default: Color,
    pub border_active: Color,

    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    pub scrollbar_thumb: Color,
    pub scrollbar_track: Color,
    pub overlay: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::github_dark()
    }
}

impl ThemeColors {
    pub fn material_design3() -> Self {
        Self {
            primary: hex_to_color("#00D084"),
            primary_hover: hex_to_color("#00E896"),
            primary_active: hex_to_color("#00AB6B"),
            secondary: hex_to_color("#2ED4B9"),

            bg_deep: hex_to_color("#121212"),
            bg_primary: hex_to_color("#1E1E1E"),
            bg_secondary: hex_to_color("#2D2D2D"),
            bg_tertiary: hex_to_color("#3C3C3C"),
            bg_elevated: hex_to_color("#4A4A4A"),

            text_primary: Color::WHITE,
            text_secondary: hex_to_color("#A1A1A6"),
            text_muted: hex_to_color("#71717A"),

            border_subtle: hex_to_color("#3F3F46"),
            border_default: hex_to_color("#52525B"),
            border_active: hex_to_color("#00D084"),

            success: hex_to_color("#22C55E"),
            warning: hex_to_color("#F59E0B"),
            error: hex_to_color("#EF4444"),
            info: hex_to_color("#3B82F6"),

            scrollbar_thumb: hex_to_color("#FFFFFF26"),
            scrollbar_track: hex_to_color("#FFFFFF0D"),
            overlay: hex_to_color("#00000099"),
        }
    }

    pub fn github_dark() -> Self {
        Self {
            primary: hex_to_color("#38BDF8"),
            primary_hover: hex_to_color("#73D9FF"),
            primary_active: hex_to_color("#2680C2"),
            secondary: hex_to_color("#8CB2E6"),

            bg_deep: hex_to_color("#0D1117"),
            bg_primary: hex_to_color("#161B22"),
            bg_secondary: hex_to_color("#21262D"),
            bg_tertiary: hex_to_color("#30363D"),
            bg_elevated: hex_to_color("#484F58"),

            text_primary: hex_to_color("#F8FAFC"),
            text_secondary: hex_to_color("#9CA3AF"),
            text_muted: hex_to_color("#737373"),

            border_subtle: hex_to_color("#30363D"),
            border_default: hex_to_color("#484F58"),
            border_active: hex_to_color("#38BDF8"),

            success: hex_to_color("#29A355"),
            warning: hex_to_color("#D29922"),
            error: hex_to_color("#F85149"),
            info: hex_to_color("#58A6FF"),

            scrollbar_thumb: hex_to_color("#FFFFFF26"),
            scrollbar_track: hex_to_color("#FFFFFF0D"),
            overlay: hex_to_color("#00000099"),
        }
    }

    pub fn light_default() -> Self {
        Self {
            primary: hex_to_color("#00BF7A"),
            primary_hover: hex_to_color("#00CC8C"),
            primary_active: hex_to_color("#009E66"),
            secondary: hex_to_color("#26C7B3"),

            bg_deep: hex_to_color("#F2F2F5"),
            bg_primary: hex_to_color("#FAFAFA"),
            bg_secondary: hex_to_color("#FFFFFF"),
            bg_tertiary: hex_to_color("#F2F2F7"),
            bg_elevated: hex_to_color("#EBEBEE"),

            text_primary: hex_to_color("#1A1A1F"),
            text_secondary: hex_to_color("#666670"),
            text_muted: hex_to_color("#9999A6"),

            border_subtle: hex_to_color("#E0E0E6"),
            border_default: hex_to_color("#CCCCD1"),
            border_active: hex_to_color("#00BF7A"),

            success: hex_to_color("#21B35A"),
            warning: hex_to_color("#EB8C0C"),
            error: hex_to_color("#E63B3B"),
            info: hex_to_color("#3370E6"),

            scrollbar_thumb: hex_to_color("#0000004D"),
            scrollbar_track: hex_to_color("#0000001A"),
            overlay: hex_to_color("#00000066"),
        }
    }

    pub fn to_feathers_props(&self) -> ThemeProps {
        ThemeProps {
            color: HashMap::from([
                (tokens::WINDOW_BG, self.bg_primary),
                (tokens::TEXT_MAIN, self.text_primary),
                (tokens::TEXT_DIM, self.text_secondary),
                (tokens::BUTTON_BG, self.bg_tertiary),
                (tokens::BUTTON_BG_HOVER, self.bg_elevated),
                (tokens::BUTTON_BG_PRESSED, self.bg_secondary),
                (tokens::BUTTON_BG_DISABLED, self.bg_deep),
                (tokens::BUTTON_PRIMARY_BG, self.primary),
                (tokens::BUTTON_PRIMARY_BG_HOVER, self.primary_hover),
                (tokens::BUTTON_PRIMARY_BG_PRESSED, self.primary_active),
                (tokens::BUTTON_PRIMARY_BG_DISABLED, self.bg_deep),
                (tokens::BUTTON_TEXT, self.text_primary),
                (tokens::BUTTON_TEXT_DISABLED, self.text_muted),
                (tokens::BUTTON_PRIMARY_TEXT, self.text_primary),
                (tokens::BUTTON_PRIMARY_TEXT_DISABLED, self.text_muted),
                (tokens::SLIDER_BG, self.bg_secondary),
                (tokens::SLIDER_BAR, self.primary),
                (tokens::SLIDER_BAR_DISABLED, self.border_default),
                (tokens::SLIDER_TEXT, self.text_primary),
                (tokens::SLIDER_TEXT_DISABLED, self.text_muted),
                (tokens::CHECKBOX_BG, self.bg_tertiary),
                (tokens::CHECKBOX_BG_CHECKED, self.primary),
                (tokens::CHECKBOX_BG_DISABLED, self.bg_secondary),
                (tokens::CHECKBOX_BG_CHECKED_DISABLED, self.bg_tertiary),
                (tokens::CHECKBOX_BORDER, self.border_default),
                (tokens::CHECKBOX_BORDER_HOVER, self.border_active),
                (tokens::CHECKBOX_BORDER_DISABLED, self.border_subtle),
                (tokens::CHECKBOX_MARK, self.text_primary),
                (tokens::CHECKBOX_MARK_DISABLED, self.text_muted),
                (tokens::CHECKBOX_TEXT, self.text_primary),
                (tokens::CHECKBOX_TEXT_DISABLED, self.text_muted),
                (tokens::SWITCH_BG, self.bg_tertiary),
                (tokens::SWITCH_BG_DISABLED, self.bg_secondary),
                (tokens::SWITCH_BG_CHECKED, self.primary),
                (tokens::SWITCH_BG_CHECKED_DISABLED, self.bg_tertiary),
                (tokens::SWITCH_BORDER, self.border_default),
                (tokens::SWITCH_BORDER_HOVER, self.border_active),
                (tokens::SWITCH_BORDER_DISABLED, self.border_subtle),
                (tokens::SWITCH_SLIDE, self.text_primary),
                (tokens::SWITCH_SLIDE_DISABLED, self.text_muted),
            ]),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomTheme {
    pub name: String,
    pub colors: ThemeColors,
    pub is_dark: bool,
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            name: "Custom".to_string(),
            colors: ThemeColors::github_dark(),
            is_dark: true,
        }
    }
}

impl CustomTheme {
    pub fn builder() -> CustomThemeBuilder {
        CustomThemeBuilder::default()
    }
}

#[derive(Default)]
pub struct CustomThemeBuilder {
    name: String,
    is_dark: bool,
    primary: Option<Color>,
    bg_primary: Option<Color>,
    secondary: Option<Color>,
}

impl CustomThemeBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn primary(mut self, color: Color) -> Self {
        self.primary = Some(color);
        self
    }

    pub fn secondary(mut self, color: Color) -> Self {
        self.secondary = Some(color);
        self
    }

    pub fn bg_primary(mut self, color: Color) -> Self {
        self.bg_primary = Some(color);
        self
    }

    pub fn dark_mode(mut self) -> Self {
        self.is_dark = true;
        self
    }

    pub fn light_mode(mut self) -> Self {
        self.is_dark = false;
        self
    }

    pub fn build(self) -> CustomTheme {
        let mut colors = if self.is_dark {
            ThemeColors::github_dark()
        } else {
            ThemeColors::light_default()
        };

        if let Some(primary) = self.primary {
            colors.primary = primary;
            colors.primary_hover = primary;
            colors.primary_active = primary;
            colors.border_active = primary;
        }

        if let Some(secondary) = self.secondary {
            colors.secondary = secondary;
        }

        if let Some(bg) = self.bg_primary {
            colors.bg_primary = bg;
        }

        CustomTheme {
            name: self.name,
            colors,
            is_dark: self.is_dark,
        }
    }
}
