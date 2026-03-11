use bevy::prelude::*;
use bevy::{ecs::relationship::RelatedSpawner, ui::Checked};
use bevy_clipboard::{ClipboardReadResult, ReadClipboardEvent};
use bevy_feathers::controls::toggle_switch;
use bevy_http::tools::HttpTool;
use bevy_storage::StorageManager;
use bevy_tab::tab::{TabContentRoot, build_tab_view};
use bevy_theme::prelude::*;
use bevy_ui_widgets::{ValueChange, checkbox_self_update, observe};

use crate::{
    components::{
        button_click_effect::ButtonClickEffect,
        number_stepper::{NumberStepperConfig, NumberStepperSpawner, StepperValueChanged},
        scroll::ScrollSpawn,
        tick::{CountDownComp, CountDownType},
    },
    game::GameState,
    view::{TabId, res::SettingConfigRes},
};

const INTERVAL_KEY: &str = "interval";
const PRODUCT_TOP_TIME_KEY: &str = "product_top_time";
const PROFIT_PERCENT_KEY: &str = "profit_percent";
const MIN_PROFIT_KEY: &str = "min_profit";
const OFFICE_PRICE_START_KEY: &str = "office_price_start";
const OFFICE_PROFIT_KEY: &str = "office_profit";

#[derive(Event)]
struct SaveConfigEvent;

pub struct SettingPlugin;

impl Plugin for SettingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(num_change_ob)
            .add_observer(parse_curl_result_ob)
            .add_observer(save_config_observer)
            .insert_resource(Weav3rUpdTickerComp::new(5.0).set_type(CountDownType::Repeat))
            .add_systems(Startup, load_config_system)
            .add_systems(OnEnter(GameState::Menu), view.after(build_tab_view))
            .add_systems(OnEnter(GameState::Menu), init_switch_states.after(view))
            .add_systems(
                Update,
                update_curl_config_system.run_if(in_state(GameState::Menu)),
            )
            .add_systems(Update, update_tick.run_if(in_state(GameState::Menu)))
            .add_systems(Update, read_clipboard_sys.run_if(in_state(GameState::Menu)));
    }
}

