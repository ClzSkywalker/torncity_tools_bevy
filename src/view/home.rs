use crate::components::{scroll::ScrollSpawn, trader_card::*};
use crate::game::GameState;
use crate::http::favorites::{Weav3rRespComp, Weav3rSysResource};
use crate::resource::items_data::{ItemsDatabase, office_item_startup};
use crate::weav3r;
use crate::weav3r::profit::ProfitUserInfo;
use bevy::prelude::*;
use bevy_tab::tab::{TabContentRoot, build_tab_view};

use crate::view::{
    TabId,
    res::{SettingConfigRes, Weav3rFavRes},
};

pub struct Weav3rHomePlugin;

impl Plugin for Weav3rHomePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SettingConfigRes::default())
            .add_systems(OnEnter(GameState::Menu), view.after(build_tab_view))
            .add_systems(
                OnEnter(GameState::InitConfig),
                init_weav3r_fav_res.after(office_item_startup),
            )
            .add_systems(
                Update,
                update_weav3r_fav_res
                    .run_if(resource_changed::<SettingConfigRes>)
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                Update,
                (handle_weav3r_send_req_btn, handle_weav3r_resp).run_if(in_state(GameState::Menu)),
            );
    }
}

fn view(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    content_query: Query<(Entity, &TabContentRoot)>,
) {
    let mut home_root = None;

    for (entity, root) in &content_query {
        if root.id.as_str() == TabId::Home.name() {
            home_root = Some(entity);
            break;
        }
    }

    let Some(home_root) = home_root else {
        bevy::log::error!("home_root not found");
        return;
    };

    setup(&mut commands, &asset_server, home_root);
}

#[derive(Component)]
#[require(Button)]
struct Weav3rSendReqBtn;

#[derive(Component)]
struct Weav3rTraderCardItem;

pub fn setup(cmd: &mut Commands, asset_server: &AssetServer, parent: Entity) -> Entity {
    let _ = asset_server;
    let placeholder = Handle::<Image>::default();
    let demo_data = TraderCardData::mock();
    let data = vec![demo_data; 10];

    let home_root = cmd
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::Start,
                ..Default::default()
            },
            // 按钮
            Children::spawn_one((
                Node {
                    width: percent(100.),
                    height: Val::Px(30.),
                    max_height: Val::Px(30.),
                    flex_shrink: 0.0,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                children![(
                    Node {
                        width: percent(30.0),
                        height: percent(100.0),
                        flex_shrink: 0.0,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        overflow: Overflow::clip(),
                        ..Default::default()
                    },
                    BackgroundColor(Color::Srgba(Srgba::GREEN)),
                    Weav3rSendReqBtn,
                    children![(
                        Text::new("请求数据"),
                        TextFont {
                            font_size: 20.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new(Justify::Center, LineBreak::NoWrap)
                    )],
                )],
            )),
        ))
        .id();
    cmd.entity(parent).add_child(home_root);

    let trader_bundles = data
        .into_iter()
        .map(|item| {
            (
                Weav3rTraderCardItem,
                TraderCardSpawner::new(item, CardTheme::default(), placeholder.clone()).bundle(),
            )
        })
        .collect::<Vec<_>>();

    let scroll_spawn = ScrollSpawn {
        width: percent(100.0),
        height: percent(100.0),
        column_gap: px(10.0),
        row_gap: px(10.0),
    };
    let scroll_entity = cmd.spawn(scroll_spawn.bundle(trader_bundles)).id();
    cmd.entity(home_root).add_child(scroll_entity);
    scroll_entity
}

// 处理发送请求按钮的点击事件
fn handle_weav3r_send_req_btn(
    mut cmd: Commands,
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<Weav3rSendReqBtn>)>,
    weav3r_req_sys_resource: Res<Weav3rSysResource>,
    setting_config: Res<SettingConfigRes>,
    items_database: Res<ItemsDatabase>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            let target_ids = items_database
                .items
                .iter()
                .filter(|x| x.tradeable && x.sell_price >= setting_config.office_price_start)
                .map(|x| x.id)
                .chain(
                    setting_config
                        .target_ids
                        .split(',')
                        .map(|x| x.parse::<i32>().unwrap()),
                )
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");

            let sys_id = weav3r_req_sys_resource.0;
            cmd.run_system_with(
                sys_id,
                (
                    target_ids,
                    setting_config.token.clone(),
                    setting_config.cookie.clone(),
                ),
            );
            bevy::log::info!("weav3r: send request");
        }
    }
}

