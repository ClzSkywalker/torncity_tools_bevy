# Bevy ECS 指南

Bevy ECS 核心概念和项目常用模式。

## 核心概念

- **Entity**: 实体 ID
- **Component**: 附加到实体的数据
- **System**: 处理组件逻辑的函数
- **Resource**: 全局状态
- **State**: 应用状态管理

## 系统注册

```rust
// 基本注册
app.add_systems(Update, my_system);

// 按状态运行
app.add_systems(Update, my_system.run_if(in_state(GameState::Menu)));

// 进入状态时执行
app.add_systems(OnEnter(GameState::Menu), setup_ui);

// 执行顺序
app.add_systems(Update, (system_a, system_b.after(system_a)));
```

## 查询

```rust
// 读写
Query<&mut Position>

// 过滤
Query<&Position, With<Player>>
Query<&Position, Without<Player>>
Query<&Position, Changed<Position>>
```

## 资源

```rust
// 读取
fn read(r: Res<Config>) {}

// 写入
fn write(mut r: ResMut<Config>) {}

// 注册
app.init_resource::<Config>();
app.insert_resource(MyConfig::default());
```

## 事件

1.  **`Event`**：专属「触发(trigger) + 观察(observe)」的即时响应机制
2.  **`Message`**：专属「缓冲队列 + 读写」的帧间通信机制，完全替代旧版本 `Event` 的缓冲读写能力

---

## Bevy 0.18 Event 与 Message 官方定义核心区别表
| 对比维度 | Event（Bevy 0.18 官方规范） | Message（Bevy 0.18 官方规范） |
| :--- | :--- | :--- |
| **官方核心定位** | 专指**被触发(triggered)和被观察(observed)**的类型，基于Observer系统的即时事件响应机制 | 专指**被缓冲(buffered)**的类型，基于全局队列的帧间消息读写机制，完全替代旧版本`Event`的缓冲能力 |
| **核心设计目标** | 解耦「事件触发」与「即时响应逻辑」，支持定向/全局的同步回调，适配ECS观察者模式 | 实现跨系统、跨帧的状态同步，支持延迟处理的批量消息读写，严格遵循帧循环系统调度顺序 |
| **核心Trait定义** | 必须实现`Event` trait，核心关联`Trigger`类型，静态定义触发行为与观察者调度规则<br>```rust<br>pub trait Event: 'static + Send + Sync {<br>    type Trigger<'a>: Trigger<Self>;<br>}<br>``` | 必须实现`Message` trait，仅定义消息的基础约束，无触发/传播相关关联类型<br>```rust<br>pub trait Message: 'static + Send + Sync {}<br>``` |
| **核心操作API** | 触发：`World::trigger()` / `Commands::trigger()`<br>监听：`World::add_observer()` / `EntityWorldMut::observe()`<br>回调入参：`On<E>` 类型（原`Trigger<E>`重命名） | 写入：`MessageWriter<M>` 系统参数<br>读取：`MessageReader<M>` 系统参数<br>无触发/监听API，仅支持队列的读写操作 |
| **执行时机** | **同步即时执行**：触发时立即在当前调用栈内调度所有匹配的观察者，回调执行完成后才返回，不依赖系统调度顺序 | **异步延迟执行**：写入队列后，仅当读取系统按调度顺序执行时才会被处理，完全遵循帧内系统的执行先后 |
| **生命周期** | 无持久化队列，事件数据仅在触发回调的执行周期内有效，回调完成后立即释放，**不会跨帧留存** | 存储在全局`Messages<M>`资源的双缓冲队列中，默认**支持跨帧留存**，直到被`MessageReader`手动消费，或按清理规则自动释放 |
| **通信模式** | 原生支持两种模式：<br>1. 全局广播（`GlobalTrigger`，无目标实体）<br>2. 实体定向触发（`EntityTrigger`，绑定目标实体） | 全局队列广播模式：所有能访问`MessageReader<M>`的系统，均可读取队列内的全部消息，无原生定向投递能力 |
| **实体绑定能力** | 原生支持：通过`EntityEvent`派生，可将事件与特定实体强绑定，仅触发该实体上注册的观察者 | 无原生支持：消息本身无目标属性，若需定向投递，需手动在消息体中添加目标字段，自行过滤处理 |
| **层级传播能力** | 原生支持：通过`#[entity_event(propagate)]`开启，可沿实体父子层级冒泡传播，支持`propagate(false)`中断传播 | 无任何传播机制：消息仅存在于全局队列，无层级传递、冒泡等能力 |
| **耦合度** | 极低：触发方完全无需感知监听方的存在，也无需关心响应逻辑，仅负责触发事件 | 中等：读写双方需约定消息类型，读取方需主动感知消息的写入时机与队列状态，无自动回调机制 |
| **顺序性保障** | 同一触发动作的观察者按注册顺序执行；多轮触发按调用顺序同步执行，无队列乱序风险 | 严格按**写入顺序**存入队列，读取时遵循FIFO先进先出规则，100%保障写入-读取的顺序一致性 |
| **清理机制** | 无队列清理逻辑：触发回调执行完成后，事件数据立即释放，无内存残留 | 双缓冲队列机制：默认帧末自动清理已读取的消息，未读取的消息可留存到下一帧；支持自定义清理策略 |
| **注册要求** | 无需手动注册到App：实现`Event` trait后，可直接触发与监听，仅需注册观察者 | 必须手动注册到App：通过`App::add_message::<M>()`初始化全局队列，否则无法使用读写API |
| **0.18 版本状态** | 核心稳定API，无架构性变更，仅优化了触发性能与观察者调度逻辑 | 核心稳定API，无架构性变更，仅优化了队列性能与内存管理，完全替代旧版本`EventReader/EventWriter` |
| **官方典型使用场景** | 1. 实体交互回调（按钮点击、碰撞触发）<br>2. 即时响应的全局事件（游戏暂停、玩家死亡）<br>3. 组件生命周期事件（`OnAdd`/`OnRemove`）<br>4. 需要沿实体层级传播的事件 | 1. 跨系统、跨帧的批量数据传递（输入事件汇总、伤害统计）<br>2. 延迟处理的逻辑（本帧产生的事件，下一帧统一处理）<br>3. 需按顺序批量消费的指令（网络消息解析、资源加载队列） |

