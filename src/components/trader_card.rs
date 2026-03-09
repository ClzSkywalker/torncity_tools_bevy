use bevy::{ecs::relationship::RelatedSpawner, prelude::*};
use bevy_remote_image::{RemoteImagePlugin, RemoteImageTarget};
use bevy_theme::prelude::*;

use super::browser::BrowserProvider;

#[derive(Debug, Clone, Default)]
pub struct TraderCardData {
    pub name: String,
    pub total_profit: i64,
    pub link: String,
    pub items: Vec<TraderItemData>,
    pub is_new: bool,
}

impl PartialEq for TraderCardData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.total_profit == other.total_profit
            && self.link == other.link
            && self.items.len() == other.items.len()
            && self
                .items
                .iter()
                .zip(other.items.iter())
                .all(|(a, b)| a.eq(b))
            && self.is_new == other.is_new
    }
}

impl Eq for TraderCardData {}

impl TraderCardData {
    pub fn mock() -> Self {
        Self {
            name: "FrankCastle".to_string(),
            total_profit: 11750,
            link: "https://www.torn.com".to_string(),
            is_new: false,
            items: vec![
                TraderItemData {
                    image_url: "https://www.torn.com/images/items/35/large.png".to_string(),
                    official: true,
                    name: "Cobra Derringer".to_string(),
                    quantity: 1,
                    buy: 50000,
                    sell: 55000,
                    single_profit: 503,
                    total_profit: 5003,
                    percent: 3.09,
                },
                TraderItemData {
                    image_url: "https://www.torn.com/images/items/7/large.png".to_string(),
                    official: false,
                    name: "Golf Club".to_string(),
                    quantity: 5,
                    buy: 6000,
                    sell: 10750,
                    single_profit: 950,
                    total_profit: 4750,
                    percent: 44.19,
                },
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraderItemData {
    pub image_url: String,
    pub official: bool,
    pub name: String,
    pub quantity: u32,
    pub buy: i64,
    pub sell: i64,
    pub single_profit: i64,
    pub total_profit: i64,
    pub percent: f32,
}

impl PartialEq for TraderItemData {
    fn eq(&self, other: &Self) -> bool {
        self.image_url == other.image_url
            && self.official == other.official
            && self.name == other.name
            && self.quantity == other.quantity
            && self.buy == other.buy
            && self.sell == other.sell
    }
}

impl Eq for TraderItemData {}

impl Default for TraderItemData {
    fn default() -> Self {
        Self {
            image_url: "https://www.torn.com/images/items/35/large.png".to_string(),
            official: true,
            name: "Cobra Derringer".to_string(),
            quantity: 1,
            buy: 50000,
            sell: 55000,
            single_profit: 503,
            total_profit: 5003,
            percent: 3.09,
        }
    }
}

pub struct TraderCardPlugin;

impl Plugin for TraderCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RemoteImagePlugin)
            .add_systems(Update, handle_link_clicks);
    }
}

#[derive(Clone)]
pub struct CardConfig {
    pub card_width: f32,
    pub card_border_width: f32,
    pub section_border_width: f32,
    pub image_width: f32,
    pub image_height: f32,
    pub row_gap: f32,
    pub section_padding: f32,
    pub title_font_size: f32,
    pub value_font_size: f32,
}

impl Default for CardConfig {
    fn default() -> Self {
        Self {
            card_width: 300.0,
            card_border_width: 1.0,
            section_border_width: 1.0,
            image_width: 50.0,
            image_height: 30.0,
            row_gap: 6.0,
            section_padding: 6.0,
            title_font_size: 20.0,
            value_font_size: 14.0,
        }
    }
}

pub struct TraderCardSpawner {
    data: TraderCardData,
    config: CardConfig,
    placeholder: Handle<Image>,
}

impl TraderCardSpawner {
    pub fn new(data: TraderCardData, placeholder: Handle<Image>) -> Self {
        Self {
            data,
            config: CardConfig::default(),
            placeholder,
        }
    }

    pub fn bundle(self) -> impl Bundle {
        let config = self.config;
        let data = self.data;
        let placeholder = self.placeholder;

        (
            Node {
                width: Val::Px(config.card_width),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(config.card_border_width)),
                ..Default::default()
            },
            ThemedBackground::secondary(),
            ThemedBorder::active(),
            Children::spawn(SpawnWith(|spawner: &mut RelatedSpawner<ChildOf>| {
                if data.is_new {
                    spawner.spawn((
                        header_section_bundle(data.clone(), config.clone()),
                        ThemedState::success(),
                    ));
                } else {
                    spawner.spawn(header_section_bundle(data.clone(), config.clone()));
                }
                spawner.spawn(items_section_bundle(data, config, placeholder));
            })),
        )
    }
}

