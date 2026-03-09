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

```rust
// 定义 Event
#[derive(Event)]
pub struct MyEvent;

// 注册
app.add_event::<MyEvent>();

// 发送
commands.trigger(MyEvent { data: "".into() });

// 读取
fn handle(mut events: EventReader<MyEvent>) {
    for event in events.read() {}
}
```

### Event vs Message

Bevy 有两种消息传递机制，适用不同场景：

| 特性 | Event | Message |
|------|-------|---------|
| 注册 | 需 `app.add_event::<T>()` | 无需注册 |
| 生命周期 | 每帧清空（至少保留2帧） | 仅当帧有效 |
| 发送 | `EventWriter<T>` / `commands.trigger()` | Bevy 内置（只读） |
| 存储 | Events<T> 资源（Vec） | 立即投递 |
| 适用场景 | 跨系统通信、异步任务、游戏事件 | 输入处理、即时响应 |

**Event 适用场景**：
- 游戏事件（玩家升级、道具拾取）
- HTTP 响应等异步任务
- 需要跨帧累积的事件队列

**Message 适用场景**：
- 输入处理（键盘、鼠标、触摸）
- 需要即时响应的系统

```rust
// Event 示例：跨系统通信
#[derive(Event)]
pub struct LevelUpEvent(Entity);

app.add_event::<LevelUpEvent>();
fn send_event(mut writer: EventWriter<LevelUpEvent>) {
    writer.send(LevelUpEvent(entity));
}
fn handle_event(mut reader: EventReader<LevelUpEvent>) {
    for ev in reader.read() {}
}

// Message 示例：输入处理（内置类型，无需注册）
fn handle_input(mut mouse_wheel: MessageReader<MouseWheel>) {
    for event in mouse_wheel.read() {
        println!("scroll: {}", event.y);
    }
}
```

详见 [Bevy Cheatbook - Events](https://bevy-cheatbook.github.io/programming/events.html)

## 性能优化

- 使用 `With<T>` / `Without<T>` 过滤
- 使用 `Changed<T>` 只处理变更
- 批量操作: `query.iter_mut()`

详见 [Bevy 官方文档](https://bevyengine.org/learn/book/getting-started/ecs/)
