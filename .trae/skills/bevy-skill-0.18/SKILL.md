---
name: bevy-skill-0.18
description: Bevy 0.18 插件开发与架构设计指南。提供 ECS 组件设计、系统组织、插件系统、多 crate 架构等最佳实践。
---

# Bevy 0.18 插件开发与架构指南

本 Skill 提供 Bevy 0.18 插件开发与架构设计的最佳实践指导。

## 目录

- [1. 核心概念](#1-核心概念)
  - [1.1 ECS 基础](#11-ecs-基础)
  - [1.2 Event vs Message](#12-event-vs-message)
  - [1.3 插件系统](#13-插件系统)
- [2. 插件开发模式](#2-插件开发模式)
  - [2.1 单 Crate 多插件](#21-单-crate-多插件)
  - [2.2 多 Crate 架构](#22-多-crate-架构)
- [3. 生命周期与调度](#3-生命周期与调度)
  - [3.1 App 生命周期](#31-app-生命周期)
  - [3.2 帧调度阶段](#32-帧调度阶段)
  - [3.3 插件生命周期](#33-插件生命周期)
  - [3.4 组件生命周期](#34-组件生命周期)
- [4. 最佳实践](#4-最佳实践)
  - [4.1 组件设计](#41-组件设计)
  - [4.2 系统组织](#42-系统组织)
  - [4.3 状态管理](#43-状态管理)
  - [4.4 性能优化](#44-性能优化)
- [5. 参考资料](#5-参考资料)

---

## 1. 核心概念

### 1.1 ECS 基础

Bevy 使用 Entity Component System (ECS) 架构：

- **Entity**: 唯一 ID，用于附加组件
- **Component**: 数据结构，派生 `#[derive(Component)]`
- **System**: 逻辑函数，通过参数化查询访问数据
- **Resource**: 全局单例状态，派生 `#[derive(Resource)]`
- **State**: 应用状态机，用于管理游戏状态

```rust
#[derive(Component)]
struct Player {
    health: f32,
    speed: f32,
}

#[derive(Resource)]
struct GameConfig {
    difficulty: f32,
}

fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in &mut query {
        if keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += player.speed * time.delta_secs();
        }
    }
}
```

### 1.2 Event vs Message

Bevy 0.18 将消息系统拆分为两个独立概念：

#### Event（事件）

用于「触发 + 观察」模式的即时响应：

- 派生 `#[derive(Event)]`
- 通过 `trigger()` 立即触发
- 使用观察者模式 `On<E>` 响应
- 支持实体绑定 `EntityEvent`
- 支持层级传播 `#[entity_event(propagate)]`

```rust
#[derive(Event)]
struct GameOverEvent {
    final_score: u32,
}

#[derive(EntityEvent, Debug)]
#[entity_event(propagate)]
struct ButtonClickEvent {
    #[event_target]
    entity: Entity,
}

// 触发
commands.trigger(GameOverEvent { final_score: 100 });

// 观察
fn handle_game_over(mut event: On<GameOverEvent>) {
    info!("游戏结束！得分：{}", event.final_score);
}
```

#### Message（消息）

用于「队列 + 缓冲」模式的帧间通信：

- 派生 `#[derive(Message)]`
- 通过 `MessageWriter` 写入队列
- 通过 `MessageReader` 延迟读取
- 默认留存 2 帧后自动清理
- 每个读者维护独立游标

```rust
#[derive(Message)]
struct PlayerDamageMessage {
    player_id: Entity,
    damage: f32,
}

fn produce_damage(mut writer: MessageWriter<PlayerDamageMessage>) {
    writer.send(PlayerDamageMessage {
        player_id: player_entity,
        damage: 10.0,
    });
}

fn consume_damage(mut reader: MessageReader<PlayerDamageMessage>) {
    for msg in reader.read() {
        // 处理消息
    }
}
```

#### 选型原则

| 场景 | 推荐 |
|------|------|
| 按钮点击、碰撞检测、即时应答 | Event |
| 批量数据、跨帧处理、网络消息队列 | Message |

> **关键理解**：`Event` trait 继承自 `Message` trait，所有事件都是消息。

详见 [deepwiki: message-vs-events](https://deepwiki.com/bevyengine/bevy/2.6-change-detection-and-events#message-vs-events)

### 1.3 插件系统

#### Plugin Trait

```rust
pub trait Plugin: Any + Send + Sync {
    fn build(&self, app: &mut App);
    fn ready(&self, app: &App) -> bool { true }
    fn finish(&self, app: &mut App) {}
    fn cleanup(&self, app: &mut App) {}
}
```

#### Plugin Groups

- **DefaultPlugins**: 完整功能集（渲染、窗口、输入、音频等）
- **MinimalPlugins**: 仅核心功能（TaskPool、Time、ScheduleRunner）

#### 执行顺序

来自 Bevy 官方 `crates/bevy_internal/src/default_plugins.rs`：

1. **Core**: PanicHandler → Log → TaskPool → Time → Diagnostics
2. **Platform**: Input → Window → Winit
3. **Assets**: WebAsset → Asset
4. **Rendering**: Render → Image → CorePipeline → PBR → Sprite → UI
5. **Optional**: Audio, Animation, Gizmo, Picking

---

## 2. 插件开发模式

### 2.1 单 Crate 多插件

在单个 crate 内拆分多个插件：

```rust
pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, game_update)
           .add_event::<GameEvent>()
           .init_resource::<GameState>();
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu);
    }
}

// 使用
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(GameLogicPlugin)
    .add_plugins(UiPlugin)
    .run();
```

### 2.2 多 Crate 架构

#### Workspace 结构

```
项目/
├── Cargo.toml              # workspace root
├── src/
│   └── main.rs
├── crates/
│   ├── game-core/          # 核心逻辑
│   ├── game-ui/            # UI 插件
│   └── game-assets/        # 资源加载
```

#### 依赖原则

**不要直接依赖整个 `bevy` crate**，按需选择性依赖：

| 需求 | 依赖 |
|------|------|
| 仅 ECS 核心 | `bevy_ecs` |
| 插件系统 | `bevy_app` |
| 资源加载 | `bevy_asset` |
| 窗口管理 | `bevy_window`, `bevy_winit` |
| 渲染功能 | `bevy_render`, `bevy_pbr`, `bevy_sprite` |
| UI 系统 | `bevy_ui` |
| 输入处理 | `bevy_input` |
| 完整引擎 | `bevy` |

```toml
# game-core/Cargo.toml
[dependencies]
bevy_ecs = "0.18"
bevy_app = "0.18"

# game-ui/Cargo.toml
[dependencies]
bevy_ecs = "0.18"
bevy_app = "0.18"
bevy_ui = "0.18"
```

#### 拆分时机

- 需要独立发布到 crates.io
- 功能边界清晰，可独立测试
- 团队分工明确，减少耦合

---

## 3. 生命周期与调度

### 3.1 App 生命周期

```
Startup → Run (多帧循环) → Cleanup
```

- **Startup**: 应用初始化，系统只执行一次
- **Run**: 主循环，每帧执行
- **Cleanup**: 应用退出时的清理

### 3.2 帧调度阶段

默认 `Update` schedule 内的执行顺序：

```
First → PreUpdate → Update → PostUpdate → Last
```

| 阶段 | 用途 |
|------|------|
| First | 消息清理、输入处理 |
| PreUpdate | 物理、动画前置 |
| Update | 游戏逻辑主阶段 |
| PostUpdate | 渲染同步 |
| Last | UI、清理 |

### 3.3 插件生命周期

| 方法 | 调用时机 | 用途 |
|------|----------|------|
| `build()` | 插件添加时 | 注册系统、资源、事件 |
| `ready()` | 每帧检查 | 等待依赖完成 |
| `finish()` | App 运行前 | 配置最终化 |
| `cleanup()` | App 退出时 | 清理资源 |

### 3.4 组件生命周期

#### 变化检测

基于 tick（帧计数器）的变化追踪：

- `Added<T>`: 组件刚被添加到实体
- `Changed<T>`: 组件在本帧被修改

```rust
fn init_new_players(query: Query<&Transform, Added<Player>>) {
    for transform in &query {
        // 新添加的玩家初始化
    }
}

fn update_moving_players(query: Query<&Transform, Changed<Transform>>) {
    // 处理移动
}
```

#### Tick 系统

- 每个组件存储 `added` 和 `changed` tick
- 系统记录 `last_run` tick
- 比较判断组件是否变化

---

## 4. 最佳实践

### 4.1 组件设计

- 组件应该是纯数据，不包含逻辑
- 使用元组组件组合多个数据
- 避免频繁创建/销毁组件

```rust
// Good
#[derive(Component)]
struct Player {
    health: f32,
    speed: f32,
}

// Good: 组合
commands.spawn((
    Player { health: 100.0, speed: 5.0 },
    Transform::default(),
));
```

### 4.2 系统组织

- 分离初始化 (`Startup`) 和更新 (`Update`)
- 使用状态条件 `run_if(in_state(...))`
- 按功能模块组织系统组

```rust
app.add_systems(Update, (
    input_system,
    physics_system,
    game_logic_system,
).chain());

app.add_systems(Update, render_system.run_if(in_state(GameState::Playing)));
```

### 4.3 状态管理

```rust
#[deriveStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}
```

### 4.4 性能优化

- 使用 `With<T>` / `Without<T>` 过滤
- 使用 `Changed<T>` 只处理变化的实体
- 批量操作 `query.iter_mut()`
- 避免每帧创建资源

```rust
// Good: 变化检测
fn update_changed(query: Query<&Transform, Changed<Transform>>) {}

// Good: 过滤
fn update_players(query: Query<&Transform, With<Player>>) {}

// Bad: 频繁创建
fn bad_system(mut commands: Commands) {
    commands.spawn(Enemy::new()); // 每帧创建
}
```

---

## 5. 参考资料

- [DeepWiki Bevy Architecture](https://deepwiki.com/bevyengine/bevy)
- [Bevy GitHub](https://github.com/bevyengine/bevy)
- [Bevy 官方文档](https://bevyengine.org/learn/book/getting-started/)