// 处理响应结果
fn handle_weav3r_resp(
    mut cmd: Commands,
    _asset_server: Res<AssetServer>,
    query: Query<(Entity, &Weav3rRespComp)>,
    trader_card_query: Query<(Entity, &ChildOf), With<Weav3rTraderCardItem>>,
    mut weav3r_fav_res: ResMut<Weav3rFavRes>,
) {
    for (entity, weav3r_resp_resource) in &query {
        cmd.entity(entity).despawn();

        let responses = weav3r_resp_resource.responses.clone();

        // todo 这里要保存
        let favorites_res = &mut weav3r_fav_res.0;
        favorites_res.set_new_profit(responses.items);

        let trader_card_data = favorites_res
            .user_profit_result
            .iter()
            .cloned()
            .map(profit_to_trader_card_data)
            .collect::<Vec<_>>();

        let mut cards_parent = None;
        for (card_entity, childof) in &trader_card_query {
            if cards_parent.is_none() {
                cards_parent = Some(childof.parent());
            }
            cmd.entity(card_entity).despawn();
        }

        if trader_card_data.is_empty() {
            bevy::log::info!("weav3r: trader_card_data is empty");
            continue;
        }
        bevy::log::info!("weav3r: load {} trader cards", trader_card_data.len());

        let placeholder = Handle::<Image>::default();
        let trader_bundles = trader_card_data
            .into_iter()
            .take(20)
            .map(|item| {
                (
                    Weav3rTraderCardItem,
                    TraderCardSpawner::new(item, CardTheme::default(), placeholder.clone())
                        .bundle(),
                )
            })
            .collect::<Vec<_>>();

        if let Some(cards_parent) = cards_parent {
            cmd.entity(cards_parent).with_children(|builder| {
                for trader_card_bundle in trader_bundles {
                    builder.spawn(trader_card_bundle);
                }
            });
        }
    }
}

fn profit_to_trader_card_data(favorites_res: ProfitUserInfo) -> TraderCardData {
    let items: Vec<TraderItemData> = favorites_res
        .items
        .into_iter()
        .map(|profit_item| TraderItemData {
            image_url: profit_item.image,
            official: profit_item.final_profit.is_office,
            name: profit_item.name,
            quantity: profit_item.quantity as u32,
            buy: profit_item.single_recyle_price as i64,
            sell: profit_item.final_profit.single_sell_price as i64,
            single_profit: profit_item.final_profit.single_profit_value,
            total_profit: profit_item.final_profit.total_profit_value,
            percent: profit_item.final_profit.percentage,
        })
        .collect();

    TraderCardData {
        name: favorites_res.player_name,
        total_profit: favorites_res.total_profit_price,
        link: weav3r::profit::get_bazaar_url(favorites_res.player_id),
        items,
    }
}

// 初始化weav3r_fav_res官方数据
fn init_weav3r_fav_res(mut cmd: Commands, items_database: Res<ItemsDatabase>) {
    let mut weav3r_fav_res = Weav3rFavRes::default();
    let favorites_res = &mut weav3r_fav_res.0;
    favorites_res.filter.office_item_map = items_database
        .items
        .iter()
        .map(|item| (item.id, item.clone()))
        .collect();
    cmd.insert_resource(weav3r_fav_res);
}

// 更新weav3r_fav_res数据
fn update_weav3r_fav_res(
    setting_config: Res<SettingConfigRes>,
    mut weav3r_fav_res: ResMut<Weav3rFavRes>,
) {
    let favorites_res = &mut weav3r_fav_res.0;
    favorites_res.filter.min_profit = setting_config.min_profit;
    favorites_res.filter.office_sell_price = setting_config.office_price_start;
    favorites_res.filter.office_sell_profit = setting_config.office_profit;
    favorites_res.filter.min_profit_percentage = setting_config.profit_percent;
    favorites_res.filter.target_ids = setting_config
        .target_ids
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}
