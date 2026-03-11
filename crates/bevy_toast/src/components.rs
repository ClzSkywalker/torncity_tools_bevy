use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;
use std::time::Duration;

use crate::style::ToastKind;

#[derive(Component)]
pub struct Toast {
    pub id: uuid::Uuid,
    pub duration: Duration,
    pub remaining: Duration,
}

impl Toast {
    pub fn new(duration: Duration) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            duration,
            remaining: duration,
        }
    }
}

#[derive(Component)]
pub struct ToastRoot(pub crate::plugin::ToastRootArea);

#[derive(Bundle)]
pub struct ToastBundle {
    pub toast: Toast,
    pub background_color: BackgroundColor,
    pub node: Node,
}

impl ToastBundle {
    pub fn new(kind: ToastKind, theme: &crate::style::ToastTheme) -> Self {
        Self {
            toast: Toast::new(Duration::from_secs(2)),
            background_color: BackgroundColor(theme.kind_color(kind)),

            node: Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: theme.padding,
                border_radius: BorderRadius::all(theme.corner_radius),
                ..Default::default()
            },
        }
    }
}