fn header_section_bundle(data: TraderCardData, config: CardConfig) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(config.row_gap),
            padding: UiRect::all(Val::Px(config.section_padding)),
            border: UiRect::bottom(Val::Px(config.section_border_width)),
            ..Default::default()
        },
        BackgroundColor(Color::NONE),
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn((
                Node {
                    width: Val::Percent(70.0),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                Children::spawn(SpawnWith(move |spawner2: &mut RelatedSpawner<ChildOf>| {
                    spawner2.spawn((
                        ThemedText::primary(),
                        label_value_text_bundle(
                            None,
                            format!("Name:{}", data.name),
                            config.title_font_size,
                        ),
                    ));
                    spawner2.spawn((
                        ThemedText::primary(),
                        label_value_text_bundle(
                            None,
                            format!("Profit:{}", data.total_profit),
                            config.title_font_size,
                        ),
                    ));
                })),
            ));
            spawner.spawn((
                Button,
                Node {
                    width: Val::Percent(30.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(2.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::NONE),
                CardLinkButton {
                    url: data.link.clone(),
                },
                Children::spawn(SpawnWith(move |spawner2: &mut RelatedSpawner<ChildOf>| {
                    spawner2.spawn((
                        ThemedText::primary(),
                        label_value_text_bundle(
                            None,
                            "Link".to_string(),
                            config.value_font_size,
                        ),
                    ));
                })),
            ));
        })),
    )
}

fn items_section_bundle(
    data: TraderCardData,
    config: CardConfig,
    placeholder: Handle<Image>,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        Children::spawn(SpawnIter(
            data.items
                .into_iter()
                .map(move |item| item_bundle(item, config.clone(), placeholder.clone())),
        )),
    )
}

fn item_bundle(item: TraderItemData, config: CardConfig, placeholder: Handle<Image>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            column_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(config.section_padding)),
            border: UiRect::bottom(Val::Px(config.section_border_width)),
            ..Default::default()
        },
        ThemedBorder::active(),
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn(product_header_bundle(
                item.clone(),
                config.clone(),
                placeholder,
            ));
            spawner.spawn(product_body_bundle(item, config));
        })),
    )
}

fn product_header_bundle(
    item: TraderItemData,
    config: CardConfig,
    placeholder: Handle<Image>,
) -> impl Bundle {
    let official = item.official;

    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn((
                RemoteImageTarget {
                    url: item.image_url.clone(),
                },
                Node {
                    width: Val::Px(config.image_width),
                    height: Val::Px(config.image_height),
                    border: UiRect::all(Val::Px(1.0)),
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                ImageNode::new(placeholder.clone()),
            ));

            if official {
                spawner.spawn((
                    Node {
                        width: Val::Px(config.value_font_size + 10.0),
                        height: Val::Px(config.value_font_size + 10.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::horizontal(Val::Px(6.0)),
                        border_radius: BorderRadius::MAX,
                        ..Default::default()
                    },
                    ThemedBorder::active(),
                    BackgroundColor(Color::NONE),
                    Children::spawn(SpawnWith(move |spawner2: &mut RelatedSpawner<ChildOf>| {
                        spawner2.spawn((
                            ThemedText::primary(),
                            Text::new("官"),
                            TextFont {
                                font_size: config.value_font_size - 2.0,
                                ..Default::default()
                            },
                        ));
                    })),
                ));
            }

            spawner.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(config.row_gap),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                Children::spawn(SpawnWith(move |spawner2: &mut RelatedSpawner<ChildOf>| {
                    spawner2.spawn((
                        ThemedText::primary(),
                        label_value_text_bundle(
                            None,
                            format!("Name:{}", item.name),
                            config.value_font_size,
                        ),
                    ));
                    spawner2.spawn((
                        ThemedText::secondary(),
                        label_value_text_bundle(
                            None,
                            format!("Quantity:{}", item.quantity),
                            config.value_font_size,
                        ),
                    ));
                })),
            ));
        })),
    )
}

fn product_body_bundle(item: TraderItemData, config: CardConfig) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(config.row_gap),
            flex_grow: 1.0,
            ..Default::default()
        },
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn((
                ThemedText::secondary(),
                label_value_text_bundle(
                    None,
                    format!("Buy:{}", item.buy),
                    config.value_font_size,
                ),
            ));
            spawner.spawn((
                ThemedText::secondary(),
                label_value_text_bundle(
                    None,
                    format!("Sell:{}", item.sell),
                    config.value_font_size,
                ),
            ));
            spawner.spawn((
                ThemedText::secondary(),
                label_value_text_bundle(
                    None,
                    format!("Total Profit:{}", item.total_profit),
                    config.value_font_size,
                ),
            ));
            spawner.spawn((
                ThemedText::secondary(),
                label_value_text_bundle(
                    None,
                    format!("Single Profit:{}", item.single_profit),
                    config.value_font_size,
                ),
            ));
            spawner.spawn((
                ThemedText::secondary(),
                label_value_text_bundle(
                    None,
                    format!("Percent:{:.2}%", item.percent),
                    config.value_font_size,
                ),
            ));
        })),
    )
}

fn label_value_text_bundle(
    width_percent: Option<f32>,
    value: String,
    font_size: f32,
) -> impl Bundle {
    let width = width_percent.map(Val::Percent).unwrap_or_default();

    (
        Node {
            width,
            overflow: Overflow::clip(),
            ..Default::default()
        },
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn((
                Text::new(value),
                TextFont {
                    font_size,
                    ..Default::default()
                },
            ));
        })),
    )
}

#[derive(Component, Debug, Clone)]
pub struct CardLinkButton {
    pub url: String,
}

fn handle_link_clicks(
    query: Query<(&Interaction, &CardLinkButton), (Changed<Interaction>, With<Button>)>,
) {
    let browser = BrowserProvider::new();
    for (interaction, link) in &query {
        if *interaction == Interaction::Pressed
            && let Err(err) = browser.open(&link.url)
        {
            warn!("open url failed: {}, err: {err}", link.url);
        }
    }
}
