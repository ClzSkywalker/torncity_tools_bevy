use bevy::prelude::*;
use std::time::Duration;

use crate::channel::{ToastChannel, ToastPriority};
use crate::layout::ToastPosition;

#[derive(Component, Clone)]
pub struct Toast {
    pub id: uuid::Uuid,
    pub channel: ToastChannel,
    pub priority: ToastPriority,
    pub created_at: f64,
    pub duration: Duration,
    pub remaining: Duration,
    pub position: ToastPosition,
}

#[derive(Component)]
pub struct ToastChild;

#[derive(Component)]
pub struct ToastText(pub String);

#[derive(Component, Clone)]
pub struct ToastAction {
    pub action_type: ToastActionType,
    pub data: Option<serde_json::Value>,
}

#[derive(Component, Clone)]
pub enum ToastActionType {
    OpenScreen(String),
    ShowDialog(String),
    Custom(String),
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ToastState {
    Appearing,
    Staying,
    Disappearing,
}

impl Default for ToastState {
    fn default() -> Self {
        Self::Appearing
    }
}

#[derive(Bundle)]
pub struct ToastBundle {
    pub toast: Toast,
    pub background_color: BackgroundColor,
    pub node: Node,
}

impl ToastBundle {
    pub fn new(toast: Toast, bg_color: Color) -> Self {
        Self {
            toast,
            background_color: BackgroundColor(bg_color),
            node: Node {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
        }
    }
}
