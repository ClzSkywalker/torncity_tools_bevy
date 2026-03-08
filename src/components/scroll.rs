use bevy::{
    ecs::relationship::RelatedSpawner,
    input::{
        mouse::{MouseScrollUnit, MouseWheel},
        touch::Touches,
    },
    picking::hover::HoverMap,
    prelude::*,
};
use bevy_ui_widgets::{ControlOrientation, CoreScrollbarThumb, Scrollbar};
use bevy_theme::prelude::*;

const SCROLL_LINE_SPEED: f32 = 36.0;

pub struct ScrollXPlugin;

impl Plugin for ScrollXPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_mouse_wheel_scroll, handle_touch_scroll));
    }
}

#[derive(Debug, Clone, Default)]
pub struct ScrollSpawn {
    pub width: Val,
    pub height: Val,
    pub column_gap: Val,
    pub row_gap: Val,
    pub background: Option<ThemedBackground>,
}

impl ScrollSpawn {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    pub fn with_column_gap(mut self, column_gap: Val) -> Self {
        self.column_gap = column_gap;
        self
    }

    pub fn with_row_gap(mut self, row_gap: Val) -> Self {
        self.row_gap = row_gap;
        self
    }

    pub fn bundle(self, data: Vec<impl Bundle>) -> impl Bundle {
        self.bundle_with_marker(data, None::<NoMarker>)
    }

    pub fn bundle_with_marker(
        self,
        data: Vec<impl Bundle>,
        marker: Option<impl Component>,
    ) -> impl Bundle {
        let column_gap = self.column_gap;
        let row_gap = self.row_gap;

        (
            Node {
                width: self.width,
                height: self.height,
                flex_grow: 1.0,
                min_height: Val::Px(0.0),
                padding: UiRect::all(Val::Px(2.0)),
                row_gap: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(1.0)),
                ..Default::default()
            },
            self.background.unwrap_or(ThemedBackground::primary()),
            BackgroundColor(Color::BLACK),
            Children::spawn(SpawnWith(move |sp: &mut RelatedSpawner<ChildOf>| {
                let mut entity_commands = sp.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        min_height: Val::Px(0.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        overflow: Overflow::scroll_y(),
                        flex_wrap: FlexWrap::Wrap,
                        column_gap,
                        row_gap,
                        align_content: AlignContent::FlexStart,
                        ..Default::default()
                    },
                    ScrollView,
                    ScrollPosition::default(),
                    Children::spawn(SpawnIter(data.into_iter())),
                ));
                if let Some(marker) = marker {
                    entity_commands.insert(marker);
                }
                let view = entity_commands.id();

                // 滑动条
                sp.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(6.0),
                        top: Val::Px(6.0),
                        bottom: Val::Px(6.0),
                        width: Val::Px(10.0),
                        padding: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..Default::default()
                    },
                    Scrollbar::new(view, ControlOrientation::Vertical, 24.0),
                    Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
                        spawner.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                ..Default::default()
                            },
                            ThemedBackground::elevated(),
                            BackgroundColor(Color::BLACK),
                            CoreScrollbarThumb,
                        ));
                    })),
                ));
            })),
        )
    }
}

/// Empty marker for when no child marker is needed
#[derive(Component, Default)]
struct NoMarker;

#[derive(Component)]
struct ScrollView;

fn handle_mouse_wheel_scroll(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scroll_views: Query<(&mut ScrollPosition, &Node, &ComputedNode), With<ScrollView>>,
    parents: Query<&ChildOf>,
) {
    for event in mouse_wheel_events.read() {
        let mut delta_y = -event.y;
        if event.unit == MouseScrollUnit::Line {
            delta_y *= SCROLL_LINE_SPEED;
        }

        if delta_y == 0.0 {
            continue;
        }

        let mut consumed = false;
        for pointer_map in hover_map.values() {
            for hovered in pointer_map.keys().copied() {
                if scroll_from_hovered_entity(hovered, delta_y, &mut scroll_views, &parents) {
                    consumed = true;
                    break;
                }
            }
            if consumed {
                break;
            }
        }
    }
}

fn handle_touch_scroll(
    touches: Res<Touches>,
    hover_map: Res<HoverMap>,
    mut scroll_views: Query<(&mut ScrollPosition, &Node, &ComputedNode), With<ScrollView>>,
    parents: Query<&ChildOf>,
) {
    for touch in touches.iter() {
        let delta_y = -touch.delta().y;
        if delta_y.abs() <= f32::EPSILON {
            continue;
        }

        let mut consumed = false;
        for pointer_map in hover_map.values() {
            for hovered in pointer_map.keys().copied() {
                if scroll_from_hovered_entity(hovered, delta_y, &mut scroll_views, &parents) {
                    consumed = true;
                    break;
                }
            }
            if consumed {
                break;
            }
        }
    }
}

fn scroll_from_hovered_entity(
    start: Entity,
    delta_y: f32,
    scroll_views: &mut Query<(&mut ScrollPosition, &Node, &ComputedNode), With<ScrollView>>,
    parents: &Query<&ChildOf>,
) -> bool {
    let mut current = Some(start);
    let mut depth = 0;

    while let Some(entity) = current {
        if let Ok((mut scroll_position, node, computed)) = scroll_views.get_mut(entity) {
            if node.overflow.y != OverflowAxis::Scroll {
                return false;
            }

            let max_offset =
                (computed.content_size() - computed.size()) * computed.inverse_scale_factor();
            let max_scroll = max_offset.y.max(0.0);
            let old_y = scroll_position.y;
            scroll_position.y = (scroll_position.y + delta_y).clamp(0.0, max_scroll);
            return (scroll_position.y - old_y).abs() > f32::EPSILON;
        }

        depth += 1;
        if depth > 64 {
            break;
        }
        current = parents.get(entity).ok().map(ChildOf::parent);
    }

    false
}
