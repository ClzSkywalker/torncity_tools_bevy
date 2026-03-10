use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_color::prelude::*;
use bevy_image::Image;
use bevy_ui::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastKind {
    Success,
    Error,
    Warning,
    Info,
    Custom(Color),
}

impl Default for ToastKind {
    fn default() -> Self {
        Self::Info
    }
}

impl ToastKind {
    pub fn color(&self) -> Color {
        match self {
            ToastKind::Success => Color::srgb(0.2, 0.8, 0.2),
            ToastKind::Error => Color::srgb(0.9, 0.2, 0.2),
            ToastKind::Warning => Color::srgb(0.9, 0.8, 0.2),
            ToastKind::Info => Color::srgb(0.2, 0.5, 0.9),
            ToastKind::Custom(c) => *c,
        }
    }
}

#[derive(Clone)]
pub enum ToastIcon {
    Success,
    Error,
    Warning,
    Info,
    Custom(Handle<Image>),
}

impl Default for ToastIcon {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Clone)]
pub enum ToastContent {
    Text(String),
    IconText { icon: ToastIcon, text: String },
    Custom(Entity),
}

impl Default for ToastContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ToastContent {
    pub fn text(&self) -> &str {
        match self {
            ToastContent::Text(s) => s.as_str(),
            ToastContent::IconText { text, .. } => text.as_str(),
            ToastContent::Custom(_) => "",
        }
    }
}

#[derive(Clone)]
pub struct ToastStyleOverride {
    pub background: Option<Color>,
    pub text_color: Option<Color>,
    pub corner_radius: Option<Val>,
    pub padding: Option<Val>,
    pub max_width: Option<Val>,
}

impl Default for ToastStyleOverride {
    fn default() -> Self {
        Self {
            background: None,
            text_color: None,
            corner_radius: None,
            padding: None,
            max_width: None,
        }
    }
}
