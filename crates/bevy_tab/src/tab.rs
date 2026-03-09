use bevy_camera::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_text::prelude::*;
use bevy_ui::prelude::*;

use bevy_theme::prelude::*;

#[derive(Clone)]
pub struct TabItemConfig {
    pub id: String,
    pub label: String,
}

impl TabItemConfig {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

#[derive(Resource, Clone)]
pub struct ViewTabConfig {
    pub tabs: Vec<TabItemConfig>,
    pub initial_tab: String,
    pub style: TabStyleConfig,
}

impl Default for ViewTabConfig {
    fn default() -> Self {
        let tabs = vec![
            TabItemConfig::new("home", "Home"),
            TabItemConfig::new("setting", "Setting"),
        ];
        Self {
            initial_tab: "home".to_string(),
            tabs,
            style: TabStyleConfig::default(),
        }
    }
}

impl ViewTabConfig {
    pub fn normalized(mut self) -> Self {
        if self.tabs.is_empty() {
            self.tabs = ViewTabConfig::default().tabs;
        }

        self.tabs.retain(|tab| !tab.id.is_empty());
        if self.tabs.is_empty() {
            self.tabs = ViewTabConfig::default().tabs;
        }

        if !self.tabs.iter().any(|tab| tab.id == self.initial_tab) {
            self.initial_tab = self.tabs[0].id.clone();
        }
        self
    }
}

#[derive(Clone)]
pub struct TabStyleConfig {
    pub root_padding: UiRect,
    pub content_padding: UiRect,
    pub tab_bar_height: Val,
    pub tab_bar_padding: UiRect,
    pub tab_bar_gap: Val,
    pub tab_button_height: Val,
    pub tab_button_radius: BorderRadius,
    pub tab_font_size: f32,
}

impl Default for TabStyleConfig {
    fn default() -> Self {
        Self {
            root_padding: UiRect::all(px(5.0)),
            content_padding: UiRect::all(px(0.0)),
            tab_bar_height: px(70.0),
            tab_bar_padding: UiRect::axes(px(8.0), px(5.0)),
            tab_bar_gap: px(10.0),
            tab_button_height: px(46.0),
            tab_button_radius: BorderRadius::all(px(10.0)),
            tab_font_size: 20.0,
        }
    }
}

#[derive(Resource, Clone)]
pub struct ActiveTab(pub String);

#[derive(Component)]
pub struct TabButton {
    pub id: String,
}

#[derive(Component)]
pub struct TabBarRoot;

#[derive(Component)]
pub struct TabButtonLabel;

#[derive(Component)]
pub struct TabPanel {
    pub id: String,
}

#[derive(Component)]
pub struct TabContentRootMarker;

#[derive(Component)]
pub struct TabContentRoot {
    pub id: String,
}

pub fn build_tab_view(
    mut commands: Commands,
    config: Res<ViewTabConfig>,
) {
    commands
        .spawn((
            Name::new("ViewRoot"),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: config.style.root_padding,
                ..Default::default()
            },
            ThemedBackground::deep(),
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|root| {
            root.spawn((
                Name::new("ContentContainer"),
                TabContentRootMarker,
                Node {
                    width: percent(100.0),
                    flex_grow: 1.0,
                    min_height: px(0.0),
                    padding: config.style.content_padding,
                    ..Default::default()
                },
            ))
            .with_children(|content| {
                for tab in &config.tabs {
                    content.spawn((
                        Name::new(tab_name(tab.id.clone())),
                        TabContentRoot { id: tab.id.clone() },
                        TabPanel { id: tab.id.clone() },
                        Node {
                            width: percent(100.0),
                            flex_grow: 1.0,
                            min_height: px(0.0),
                            display: if tab.id == config.initial_tab {
                                Display::Flex
                            } else {
                                Display::None
                            },
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        if tab.id == config.initial_tab {
                            Visibility::Visible
                        } else {
                            Visibility::Hidden
                        },
                    ));
                }
            });

            root.spawn((
                Name::new("TabBar"),
                TabBarRoot,
                Node {
                    width: percent(100.0),
                    height: config.style.tab_bar_height,
                    max_height: config.style.tab_bar_height,
                    min_height: config.style.tab_bar_height,
                    flex_direction: FlexDirection::Row,
                    column_gap: config.style.tab_bar_gap,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    padding: config.style.tab_bar_padding,
                    ..Default::default()
                },
                ThemedBackground::deep(),
                BackgroundColor(Color::BLACK),
            ))
            .with_children(|bar| {
                for tab in &config.tabs {
                    spawn_tab_button(bar, tab, &config.style, tab.id == config.initial_tab);
                }
            });
        });
}

fn spawn_tab_button(
    parent: &mut ChildSpawnerCommands,
    tab: &TabItemConfig,
    style: &TabStyleConfig,
    is_active: bool,
) {
    parent
        .spawn((
            Button,
            TabButton {
                id: tab.id.clone(),
            },
            Node {
                flex_grow: 1.0,
                height: style.tab_button_height,
                min_height: style.tab_button_height,
                max_height: style.tab_button_height,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: style.tab_button_radius,
                ..Default::default()
            },
            if is_active {
                ThemedBackground::elevated()
            } else {
                ThemedBackground::secondary()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|button| {
            button.spawn((
                TabButtonLabel,
                Text::new(tab.label.clone()),
                TextFont {
                    font_size: style.tab_font_size,
                    ..Default::default()
                },
                if is_active {
                    ThemedText::primary()
                } else {
                    ThemedText::secondary()
                },
                TextColor(Color::BLACK),
            ));
        });
}

pub fn switch_active_tab(
    mut active_tab: ResMut<ActiveTab>,
    query: Query<(&Interaction, &TabButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, tab_button) in &query {
        if *interaction == Interaction::Pressed {
            active_tab.0 = tab_button.id.clone();
        }
    }
}

pub fn update_tab_visibility(
    active_tab: Res<ActiveTab>,
    mut query: Query<(&TabPanel, &mut Visibility, &mut Node)>,
) {
    for (panel, mut visibility, mut node) in &mut query {
        let is_active = panel.id == active_tab.0;
        *visibility = if is_active {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        node.display = if is_active {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn sync_active_tab_style(
    active_tab: Res<ActiveTab>,
    mut button_query: Query<(&TabButton, &mut ThemedBackground, &Children), With<Button>>,
    mut text_query: Query<(&TabButtonLabel, &mut ThemedText)>,
) {
    for (tab_button, mut themed_bg, children) in &mut button_query {
        let is_active = tab_button.id == active_tab.0;
        themed_bg.layer = if is_active {
            ThemedBackgroundLayer::Elevated
        } else {
            ThemedBackgroundLayer::Secondary
        };

        for &child in children {
            if let Ok((_, mut themed_text)) = text_query.get_mut(child) {
                themed_text.layer = if is_active {
                    ThemedTextLayer::Primary
                } else {
                    ThemedTextLayer::Secondary
                };
            }
        }
    }
}

fn tab_name(name: String) -> String {
    format!("ContentRoot_{}", name)
}
