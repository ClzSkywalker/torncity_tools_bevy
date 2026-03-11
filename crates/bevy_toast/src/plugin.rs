use std::time::Duration;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::relationship::RelatedSpawner;
use bevy_text::prelude::*;
use bevy_time::Time;
use bevy_ui::prelude::*;
use bevy_ui::FocusPolicy;
use bevy_picking::prelude::Pickable;

use crate::components::{Toast, ToastBundle, ToastRoot};
use crate::events::ToastEvent;
use crate::style::{ToastPosition, ToastTheme};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastRootArea {
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

impl ToastPosition {
    pub fn root_area(&self) -> ToastRootArea {
        match self {
            ToastPosition::TopLeft => ToastRootArea::TopLeft,
            ToastPosition::TopCenter => ToastRootArea::TopCenter,
            ToastPosition::TopRight => ToastRootArea::TopRight,
            ToastPosition::CenterLeft => ToastRootArea::CenterLeft,
            ToastPosition::Center => ToastRootArea::Center,
            ToastPosition::CenterRight => ToastRootArea::CenterRight,
            ToastPosition::BottomLeft => ToastRootArea::BottomLeft,
            ToastPosition::BottomCenter => ToastRootArea::BottomCenter,
            ToastPosition::BottomRight => ToastRootArea::BottomRight,
        }
    }
}

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToastTheme>()
            .add_observer(toast_spawn_observer)
            .add_systems(Startup, setup_toast_roots)
            .add_systems(Update, toast_lifetime_system);
    }
}

fn setup_toast_roots(mut commands: Commands) {
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            width: percent(100.0),
            height: percent(100.0),
            ..Default::default()
        },
        ZIndex(1000),
        FocusPolicy::Pass,
        Pickable::IGNORE,
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                Pickable::IGNORE,
                Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
                    spawner.spawn(node_tpl(
                        ToastRoot(ToastRootArea::TopLeft),
                        JustifyContent::Start,
                        AlignItems::Start,
                    ));
                    spawner.spawn(node_tpl(
                        ToastRoot(ToastRootArea::TopCenter),
                        JustifyContent::Start,
                        AlignItems::Center,
                    ));
                    spawner.spawn(node_tpl(
                        ToastRoot(ToastRootArea::TopRight),
                        JustifyContent::Start,
                        AlignItems::End,
                    ));
                })),
            ));

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                Pickable::IGNORE,
                Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
                    spawner.spawn(node_tpl(
                        ToastRoot(ToastRootArea::CenterLeft),
                        JustifyContent::Center,
                        AlignItems::Start,
                    ));
                    spawner.spawn(node_tpl(
                        ToastRoot(ToastRootArea::Center),
                        JustifyContent::Center,
                        AlignItems::Center,
                    ));
                    spawner.spawn(node_tpl(
                        ToastRoot(ToastRootArea::CenterRight),
                        JustifyContent::Center,
                        AlignItems::End,
                    ));
                })),
            ));

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                Pickable::IGNORE,
                Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
                    spawner.spawn(node_tpl(
                        ToastRoot(ToastRootArea::BottomLeft),
                        JustifyContent::End,
                        AlignItems::Start,
                    ));
                    spawner.spawn(node_tpl(
                        ToastRoot(ToastRootArea::BottomCenter),
                        JustifyContent::End,
                        AlignItems::Center,
                    ));
                    spawner.spawn(node_tpl(
                        ToastRoot(ToastRootArea::BottomRight),
                        JustifyContent::End,
                        AlignItems::End,
                    ));
                })),
            ));
        })),
    ));
}

fn node_tpl(
    marker: ToastRoot,
    justify_content: JustifyContent,
    align_items: AlignItems,
) -> impl Bundle {
    (
        marker,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            flex_grow: 1.,
            justify_content,
            align_items,
            padding: UiRect::new(
                Val::Px(5.0),
                Val::Px(5.0),
                Val::Px(5.0),
                Val::Px(5.0),
            ),
            ..Default::default()
        },
        Pickable::IGNORE,
    )
}

fn toast_spawn_observer(
    event: On<ToastEvent>,
    mut commands: Commands,
    roots: Query<(Entity, &ToastRoot)>,
    theme: Res<ToastTheme>,
) {
    let toast_event = event.event();
    let target_area = toast_event.position.root_area();

    for (entity, root) in roots.iter() {
        if root.0 != target_area {
            continue;
        }
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                ToastBundle::new(toast_event.kind, &theme),
                children![(
                    Text(toast_event.text.clone()),
                    TextFont {
                        font_size: 16.0,
                        ..Default::default()
                    },
                    TextColor(theme.text_color),
                )],
            ));
        });
        break;
    }
}

fn toast_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut toasts: Query<(Entity, &mut Toast)>,
) {
    let delta = time.delta();
    let mut to_despawn = Vec::new();

    for (entity, mut toast) in toasts.iter_mut() {
        toast.remaining = toast.remaining.saturating_sub(delta);

        if toast.remaining <= Duration::ZERO {
            to_despawn.push(entity);
        }
    }

    for entity in to_despawn {
        commands.entity(entity).despawn();
    }
}
