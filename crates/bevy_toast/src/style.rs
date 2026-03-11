use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastKind {
    Success,
    Error,
    Warning,
    Info,
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Default for ToastPosition {
    fn default() -> Self {
        Self::BottomCenter
    }
}

impl ToastPosition {
    pub fn align(&self) -> (JustifyContent, AlignItems) {
        match self {
            ToastPosition::TopLeft => (JustifyContent::Start, AlignItems::Start),
            ToastPosition::TopCenter => (JustifyContent::Center, AlignItems::Start),
            ToastPosition::TopRight => (JustifyContent::End, AlignItems::Start),
            ToastPosition::CenterLeft => (JustifyContent::Start, AlignItems::Center),
            ToastPosition::Center => (JustifyContent::Center, AlignItems::Center),
            ToastPosition::CenterRight => (JustifyContent::End, AlignItems::Center),
            ToastPosition::BottomLeft => (JustifyContent::Start, AlignItems::End),
            ToastPosition::BottomCenter => (JustifyContent::Center, AlignItems::End),
            ToastPosition::BottomRight => (JustifyContent::End, AlignItems::End),
        }
    }
}

#[derive(Resource)]
pub struct ToastTheme {
    pub success_color: Color,
    pub error_color: Color,
    pub warning_color: Color,
    pub info_color: Color,
    pub background: Color,
    pub text_color: Color,
    pub corner_radius: Val,
    pub padding: UiRect,
    pub max_width: Val,
}

impl Default for ToastTheme {
    fn default() -> Self {
        Self {
            success_color: Color::srgba(0.2, 0.8, 0.2, 0.75),
            error_color: Color::srgba(0.9, 0.2, 0.2, 0.75),
            warning_color: Color::srgba(0.9, 0.8, 0.2, 0.75),
            info_color: Color::srgba(0.2, 0.5, 0.9, 0.75),
            background: Color::srgba(0.15, 0.15, 0.15, 0.75),
            text_color: Color::WHITE,
            corner_radius: Val::Px(8.0),
            padding: UiRect::all(Val::Px(12.0)),
            max_width: Val::Percent(80.0),
        }
    }
}

impl ToastTheme {
    pub fn kind_color(&self, kind: ToastKind) -> Color {
        match kind {
            ToastKind::Success => self.success_color,
            ToastKind::Error => self.error_color,
            ToastKind::Warning => self.warning_color,
            ToastKind::Info => self.info_color,
        }
    }
}