fn build_setting_ui(
    commands: &mut Commands,
    view_entity: Entity,
    setting_config: &SettingConfigRes,
) {
    let interval = setting_config.interval;
    let product_top_time = setting_config.product_top_time; 
    let profit_percent = setting_config.profit_percent;
    let min_profit = setting_config.min_profit as f32;
    let office_price_start = setting_config.office_price_start as f32;
    let office_profit = setting_config.office_profit as f32;
    let target_ids = setting_config
        .target_ids
        .split(",")
        .collect::<Vec<&str>>()
        .join(",");
    let token = setting_config.token.clone();
    let cookie = setting_config.cookie.clone();

    let view_bundle = (
        Node {
            width: percent(100.0),
            height: percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            column_gap: Val::Px(20.0),
            ..Default::default()
        },
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            parent.spawn(row_text_switch(
                "启动".to_string(),
                (
                    toggle_switch(()),
                    StartSwitchComp,
                    observe(checkbox_self_update),
                    observe(update_start_switch_btn),
                ),
            ));

            parent.spawn(row_text_switch(
                "音频".to_string(),
                (
                    toggle_switch(()),
                    AudioSwitchComp,
                    observe(checkbox_self_update),
                    observe(update_audio_switch_btn),
                ),
            ));

            parent.spawn(row_text_number_step(
                "W3v 刷新间隔时间".to_string(),
                NumberStepperConfig {
                    id: Some(INTERVAL_KEY.to_string()),
                    value: interval,
                    min: 3.,
                    max: 100.0,
                    step: 0.5,
                    decimal_places: 1,
                    unit: Some("秒".to_string()),
                },
            ));

            parent.spawn(row_text_number_step(
                "新商品置顶时间".to_string(),
                NumberStepperConfig {
                    id: Some(PRODUCT_TOP_TIME_KEY.to_string()),
                    value: product_top_time as f32,
                    min: 0.,
                    max: 200.0,
                    step: 1.0,
                    decimal_places: 1,
                    unit: Some("%".to_string()),
                },
            ));

            parent.spawn(row_text_number_step(
                "利润百分比".to_string(),
                NumberStepperConfig {
                    id: Some(PROFIT_PERCENT_KEY.to_string()),
                    value: profit_percent,
                    min: 0.,
                    max: 100.0,
                    step: 0.5,
                    decimal_places: 1,
                    unit: Some("%".to_string()),
                },
            ));
            parent.spawn(row_text_number_step(
                "最低利润".to_string(),
                NumberStepperConfig {
                    id: Some(MIN_PROFIT_KEY.to_string()),
                    value: min_profit,
                    min: 0.,
                    max: 99999999.,
                    step: 1000.0,
                    decimal_places: 0,
                    unit: Some("$".to_string()),
                },
            ));
            parent.spawn(row_text_number_step(
                "官方回收最低价".to_string(),
                NumberStepperConfig {
                    id: Some(OFFICE_PRICE_START_KEY.to_string()),
                    value: office_price_start,
                    min: 0.,
                    max: 99999999.0,
                    step: 100.0,
                    decimal_places: 0,
                    unit: Some("$".to_string()),
                },
            ));
            parent.spawn(row_text_number_step(
                "官方回收利润".to_string(),
                NumberStepperConfig {
                    id: Some(OFFICE_PROFIT_KEY.to_string()),
                    value: office_profit,
                    min: 0.,
                    max: 99999999.0,
                    step: 1000.0,
                    decimal_places: 0,
                    unit: Some("$".to_string()),
                },
            ));

            parent.spawn(col_text_text(
                "目标ID".to_string(),
                target_ids,
                TargetIdsTextComp,
            ));
            parent.spawn(col_text_text("Token".to_string(), token, TokenTextComp));
            parent.spawn(col_text_text("Cookie".to_string(), cookie, CookieTextComp));

            parent.spawn((btn_bundle("Parse Curl".to_string()), ParseCurlBtn));
        })),
    );

    let scroll_spawn = ScrollSpawn {
        width: percent(100.0),
        height: percent(100.0),
        background: Some(ThemedBackground::primary()),
        ..Default::default()
    };
    commands.spawn((ChildOf(view_entity), scroll_spawn.bundle(vec![view_bundle])));
}

fn view(
    mut commands: Commands,
    content_query: Query<(Entity, &TabContentRoot)>,
    setting_config: Res<SettingConfigRes>,
) {
    let mut view_entity = None;

    for (entity, root) in &content_query {
        if root.id.as_str() == TabId::Setting.name() {
            view_entity = Some(entity);
            break;
        }
    }

    let Some(view_entity) = view_entity else {
        return;
    };

    build_setting_ui(&mut commands, view_entity, &setting_config);
}
#[derive(Component)]
struct ParseCurlBtn;

#[derive(Component)]
struct TokenTextComp;

#[derive(Component)]
struct CookieTextComp;

#[derive(Component)]
struct TargetIdsTextComp;

#[derive(Component)]
struct StartSwitchComp;

#[derive(Component)]
struct AudioSwitchComp;

// 按钮组件
fn btn_bundle(text: String) -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(10.0), Val::Px(10.0)),
            border_radius: BorderRadius::all(px(4.0)),
            ..Default::default()
        },
        BackgroundColor(Color::Srgba(Srgba::GREEN)),
        ThemedBackground::secondary(),
        Button,
        ButtonClickEffect::default(),
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn((
                Text::new(text),
                TextLayout::new_with_justify(Justify::Center).with_linebreak(LineBreak::NoWrap),
            ));
        })),
    )
}

