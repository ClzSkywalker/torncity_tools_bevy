use bevy::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::channel::ToastChannels;
use crate::components::{Toast, ToastBundle, ToastState};
use crate::events::{ToastDismissEvent, ToastEvent};
use crate::queue::ToastQueue;
use crate::resource::{ToastConfig, ToastTheme};

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToastConfig>()
            .init_resource::<ToastTheme>()
            .init_resource::<ToastQueue>()
            .init_resource::<ToastChannels>()
            .add_observer(receive_toast_event)
            .add_observer(receive_toast_dismiss_event)
            .add_systems(Update, toast_lifetime_system)
            .add_systems(Update, toast_spawn_system);
    }
}

fn receive_toast_dismiss_event(_event: On<ToastDismissEvent>, mut queue: ResMut<ToastQueue>) {
    queue.mark_shown();
}

fn receive_toast_event(event: On<ToastEvent>, mut queue: ResMut<ToastQueue>) {
    queue.push(event.event().clone());
}

fn toast_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut toasts: Query<(Entity, &mut Toast, &mut ToastState)>,
) {
    let delta = time.delta();
    let mut to_despawn = Vec::new();

    for (entity, mut toast, mut state) in toasts.iter_mut() {
        toast.remaining = toast.remaining.saturating_sub(delta);

        match *state {
            ToastState::Appearing => {
                if toast.remaining < toast.duration - Duration::from_millis(200) {
                    *state = ToastState::Staying;
                }
            }
            ToastState::Staying => {
                if toast.remaining <= Duration::ZERO {
                    *state = ToastState::Disappearing;
                }
            }
            ToastState::Disappearing => {
                if toast.remaining < Duration::from_millis(150) {
                    to_despawn.push(entity);
                }
            }
        }
    }

    for entity in to_despawn {
        commands.entity(entity).despawn();
    }
}

fn toast_spawn_system(
    mut commands: Commands,
    mut queue: ResMut<ToastQueue>,
    theme: Res<ToastTheme>,
) {
    if let Some(event) = queue.pop() {
        let position = event.position;
        let anchor = position.anchor();

        let bg_color = match event.kind {
            crate::style::ToastKind::Success => theme.success_color,
            crate::style::ToastKind::Error => theme.error_color,
            crate::style::ToastKind::Warning => theme.warning_color,
            crate::style::ToastKind::Info => theme.info_color,
            crate::style::ToastKind::Custom(c) => c,
        };

        let text_content = match &event.content {
            crate::style::ToastContent::Text(s) => s.clone(),
            crate::style::ToastContent::IconText { text, .. } => text.clone(),
            crate::style::ToastContent::Custom(_) => String::new(),
        };

        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0);

        commands
            .spawn((ToastBundle::new(
                Toast {
                    id: uuid::Uuid::new_v4(),
                    channel: event.channel,
                    priority: event.priority,
                    created_at,
                    duration: event.duration,
                    remaining: event.duration,
                    position,
                },
                bg_color,
            ),))
            .with_children(|parent| {
                parent.spawn((
                    Text(text_content),
                    TextFont {
                        font_size: 16.0,
                        ..Default::default()
                    },
                    TextColor(theme.text_color),
                ));
            });
    }
}
