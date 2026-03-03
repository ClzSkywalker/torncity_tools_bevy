use bevy::{ecs::relationship::RelatedSpawner, prelude::*};
use bevy_remote_image::{RemoteImagePlugin, RemoteImageTarget};

use super::browser::BrowserProvider;

#[derive(Debug, Clone, Default)]
pub struct TraderCardData {
    pub name: String,
    pub total_profit: i64,
    pub link: String,
    pub items: Vec<TraderItemData>,
}

impl TraderCardData {
    pub fn mock() -> Self {
        Self {
            name: "FrankCastle".to_string(),
            total_profit: 11750,
            link: "https://www.torn.com".to_string(),
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

#[derive(Debug, Clone)]
pub struct CardTheme {
    pub card_width: f32,
    pub card_background: Color,
    pub card_border_color: Color,
    pub card_border_width: f32,
    pub section_border_color: Color,
    pub section_border_width: f32,
    pub image_width: f32,
    pub image_height: f32,
    pub row_gap: f32,
    pub section_padding: f32,
    pub title_font_size: f32,
    pub value_font_size: f32,
    pub text_color: Color,
    pub muted_text_color: Color,
    pub placeholder_image_path: String,
}

impl Default for CardTheme {
    fn default() -> Self {
        Self {
            card_width: 300.0,
            card_background: Color::srgb(0.17, 0.17, 0.17),
            card_border_color: Color::srgb(0.20, 0.65, 0.22),
            card_border_width: 1.0,
            section_border_color: Color::srgb(0.20, 0.65, 0.22),
            section_border_width: 1.0,
            image_width: 50.0,
            image_height: 30.0,
            row_gap: 6.0,
            section_padding: 6.0,
            title_font_size: 20.0,
            value_font_size: 14.0,
            text_color: Color::WHITE,
            muted_text_color: Color::srgb(0.80, 0.80, 0.80),
            placeholder_image_path: "icons/loading.png".to_string(),
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

pub struct TraderCardSpawner {
    data: TraderCardData,
    theme: CardTheme,
    placeholder: Handle<Image>,
}

impl TraderCardSpawner {
    pub fn new(data: TraderCardData, theme: CardTheme, placeholder: Handle<Image>) -> Self {
        Self {
            data,
            theme,
            placeholder,
        }
    }

    pub fn bundle(self) -> impl Bundle {
        let card_width = self.theme.card_width;
        let card_border_width = self.theme.card_border_width;
        let card_background = self.theme.card_background;
        let card_border_color = self.theme.card_border_color;

        let data = self.data;
        let theme = self.theme;
        let placeholder = self.placeholder;

        (
            Node {
                width: Val::Px(card_width),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(card_border_width)),
                ..Default::default()
            },
            BackgroundColor(card_background),
            BorderColor::all(card_border_color),
            Children::spawn(SpawnWith(|spawner: &mut RelatedSpawner<ChildOf>| {
                spawner.spawn(header_section_bundle(data.clone(), theme.clone()));
                spawner.spawn(items_section_bundle(data, theme, placeholder));
            })),
        )
    }
}

fn header_section_bundle(data: TraderCardData, theme: CardTheme) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(theme.row_gap),
            padding: UiRect::all(Val::Px(theme.section_padding)),
            border: UiRect::bottom(Val::Px(theme.section_border_width)),
            ..Default::default()
        },
        BorderColor::all(theme.section_border_color),
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            // 左侧信息列
            spawner.spawn((
                Node {
                    width: Val::Percent(70.0),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                Children::spawn(SpawnWith(move |spawner2: &mut RelatedSpawner<ChildOf>| {
                    spawner2.spawn(label_value_text_bundle(
                        None,
                        format!("Name:{}", data.name),
                        theme.title_font_size,
                        theme.text_color,
                    ));
                    spawner2.spawn(label_value_text_bundle(
                        None,
                        format!("Profit:{}", data.total_profit),
                        theme.title_font_size,
                        theme.text_color,
                    ));
                })),
            ));
            // 右侧链接按钮
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
                    spawner2.spawn(label_value_text_bundle(
                        None,
                        "Link".to_string(),
                        theme.value_font_size,
                        theme.text_color,
                    ));
                })),
            ));
        })),
    )
}

