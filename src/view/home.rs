use crate::components::tick::CountDownState;
use crate::components::{scroll::ScrollSpawn, trader_card::*};
use crate::game::GameState;
use crate::http::favorites::{Weav3rRespComp, Weav3rSysResource};
use crate::resource::AudioAssets;
use crate::resource::items_data::{OfficeItemsDbRes, office_item_startup};
use crate::view::setting::{Weav3rUpdTickerComp, load_config_system};
use crate::view::trader_card_manager::{
    CurrentTraderCards, TraderCardScrollMarker, handle_trader_card_update,
};
use crate::weav3r;
use crate::weav3r::profit::ProfitUserInfo;
use bevy::prelude::*;
use bevy_ecs::relationship::RelatedSpawner;
use bevy_tab::tab::{TabContentRoot, build_tab_view};
use bevy_theme::prelude::*;
use bevy_toast::events::ToastEvent;
use bevy_toast::style::ToastPosition;

use crate::view::{
    TabId,
    res::{SettingConfigRes, Weav3rFavRes},
};
pub struct Weav3rHomePlugin;

impl Plugin for Weav3rHomePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentTraderCards>()
            .add_systems(
                OnEnter(GameState::InitConfig),
                init_weav3r_fav_res
                    .after(office_item_startup)
                    .after(load_config_system),
            )
            .add_systems(OnEnter(GameState::Menu), view.after(build_tab_view))
            .add_systems(
                OnEnter(GameState::Menu),
                (startup_trigger_weav3r_request, upd_running_state).after(view),
            )
            .add_systems(
                Update,
                (update_weav3r_fav_res_on_change, upd_running_state)
                    .run_if(resource_changed::<SettingConfigRes>)
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                Update,
                (
                    handle_weav3r_send_req_btn,
                    handle_weav3r_resp,
                    update_ticker,
                )
                    .run_if(in_state(GameState::Menu)),
            );
    }
}

fn view(mut commands: Commands, content_query: Query<(Entity, &TabContentRoot)>) {
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

    setup(&mut commands, home_root);
}

#[derive(Component)]
#[require(Button)]
struct Weav3rSendReqBtn;

#[derive(Component)]
struct Weav3rRunningComp;

pub fn setup(cmd: &mut Commands, parent: Entity) -> Entity {
    let home_root = cmd
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                ..Default::default()
            },
            Children::spawn_one((
                Node {
                    flex_direction: FlexDirection::Row,
                    width: percent(100.0),
                    height: Val::Px(50.),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
                    // 请求数据按钮
                    spawner.spawn((
                        Node {
                            width: percent(30.),
                            height: percent(100.),
                            flex_shrink: 0.0,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        children![(
                            Node {
                                width: percent(100.0),
                                height: percent(100.0),
                                flex_shrink: 0.0,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                overflow: Overflow::clip(),
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                ..Default::default()
                            },
                            ThemedBackground::primary(),
                            BackgroundColor(Color::BLACK),
                            Weav3rSendReqBtn,
                            children![(
                                Text::new("请求数据"),
                                TextFont {
                                    font_size: 20.0,
                                    ..Default::default()
                                },
                                ThemedText::primary(),
                                TextColor(Color::BLACK),
                                TextLayout::new(Justify::Center, LineBreak::NoWrap)
                            )],
                        )],
                    ));

                    // 状态图标
                    spawner.spawn((
                        Node {
                            width: percent(30.),
                            height: percent(100.),
                            flex_shrink: 0.0,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        children![
                            (
                                Weav3rRunningComp,
                                Node {
                                    width: Val::Px(30.),
                                    height: Val::Px(30.),
                                    border_radius: BorderRadius::all(Val::Px(15.)),
                                    ..Default::default()
                                },
                                BackgroundColor(Color::Srgba(Srgba::GREEN)),
                                ThemedState::success(),
                            ),
                            (
                                Weav3rRunningComp,
                                Text::new("运行中"),
                                TextLayout::new(Justify::Center, LineBreak::NoWrap),
                                TextColor(Color::WHITE),
                                ThemedText::primary(),
                            )
                        ],
                    ));
                })),
            )),
        ))
        .id();
    cmd.entity(parent).add_child(home_root);

    let scroll_spawn = ScrollSpawn::new()
        .with_width(percent(100.0))
        .with_height(percent(100.0))
        .with_column_gap(px(10.0))
        .with_row_gap(px(10.0));
    let scroll_entity = cmd
        .spawn(scroll_spawn.bundle_with_marker(Vec::<()>::new(), Some(TraderCardScrollMarker)))
        .id();
    bevy::log::info!(
        "setup: scroll_entity = {:?}, home_root = {:?}",
        scroll_entity,
        home_root
    );
    cmd.entity(home_root).add_child(scroll_entity);
    bevy::log::info!(
        "setup: after add_child, scroll_entity = {:?}, home_root = {:?}",
        scroll_entity,
        home_root
    );

    scroll_entity
}

// 处理发送请求按钮的点击事件
fn handle_weav3r_send_req_btn(
    mut cmd: Commands,
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<Weav3rSendReqBtn>)>,
    weav3r_req_sys_resource: Res<Weav3rSysResource>,
    setting_config: Res<SettingConfigRes>,
    items_database: Res<OfficeItemsDbRes>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            trigger_weav3r_request(
                &mut cmd,
                &items_database,
                &setting_config,
                &weav3r_req_sys_resource,
            );
        }
    }
}