// 两列文本组件
fn col_text_text(text: String, value: String, comp: impl Component) -> impl Bundle {
    (
        Node {
            width: percent(100.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::FlexStart,
            border_radius: BorderRadius::all(px(8.0)),
            padding: UiRect::all(px(12.0)),
            row_gap: Val::Px(4.0),
            ..Default::default()
        },
        ThemedBackground::secondary(),
        BackgroundColor(Color::srgb(0.15, 0.15, 0.19)),
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn(Text::new(text));
            spawner.spawn((
                Node {
                    width: percent(100.0),
                    height: Val::Auto,
                    border_radius: BorderRadius::all(px(8.0)),
                    padding: UiRect::all(px(4.0)),
                    ..Default::default()
                },
                ThemedBackground::tertiary(),
                BackgroundColor(Color::Srgba(Srgba::gray(0.2))),
                Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
                    spawner.spawn((
                        Text::new(value),
                        comp,
                        TextLayout::new_with_justify(Justify::Left)
                            .with_linebreak(LineBreak::AnyCharacter),
                    ));
                })),
            ));
        })),
    )
}

// 一行文本和数值步进器组件
fn row_text_number_step(text: String, config: NumberStepperConfig) -> impl Bundle {
    row_bundle(
        (Text::new(text), ThemedText::primary()),
        NumberStepperSpawner::new(config).bundle(),
    )
}

// 一行文本和切换按钮组件
fn row_text_switch(text: String, switch_bundle: impl Bundle) -> impl Bundle {
    row_bundle(
        (Text::new(text), ThemedText::primary()),
        (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                column_gap: px(8),
                ..default()
            },
            ThemedBackground::secondary(),
            children![switch_bundle],
        ),
    )
}

// 一行组件
fn row_bundle(left: impl Bundle, right: impl Bundle) -> impl Bundle {
    (
        Node {
            width: percent(100.0),
            height: Val::Px(50.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(px(8.0)),
            border_radius: BorderRadius::all(px(4.0)),
            ..Default::default()
        },
        ThemedBackground::secondary(),
        BackgroundColor(Color::Srgba(Srgba::GREEN)),
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn(left);
            spawner.spawn(right);
        })),
    )
}

// 读取剪贴板系统
fn read_clipboard_sys(
    btn: Single<&Interaction, (Changed<Interaction>, With<Button>, With<ParseCurlBtn>)>,
    mut commands: Commands,
) {
    if btn.ne(&Interaction::Pressed) {
        return;
    }

    // 通过 Commands 触发剪贴板读取事件
    commands.trigger(ReadClipboardEvent);
}

/// 处理剪贴板读取结果并解析 curl
fn parse_curl_result_ob(
    trigger: On<ClipboardReadResult>,
    mut commands: Commands,
    mut setting_config: ResMut<SettingConfigRes>,
) {
    let Some(content) = &trigger.content else {
        if let Some(error) = &trigger.error {
            bevy::log::error!("读取剪贴板失败: {}", error);
        }
        return;
    };

    // 解析 curl 命令
    match HttpTool::from_curl(content) {
        Ok(http_tool) => {
            setting_config.token = http_tool
                .headers
                .get("next-action")
                .cloned()
                .unwrap_or_default()
                .to_string();
            // Cookie header 可能是大写或小写,尝试两种方式
            setting_config.cookie = http_tool
                .headers
                .get("Cookie")
                .cloned()
                .unwrap_or_default()
                .to_string();
            setting_config.target_ids = String::from_utf8(http_tool.body.clone())
                .unwrap_or_default()
                .replace("[", "")
                .replace("]", "");
            commands.trigger(SaveConfigEvent);
        }
        Err(e) => {
            bevy::log::error!("解析 curl 失败: {}", e);
        }
    }
}