fn items_section_bundle(
    data: TraderCardData,
    theme: CardTheme,
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
                .map(move |item| item_bundle(item, theme.clone(), placeholder.clone())),
        )),
    )
}

fn item_bundle(item: TraderItemData, theme: CardTheme, placeholder: Handle<Image>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            column_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(theme.section_padding)),
            border: UiRect::bottom(Val::Px(theme.section_border_width)),
            ..Default::default()
        },
        BorderColor::all(theme.section_border_color),
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn(product_header_bundle(
                item.clone(),
                theme.clone(),
                placeholder,
            ));
            spawner.spawn(product_body_bundle(item, theme));
        })),
    )
}

fn product_header_bundle(
    item: TraderItemData,
    theme: CardTheme,
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
            // 图片
            spawner.spawn((
                RemoteImageTarget {
                    url: item.image_url.clone(),
                },
                Node {
                    width: Val::Px(theme.image_width),
                    height: Val::Px(theme.image_height),
                    border: UiRect::all(Val::Px(1.0)),
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                ImageNode::new(placeholder.clone()),
            ));

            // 官方标记（可选）
            if official {
                spawner.spawn((
                    Node {
                        width: Val::Px(theme.value_font_size + 10.0),
                        height: Val::Px(theme.value_font_size + 10.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::left(Val::Px(6.0)),
                        border_radius: BorderRadius::MAX,
                        ..Default::default()
                    },
                    BorderColor::all(theme.text_color),
                    BackgroundColor(Color::NONE),
                    Children::spawn(SpawnWith(move |spawner2: &mut RelatedSpawner<ChildOf>| {
                        spawner2.spawn((
                            Text::new("官"),
                            TextFont {
                                font_size: theme.value_font_size - 2.0,
                                ..Default::default()
                            },
                            TextColor(theme.text_color),
                        ));
                    })),
                ));
            }

            // 商品信息列
            spawner.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(theme.row_gap),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                Children::spawn(SpawnWith(move |spawner2: &mut RelatedSpawner<ChildOf>| {
                    spawner2.spawn(label_value_text_bundle(
                        None,
                        format!("Name:{}", item.name),
                        theme.value_font_size,
                        theme.text_color,
                    ));
                    spawner2.spawn(label_value_text_bundle(
                        None,
                        format!("Quantity:{}", item.quantity),
                        theme.value_font_size,
                        theme.muted_text_color,
                    ));
                })),
            ));
        })),
    )
}

fn product_body_bundle(item: TraderItemData, theme: CardTheme) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(theme.row_gap),
            flex_grow: 1.0,
            ..Default::default()
        },
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn(label_value_text_bundle(
                None,
                format!("Buy:{}", item.buy),
                theme.value_font_size,
                theme.muted_text_color,
            ));
            spawner.spawn(label_value_text_bundle(
                None,
                format!("Sell:{}", item.sell),
                theme.value_font_size,
                theme.muted_text_color,
            ));
            spawner.spawn(label_value_text_bundle(
                None,
                format!("Total Profit:{}", item.total_profit),
                theme.value_font_size,
                theme.muted_text_color,
            ));
            spawner.spawn(label_value_text_bundle(
                None,
                format!("Single Profit:{}", item.single_profit),
                theme.value_font_size,
                theme.muted_text_color,
            ));
            spawner.spawn(label_value_text_bundle(
                None,
                format!("Percent:{:.2}%", item.percent),
                theme.value_font_size,
                theme.muted_text_color,
            ));
        })),
    )
}

fn label_value_text_bundle(
    width_percent: Option<f32>,
    value: String,
    font_size: f32,
    color: Color,
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
                TextColor(color),
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
