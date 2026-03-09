use bevy::prelude::*;
use std::time::Duration;

use crate::channel::{ToastChannel, ToastPriority};
use crate::layout::ToastPosition;
use crate::style::{ToastContent, ToastIcon, ToastKind, ToastStyleOverride};
use crate::components::ToastAction;

#[derive(Event, Clone)]
pub struct ToastEvent {
    pub content: ToastContent,
    pub kind: ToastKind,
    pub position: ToastPosition,
    pub channel: ToastChannel,
    pub priority: ToastPriority,
    pub duration: Duration,
    pub style: Option<ToastStyleOverride>,
    pub action: Option<ToastAction>,
    pub tap_to_dismiss: bool,
}

impl Default for ToastEvent {
    fn default() -> Self {
        Self {
            content: ToastContent::Text(String::new()),
            kind: ToastKind::Info,
            position: ToastPosition::BottomCenter,
            channel: ToastChannel::System,
            priority: ToastPriority::Normal,
            duration: Duration::from_secs(2),
            style: None,
            action: None,
            tap_to_dismiss: false,
        }
    }
}

impl ToastEvent {
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: ToastContent::Text(content.into()),
            ..Default::default()
        }
    }

    pub fn success(text: impl Into<String>) -> Self {
        Self {
            content: ToastContent::IconText {
                icon: ToastIcon::Success,
                text: text.into(),
            },
            kind: ToastKind::Success,
            ..Default::default()
        }
    }

    pub fn error(text: impl Into<String>) -> Self {
        Self {
            content: ToastContent::IconText {
                icon: ToastIcon::Error,
                text: text.into(),
            },
            kind: ToastKind::Error,
            ..Default::default()
        }
    }

    pub fn warning(text: impl Into<String>) -> Self {
        Self {
            content: ToastContent::IconText {
                icon: ToastIcon::Warning,
                text: text.into(),
            },
            kind: ToastKind::Warning,
            ..Default::default()
        }
    }

    pub fn info(text: impl Into<String>) -> Self {
        Self {
            content: ToastContent::IconText {
                icon: ToastIcon::Info,
                text: text.into(),
            },
            kind: ToastKind::Info,
            ..Default::default()
        }
    }

    pub fn custom(content: ToastContent) -> Self {
        Self {
            content,
            ..Default::default()
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

    pub fn with_channel(mut self, channel: ToastChannel) -> Self {
        self.channel = channel;
        self
    }

    pub fn with_priority(mut self, priority: ToastPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_style(mut self, style: ToastStyleOverride) -> Self {
        self.style = Some(style);
        self
    }

    pub fn with_on_tap(mut self, action: ToastAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_tap_to_dismiss(mut self, enabled: bool) -> Self {
        self.tap_to_dismiss = enabled;
        self
    }
}

#[derive(Event, Clone)]
pub struct ToastDismissEvent {
    pub id: uuid::Uuid,
    pub reason: DismissReason,
}

#[derive(Event, Clone)]
pub enum DismissReason {
    Timeout,
    UserTap,
    Force,
}

#[derive(Event, Clone)]
pub struct ToastActionEvent {
    pub id: uuid::Uuid,
    pub action: ToastAction,
}
