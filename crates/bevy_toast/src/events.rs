use std::time::Duration;

use bevy_ecs::prelude::*;

use crate::style::{ToastKind, ToastPosition};

#[derive(Event, Clone)]
pub struct ToastEvent {
    pub text: String,
    pub kind: ToastKind,
    pub position: ToastPosition,
    pub duration: Duration,
}

impl Default for ToastEvent {
    fn default() -> Self {
        Self {
            text: String::new(),
            kind: ToastKind::Info,
            position: ToastPosition::BottomCenter,
            duration: Duration::from_secs(2),
        }
    }
}

impl ToastEvent {
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            text: content.into(),
            kind: ToastKind::Info,
            position: ToastPosition::BottomCenter,
            duration: Duration::from_secs(2),
        }
    }

    pub fn success(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: ToastKind::Success,
            position: ToastPosition::BottomCenter,
            duration: Duration::from_secs(2),
        }
    }

    pub fn error(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: ToastKind::Error,
            position: ToastPosition::BottomCenter,
            duration: Duration::from_secs(2),
        }
    }

    pub fn warning(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: ToastKind::Warning,
            position: ToastPosition::BottomCenter,
            duration: Duration::from_secs(2),
        }
    }

    pub fn info(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: ToastKind::Info,
            position: ToastPosition::BottomCenter,
            duration: Duration::from_secs(2),
        }
    }

    pub fn with_kind(mut self, kind: ToastKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}
