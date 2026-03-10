use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    Center,
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
    pub fn anchor(&self) -> (Val, Val) {
        match self {
            ToastPosition::TopLeft => (Val::Percent(5.0), Val::Percent(5.0)),
            ToastPosition::TopCenter => (Val::Percent(50.0), Val::Percent(5.0)),
            ToastPosition::TopRight => (Val::Percent(95.0), Val::Percent(5.0)),
            ToastPosition::Center => (Val::Percent(50.0), Val::Percent(50.0)),
            ToastPosition::BottomLeft => (Val::Percent(5.0), Val::Percent(95.0)),
            ToastPosition::BottomCenter => (Val::Percent(50.0), Val::Percent(95.0)),
            ToastPosition::BottomRight => (Val::Percent(95.0), Val::Percent(95.0)),
        }
    }

    pub fn pivot(&self) -> (Val, Val) {
        match self {
            ToastPosition::TopLeft => (Val::ZERO, Val::ZERO),
            ToastPosition::TopCenter => (Val::Percent(50.0), Val::ZERO),
            ToastPosition::TopRight => (Val::Percent(100.0), Val::ZERO),
            ToastPosition::Center => (Val::Percent(50.0), Val::Percent(50.0)),
            ToastPosition::BottomLeft => (Val::ZERO, Val::Percent(100.0)),
            ToastPosition::BottomCenter => (Val::Percent(50.0), Val::Percent(100.0)),
            ToastPosition::BottomRight => (Val::Percent(100.0), Val::Percent(100.0)),
        }
    }
}

#[derive(Component)]
pub struct ToastLayout {
    pub position: ToastPosition,
    pub index: usize,
}