---

### 官方拆分的核心背景（来自0.17官方公告原文）
旧版本`Event` trait同时承担了「可观察事件（Observer处理）」和「缓冲事件（EventReader/Writer处理）」两个完全不同的职责，导致三大核心问题：
1.  概念歧义、API易误用，使用时需要大量隐含上下文
2.  类型不安全，无法静态约束事件的使用场景（比如无法禁止对实体事件做无实体触发）
3.  不必要的性能开销与代码冗余，所有事件都要兼容所有使用场景

拆分后两个概念职责单一、类型安全、性能更优，官方明确说明：**可以为同一个类型同时实现两个trait，但预期这种场景会非常少见，建议按需选择单一职责**。

---

## Bevy 0.18 标准用法示例
### 1. Event 标准用法（触发-观察模式）
```rust
use bevy::prelude::*;

// 定义全局Event，默认使用GlobalTrigger
#[derive(Event, Debug)]
struct GameOverEvent {
    final_score: u32,
}

// 定义实体定向Event，派生EntityEvent，开启层级传播
#[derive(EntityEvent, Debug)]
#[entity_event(propagate)]
struct ButtonClickEvent {
    #[event_target] // 指定事件绑定的目标实体
    entity: Entity,
    click_position: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_observer)
        .add_systems(Update, trigger_events)
        .run();
}

// 注册观察者
fn setup_observer(mut commands: Commands, world: &mut World) {
    // 全局观察者：监听所有GameOverEvent
    world.add_observer(|game_over: On<GameOverEvent>| {
        info!("游戏结束！最终得分：{}", game_over.final_score);
    });

    // 生成按钮实体，注册实体专属观察者
    let button_entity = commands.spawn(Node::default()).id();
    commands.entity(button_entity).observe(|click: On<ButtonClickEvent>| {
        info!("按钮被点击！位置：{:?}", click.click_position);
        click.propagate(false); // 中断事件冒泡传播
    });
}

// 触发事件
fn trigger_events(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    button_query: Query<Entity, With<Node>>,
) {
    // 触发全局事件
    if keyboard.just_pressed(KeyCode::Escape) {
        commands.trigger(GameOverEvent { final_score: 1200 });
    }

    // 触发实体定向事件
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(button_entity) = button_query.get_single() {
            commands.trigger(ButtonClickEvent {
                entity: button_entity,
                click_position: Vec2::new(150.0, 300.0),
            });
        }
    }
}
```

### 2. Message 标准用法（缓冲-读写模式）
```rust
use bevy::prelude::*;

// 定义Message类型
#[derive(Message, Debug, Clone)]
struct PlayerDamageMessage {
    player_id: Entity,
    damage_value: f32,
    source: Entity,
}

#[derive(Component)]
struct Player {
    health: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Message必须手动注册，初始化全局队列
        .add_message::<PlayerDamageMessage>()
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (produce_damage_msg, process_damage_msg).chain())
        .run();
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player { health: 100.0 });
}

// 写入消息：使用MessageWriter
fn produce_damage_msg(
    mut damage_writer: MessageWriter<PlayerDamageMessage>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
) {
    // 按空格触发伤害，写入消息队列
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok(player_entity) = player_query.get_single() {
            damage_writer.send(PlayerDamageMessage {
                player_id: player_entity,
                damage_value: 25.0,
                source: Entity::PLACEHOLDER,
            });
        }
    }
}

// 读取消费消息：使用MessageReader
fn process_damage_msg(
    mut damage_reader: MessageReader<PlayerDamageMessage>,
    mut player_query: Query<&mut Player>,
) {
    // 按FIFO顺序读取所有未消费的消息
    for damage_msg in damage_reader.read() {
        if let Ok(mut player) = player_query.get_mut(damage_msg.player_id) {
            player.health -= damage_msg.damage_value;
            info!(
                "受到{}点伤害，剩余血量：{:.1}",
                damage_msg.damage_value, player.health
            );
        }
    }
}
```

---

### 最终选型原则（官方设计意图）
- 当你需要**触发后立即执行回调**、**绑定实体**、**层级传播**、**极致解耦触发与响应逻辑**时，使用 `Event`
- 当你需要**跨系统/跨帧延迟处理**、**批量顺序消费**、**数据暂存**、**严格遵循系统调度顺序**时，使用 `Message`

详见 [message-vs-events](https://deepwiki.com/bevyengine/bevy/2.6-change-detection-and-events#message-vs-events)

## 性能优化

- 使用 `With<T>` / `Without<T>` 过滤
- 使用 `Changed<T>` 只处理变更
- 批量操作: `query.iter_mut()`

详见 [Bevy 官方文档](https://bevyengine.org/learn/book/getting-started/ecs/)