fn upd_running_state(
    setting_config: Res<SettingConfigRes>,
    mut bgcolor: Single<&mut ThemedState, With<Weav3rRunningComp>>,
    mut text: Single<&mut Text, With<Weav3rRunningComp>>,
) {
    if setting_config.is_run {
        bgcolor.state_type = ThemedStateType::Success;
        text.0 = "运行中".to_string();
    } else {
        bgcolor.state_type = ThemedStateType::Error;
        text.0 = "未运行".to_string();
    }
}

// 处理响应结果
fn handle_weav3r_resp(
    mut cmd: Commands,
    query: Query<(Entity, &Weav3rRespComp)>,
    cards_parents: Single<Entity, With<TraderCardScrollMarker>>,
    mut weav3r_fav_res: ResMut<Weav3rFavRes>,
    mut current_cards: ResMut<CurrentTraderCards>,
    audio_assets: Res<AudioAssets>,
    mut setting_config: ResMut<SettingConfigRes>,
) {
    for (entity, weav3r_resp_resource) in &query {
        cmd.entity(entity).despawn();

        let responses = weav3r_resp_resource.responses.clone();

        if let Some(e) = weav3r_resp_resource.err.clone() {
            bevy::log::error!("weav3r_resp error: {:?}", e);
            setting_config.is_run = false;
            cmd.trigger(
                ToastEvent::warning(format!("失败: {:?}", e)).with_position(ToastPosition::TopLeft),
            );
            continue;
        }

        let favorites_res = &mut weav3r_fav_res.0;
        favorites_res.set_new_profit(responses.items);

        let trader_card_data = favorites_res
            .user_profit_result
            .iter()
            .cloned()
            .map(|f| profit_to_trader_card_data(f, setting_config.product_top_time))
            .collect::<Vec<_>>();

        let parents = *cards_parents;

        let has_changes = handle_trader_card_update(
            &mut cmd,
            parents,
            &mut current_cards,
            trader_card_data.clone(),
        );

        if has_changes && setting_config.audio_switch {
            cmd.spawn(AudioPlayer::new(audio_assets.notification.clone()));
        }
    }
}

fn profit_to_trader_card_data(
    favorites_res: ProfitUserInfo,
    product_top_time: u32,
) -> TraderCardData {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let recent_sec = now.saturating_sub(product_top_time as u64);
    let is_new = favorites_res.created_on >= recent_sec;

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
        is_new,
        items,
    }
}

// 初始化weav3r_fav_res官方数据
fn init_weav3r_fav_res(
    mut cmd: Commands,
    items_database: Res<OfficeItemsDbRes>,
    setting_config: Res<SettingConfigRes>,
) {
    let mut weav3r_fav_res = Weav3rFavRes::default();
    update_weav3r_fav_res(setting_config, &mut weav3r_fav_res);
    let favorites_res = &mut weav3r_fav_res.0;
    favorites_res.filter.office_item_map = items_database
        .items
        .iter()
        .map(|item| (item.id, item.clone()))
        .collect();
    cmd.insert_resource(weav3r_fav_res);
}

fn update_weav3r_fav_res_on_change(
    setting_config: Res<SettingConfigRes>,
    mut weav3r_fav_res: ResMut<Weav3rFavRes>,
) {
    update_weav3r_fav_res(setting_config, &mut weav3r_fav_res);
}

// 更新weav3r_fav_res数据
fn update_weav3r_fav_res(setting_config: Res<SettingConfigRes>, data: &mut Weav3rFavRes) {
    data.0.sort.recent_sec = setting_config.product_top_time;
    data.0.filter.min_profit = setting_config.min_profit;
    data.0.filter.office_sell_price = setting_config.office_price_start;
    data.0.filter.office_sell_profit = setting_config.office_profit;
    data.0.filter.min_profit_percentage = setting_config.profit_percent;
    data.0.filter.target_ids = setting_config
        .target_ids
        .iter()
        .cloned()
        .collect();
}

fn update_ticker(
    mut cmd: Commands,
    time: Res<Time>,
    mut ticker: ResMut<Weav3rUpdTickerComp>,
    weav3r_req_sys_resource: Res<Weav3rSysResource>,
    setting_config: Res<SettingConfigRes>,
    items_database: Res<OfficeItemsDbRes>,
) {
    if !setting_config.is_run {
        return;
    }
    let state = ticker.tick(time.delta_secs());
    if CountDownState::Restore.ne(&state) {
        return;
    }

    trigger_weav3r_request(
        &mut cmd,
        &items_database,
        &setting_config,
        &weav3r_req_sys_resource,
    );
}

fn startup_trigger_weav3r_request(
    mut cmd: Commands,
    items_database: Res<OfficeItemsDbRes>,
    setting_config: Res<SettingConfigRes>,
    weav3r_req_sys_resource: Res<Weav3rSysResource>,
) {
    if !setting_config.is_run {
        return;
    }
    trigger_weav3r_request(
        &mut cmd,
        &items_database,
        &setting_config,
        &weav3r_req_sys_resource,
    );
}

fn trigger_weav3r_request(
    cmd: &mut Commands,
    items_database: &OfficeItemsDbRes,
    setting_config: &SettingConfigRes,
    weav3r_req_sys_resource: &Weav3rSysResource,
) {
    let target_ids = items_database
        .items
        .iter()
        .filter(|x| x.tradeable && x.sell_price >= setting_config.office_price_start)
        .map(|x| x.id)
        .chain(setting_config.target_ids.iter().cloned())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let sys_id: bevy_ecs::system::SystemId<In<(String, String, String)>> =
        weav3r_req_sys_resource.0;
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
