use bevy::ecs::relationship::RelatedSpawner;
use bevy::prelude::*;
use bevy_clipboard::{ClipboardReadResult, ReadClipboardEvent};
use bevy_http::tools::HttpTool;
use bevy_storage::{StorageError, StorageManager};
use bevy_tab::tab::{TabContentRoot, build_tab_view};

use crate::{
    components::{
        button_click_effect::ButtonClickEffect,
        number_stepper::{NumberStepperConfig, NumberStepperSpawner, StepperValueChanged},
        scroll::ScrollSpawn,
    },
    view::{TabId, res::SettingConfigRes},
};

const INTERVAL_KEY: &str = "interval";
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
            .add_systems(Startup, load_config_system.before(view))
            .add_systems(Startup, view.after(build_tab_view))
            .add_systems(
                Update,
                update_curl_config_system.run_if(resource_changed::<SettingConfigRes>),
            )
            .add_systems(Update, read_clipboard_sys)
            .add_systems(Update, save_config_button_system);
    }
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
        bevy::log::error!("setting_root not found");
        return;
    };

    let interval = setting_config.interval;
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

    let a = (
        Node {
            width: percent(100.0),
            height: percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(10.0),
            ..Default::default()
        },
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            // todo 开关按钮

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
            parent.spawn((btn_bundle("Save Config".to_string()), SaveConfigBtn));
        })),
    );

    let scroll_spawn = ScrollSpawn {
        width: percent(100.0),
        height: percent(100.0),
        ..Default::default()
    };
    commands.spawn((ChildOf(view_entity), scroll_spawn.bundle(vec![a])));
}

#[derive(Component)]
struct ParseCurlBtn;

#[derive(Component)]
struct SaveConfigBtn;

#[derive(Component)]
struct TokenTextComp;

#[derive(Component)]
struct CookieTextComp;

#[derive(Component)]
struct TargetIdsTextComp;

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

fn row_text_number_step(text: String, config: NumberStepperConfig) -> impl Bundle {
    row_bundle(Text::new(text), NumberStepperSpawner::new(config).bundle())
}

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
        BackgroundColor(Color::srgb(0.15, 0.15, 0.19)),
        Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
            spawner.spawn(left);
            spawner.spawn(right);
        })),
    )
}

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
        }
        Err(e) => {
            bevy::log::error!("解析 curl 失败: {}", e);
        }
    }
}

fn num_change_ob(event: On<StepperValueChanged>, mut setting_config: ResMut<SettingConfigRes>) {
    // 通过 id 字段区分不同的 stepper
    if let Some(id) = &event.id {
        match id.as_str() {
            INTERVAL_KEY => {
                setting_config.interval = event.new_value;
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
            _ => {}
        }
    }
}

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

/// 监听 Save Config 按钮点击
fn save_config_button_system(
    btn: Single<&Interaction, (Changed<Interaction>, With<Button>, With<SaveConfigBtn>)>,
    mut commands: Commands,
) {
    if btn.ne(&Interaction::Pressed) {
        return;
    }

    // 触发保存配置事件
    commands.trigger(SaveConfigEvent);
}

/// 处理保存配置事件
fn save_config_observer(
    _trigger: On<SaveConfigEvent>,
    mut storage: ResMut<StorageManager>,
    config: Res<SettingConfigRes>,
) {
    match storage.save("settings", config.as_ref()) {
        Ok(_) => {
            bevy::log::info!("✅ Configuration saved successfully");
        }
        Err(e) => {
            bevy::log::error!("❌ Failed to save configuration: {}", e);
        }
    }
}

/// 启动时加载配置
fn load_config_system(storage: Res<StorageManager>, mut config: ResMut<SettingConfigRes>) {
    match storage.load::<SettingConfigRes>("settings") {
        Ok(loaded_config) => {
            *config = loaded_config;
        }
        Err(StorageError::KeyNotFound(_)) => {
            bevy::log::info!("⚠️  No saved configuration found, using defaults");
        }
        Err(StorageError::DeserializationError(_)) => {
            bevy::log::info!("⚠️  No saved configuration found, using defaults");
        }
        Err(e) => {
            bevy::log::error!("❌ Failed to load configuration: {}, using defaults", e);
        }
    }
}
