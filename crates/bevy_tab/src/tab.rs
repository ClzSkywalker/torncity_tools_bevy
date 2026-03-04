use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;
use bevy_text::prelude::*;
use bevy_color::prelude::*;
use bevy_camera::prelude::*;

#[derive(Clone)]
pub struct TabItemConfig {
    pub id: String,
    pub label: String,
    pub active_background: Option<Color>,
    pub inactive_background: Option<Color>,
    pub active_text: Option<Color>,
    pub inactive_text: Option<Color>,
}

impl TabItemConfig {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            active_background: None,
            inactive_background: None,
            active_text: None,
            inactive_text: None,
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
    pub root_background: Color,
    pub root_padding: UiRect,
    pub content_padding: UiRect,
    pub tab_bar_background: Color,
    pub tab_bar_height: Val,
    pub tab_bar_padding: UiRect,
    pub tab_bar_gap: Val,
    pub tab_button_height: Val,
    pub tab_button_radius: BorderRadius,
    pub tab_font_size: f32,
    pub active_background: Color,
    pub inactive_background: Color,
    pub active_text: Color,
    pub inactive_text: Color,
}

impl Default for TabStyleConfig {
    fn default() -> Self {
        Self {
            root_background: Color::srgb(0.1, 0.1, 0.12),
            root_padding: UiRect::all(px(5.0)),
            content_padding: UiRect::all(px(0.0)),
            tab_bar_background: Color::srgb(0.08, 0.08, 0.1),
            tab_bar_height: px(70.0),
            tab_bar_padding: UiRect::axes(px(8.0), px(5.0)),
            tab_bar_gap: px(10.0),
            tab_button_height: px(46.0),
            tab_button_radius: BorderRadius::all(px(10.0)),
            tab_font_size: 20.0,
            active_background: Color::srgb(0.16, 0.49, 0.96),
            inactive_background: Color::srgb(0.18, 0.18, 0.18),
            active_text: Color::WHITE,
            inactive_text: Color::srgb(0.76, 0.76, 0.76),
        }
    }
}

#[derive(Resource, Clone)]
pub struct ActiveTab(pub String);

#[derive(Component)]
pub struct TabButton {
    pub id: String,
    pub active_background: Color,
    pub inactive_background: Color,
    pub active_text: Color,
    pub inactive_text: Color,
}

#[derive(Component)]
pub struct TabPanel {
    pub id: String,
}

#[derive(Component)]
pub struct TabContentRoot {
    pub id: String,
}

pub fn build_tab_view(mut commands: Commands, config: Res<ViewTabConfig>) {
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
            BackgroundColor(config.style.root_background),
        ))
        .with_children(|root| {
            root.spawn((
                Name::new("ContentContainer"),
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
                        Name::new(format!("ContentRoot_{}", tab.id)),
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
                Node {
                    width: percent(100.0),
                    height: config.style.tab_bar_height,
                    flex_direction: FlexDirection::Row,
                    column_gap: config.style.tab_bar_gap,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    padding: config.style.tab_bar_padding,
                    ..Default::default()
                },
            ))
            .with_children(|bar| {
                for tab in &config.tabs {
                    spawn_tab_button(bar, tab, &config.style);
                }
            });
        });
}

fn spawn_tab_button(parent: &mut ChildSpawnerCommands, tab: &TabItemConfig, style: &TabStyleConfig) {
    let active_background = tab.active_background.unwrap_or(style.active_background);
    let inactive_background = tab.inactive_background.unwrap_or(style.inactive_background);
    let active_text = tab.active_text.unwrap_or(style.active_text);
    let inactive_text = tab.inactive_text.unwrap_or(style.inactive_text);

    parent
        .spawn((
            Button,
            TabButton {
                id: tab.id.clone(),
                active_background,
                inactive_background,
                active_text,
                inactive_text,
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
            BackgroundColor(inactive_background),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(tab.label.clone()),
                TextFont {
                    font_size: style.tab_font_size,
                    ..Default::default()
                },
                TextColor(inactive_text),
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
    if !active_tab.is_changed() {
        return;
    }

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
    mut query: Query<(&TabButton, &mut BackgroundColor, &Children), With<Button>>,
    mut text_query: Query<&mut TextColor>,
) {
    if !active_tab.is_changed() {
        return;
    }

    for (tab_button, mut background, children) in &mut query {
        let is_active = tab_button.id == active_tab.0;
        background.0 = if is_active {
            tab_button.active_background
        } else {
            tab_button.inactive_background
        };

        if let Some(&label_entity) = children.first()
            && let Ok(mut text_color) = text_query.get_mut(label_entity)
        {
            text_color.0 = if is_active {
                tab_button.active_text
            } else {
                tab_button.inactive_text
            };
        }
    }
}