// 数值变化监听
fn num_change_ob(
    event: On<StepperValueChanged>,
    mut commands: Commands,
    mut setting_config: ResMut<SettingConfigRes>,
) {
    // 通过 id 字段区分不同的 stepper
    if let Some(id) = &event.id {
        match id.as_str() {
            INTERVAL_KEY => {
                setting_config.interval = event.new_value;
            }
            PRODUCT_TOP_TIME_KEY => {
                setting_config.product_top_time = event.new_value as u32;
            }
            PROFIT_PERCENT_KEY => {
                setting_config.profit_percent = event.new_value;
            }
            MIN_PROFIT_KEY => {
                setting_config.min_profit = event.new_value as i64;
            }
            OFFICE_PRICE_START_KEY => {
                setting_config.office_price_start = event.new_value as u64;
            }
            OFFICE_PROFIT_KEY => {
                setting_config.office_profit = event.new_value as u64;
            }
            _ => {
                bevy::log::warn!("Unknown setting key: {}", id);
                return;
            }
        }
    }
    commands.trigger(SaveConfigEvent);
}

// 更新curl文本显示
fn update_curl_config_system(
    setting_config: Res<SettingConfigRes>,
    mut text_queries: ParamSet<(
        Single<&mut Text, With<TargetIdsTextComp>>,
        Single<&mut Text, With<TokenTextComp>>,
        Single<&mut Text, With<CookieTextComp>>,
    )>,
) {
    text_queries.p0().0 = setting_config
        .target_ids
        .split(",")
        .collect::<Vec<&str>>()
        .join(",");
    text_queries.p1().0 = setting_config.token.clone();
    text_queries.p2().0 = setting_config.cookie.clone();
}

/// 处理保存配置事件
fn save_config_observer(
    _trigger: On<SaveConfigEvent>,
    storage: Res<StorageManager>,
    config: Res<SettingConfigRes>,
) {
    match storage.save_app_config(config.as_ref()) {
        Ok(_) => {
            bevy::log::info!("✅ Configuration saved successfully");
        }
        Err(e) => {
            bevy::log::error!("❌ Failed to save configuration: {}", e);
        }
    }
}

/// 启动时加载配置
pub fn load_config_system(
    storage: Res<StorageManager>,
    mut commands: Commands,
    mut ticker: ResMut<Weav3rUpdTickerComp>,
) {
    match storage.load_app_config_or_default::<SettingConfigRes>() {
        Ok(loaded_config) => {
            ticker.set_target(loaded_config.interval);
            ticker.backup();
            commands.insert_resource(loaded_config);
        }
        Err(e) => {
            bevy::log::error!("❌ Failed to load configuration: {}, using defaults", e);
        }
    }
}

#[derive(Resource)]
pub struct UpdTickerRes;

pub type Weav3rUpdTickerComp = CountDownComp<UpdTickerRes>;

// 更新定时器
fn update_tick(
    mut commands: Commands,
    mut ticker: ResMut<Weav3rUpdTickerComp>,
    setting_config: Res<SettingConfigRes>,
) {
    if ticker.target.eq(&setting_config.interval) {
        return;
    }
    ticker.restore();
    ticker.set_target(setting_config.interval);
    ticker.backup();
    commands.trigger(SaveConfigEvent);
}

fn update_audio_switch_btn(
    value_change: On<ValueChange<bool>>,
    mut commands: Commands,
    mut setting_config: ResMut<SettingConfigRes>,
) {
    setting_config.audio_switch = value_change.value;
    commands.trigger(SaveConfigEvent);
}

fn update_start_switch_btn(
    value_change: On<ValueChange<bool>>,
    mut commands: Commands,
    mut setting_config: ResMut<SettingConfigRes>,
) {
    setting_config.is_run = value_change.value;
    commands.trigger(SaveConfigEvent);
}

fn init_switch_states(
    mut commands: Commands,
    setting_config: Res<SettingConfigRes>,
    start_switch_query: Query<Entity, With<StartSwitchComp>>,
    audio_switch_query: Query<Entity, With<AudioSwitchComp>>,
) {
    for entity in start_switch_query.iter() {
        if setting_config.is_run {
            commands.entity(entity).insert(Checked);
        }
    }
    for entity in audio_switch_query.iter() {
        if setting_config.audio_switch {
            commands.entity(entity).insert(Checked);
        }
    }
}
