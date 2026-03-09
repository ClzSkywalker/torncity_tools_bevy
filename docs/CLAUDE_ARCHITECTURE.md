# 项目架构详细说明

> 这是本项目特有的架构设计。通用 ECS 概念见 [ai/bevy_ecs_guide.md](ai/bevy_ecs_guide.md)

本文档说明 Torn Trade 项目的核心架构设计。

## GameState 状态系统

项目使用 Bevy 状态系统管理应用生命周期。

### 状态定义

```rust
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Asset,      // 资源加载阶段
    InitConfig, // 初始化配置阶段
    Menu,       // 主菜单/运行阶段
}
```

### 状态流转

```
启动应用 → [Asset] → [InitConfig] → [Menu]
```

### 各阶段职责

**Asset 状态**
- 使用 `bevy_asset_loader` 加载静态资源
- 加载字体、图标、CSV 数据
- 自动流转到 `InitConfig`

**InitConfig 状态**
- 初始化 `OfficeItemsDbRes` 和 `SettingConfigRes`
- 条件满足后流转到 `Menu`

**Menu 状态**
- 显示主界面 UI
- 运行核心业务逻辑（HTTP 请求、数据处理、UI 更新）

### 状态使用模式

**1. 进入状态时执行**
```rust
app.add_systems(OnEnter(GameState::Menu), setup_ui);
```

**2. 条件运行系统**
```rust
app.add_systems(Update, my_system.run_if(in_state(GameState::Menu)));
```

**3. 状态流转**
```rust
fn transition_to_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Menu);
}
```

**4. 资源变化触发**
```rust
app.add_systems(
    Update,
    update_config.run_if(
        resource_changed::<SettingConfigRes>
            .and_then(in_state(GameState::Menu))
    )
);
```

## 插件系统

### 核心插件

**GamePlugin** (`src/game.rs`)
- 管理游戏状态
- 注册 LoadingPlugin 和 ViewPlugin

**ViewPlugin** (`src/view/mod.rs`)
- 管理 UI 和相机
- 注册所有子插件

**ComponentsPlugin** (`src/components/mod.rs`)
- 管理可复用 UI 组件

### 自定义插件

**Weav3rHomePlugin** (`src/view/home.rs`)
- Home 页面，处理 weav3r 数据和 UI

**SettingPlugin** (`src/view/setting.rs`)
- 设置页面，处理用户配置

**LoadingPlugin** (`src/resource/mod.rs`)
- 资源加载插件

使用 `bevy_theme` 插件，详见 [crates/bevy_theme/README.md](crates/bevy_theme/README.md)

## 数据流

```
HTTP Request (ehttp)
    ↓
HTTP Response (Event)
    ↓
Data Processing (System)
    ↓
Resource Update
    ↓
UI Update (Query)
```

### HTTP 请求发送

**实现位置**: `src/view/home.rs` - `handle_weav3r_send_req_btn`

```rust
fn handle_weav3r_send_req_btn(
    mut commands: Commands,
    query: Query<&Interaction, (Changed<Interaction>, With<Weav3rSendReqBtn>)>,
    setting_config: Res<SettingConfigRes>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            commands.trigger(Weav3rReqEvent::new(setting_config.api_key.clone()));
        }
    }
}
```

### HTTP 响应处理

**实现位置**: `src/view/home.rs` - `handle_weav3r_resp`

```rust
fn handle_weav3r_resp(
    mut commands: Commands,
    mut events: EventReader<Weav3rRespEvent>,
    mut weav3r_fav_res: ResMut<Weav3rFavRes>,
    items_database: Res<OfficeItemsDbRes>,
) {
    for event in events.read() {
        match &event.resp {
            Ok(data) => {
                weav3r_fav_res.update(data.clone(), &items_database);
            }
            Err(e) => {
                bevy::log::error!("Failed to fetch weav3r data: {:?}", e);
            }
        }
    }
}
```

### UI 更新

**实现位置**: `src/components/trader_card.rs` - `update_trader_card`

```rust
fn update_trader_card(
    mut query: Query<&mut TraderCardData>,
    weav3r_fav_res: Res<Weav3rFavRes>,
) {
    for mut card_data in &mut query {
        if let Some(item) = weav3r_fav_res.get_item(&card_data.id) {
            card_data.update_from_item(item);
        }
    }
}
```

## UI 组件层次

```
Camera2d
    └── TabContentRoot
        ├── TabBar
        │   ├── TabItem (Home)
        │   └── TabItem (Setting)
        └── TabContent
            ├── HomeTabContent
            │   ├── Button (刷新按钮)
            │   ├── ScrollContainer
            │   │   └── TraderCard * N
            │   └── CountDown
            └── SettingTabContent
                ├── Input (API Key)
                ├── Button (保存)
                └── Text (提示)
```

## 核心资源

### OfficeItemsDbRes

**位置**: `src/resource/items_data.rs`

```rust
#[derive(Resource, Clone, Debug)]
pub struct OfficeItemsDbRes {
    pub items: HashMap<String, ItemInfo>,
}
```

### SettingConfigRes

**位置**: `src/view/res.rs`

```rust
#[derive(Resource, Clone, Debug, Default)]
pub struct SettingConfigRes {
    pub api_key: String,
    pub refresh_interval: u64,
}
```

### Weav3rFavRes

**位置**: `src/view/res.rs`

```rust
#[derive(Resource, Clone, Debug, Default)]
pub struct Weav3rFavRes {
    pub favorites_data: FavoritesData,
    pub last_update: Option<DateTime<Utc>>,
}
```

## 事件系统

### 事件定义

**Weav3rReqEvent** - 触发 HTTP 请求
**Weav3rRespEvent** - 传递 HTTP 响应

### 事件使用

```rust
// 发送事件
commands.trigger(Weav3rReqEvent::new(api_key));

// 读取事件
fn handle_event(mut events: EventReader<Weav3rRespEvent>) {
    for event in events.read() {
        // 处理事件
    }
}
```

## 架构优势

1. **模块化** - 每个插件职责单一
2. **可扩展** - 添加新功能只需创建新插件
3. **性能优化** - 状态系统避免不必要的计算
4. **类型安全** - Rust 类型系统确保代码安全
5. **事件驱动** - 异步操作通过事件系统处理
