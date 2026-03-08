# Bevy ECS 模式指南

本文档说明 Bevy ECS 的核心概念和项目中的常用模式。

## ECS 基础概念

### Entity (实体)
游戏世界中的唯一标识符，本质上只是一个 ID。

```rust
// 创建实体
commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 }));

// 获取实体 ID
for (entity, component) in &query {
    println!("Entity ID: {:?}", entity);
}
```

### Component (组件)
附加到实体上的数据，不包含逻辑。

```rust
#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
#[require(Position)]
struct Player { name: String }
```

### System (系统)
处理组件的逻辑，不存储状态。

```rust
fn move_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}
```

## 系统注册

### 基本注册

```rust
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, my_system);
    }
}
```

### 按状态运行

```rust
app.add_systems(Update, my_system.run_if(in_state(GameState::Menu)));
```

### 状态进入时运行

```rust
app.add_systems(OnEnter(GameState::Menu), setup_ui);
```

### 系统执行顺序

```rust
app.add_systems(Update, (
    system_a,
    system_b.after(system_a),
    system_c.before(system_b),
));
```

### 条件运行

```rust
// 资源变化时运行
app.add_systems(Update, update_config.run_if(resource_changed::<SettingConfigRes>));

// 多个条件
app.add_systems(Update, my_system.run_if(
    resource_changed::<SettingConfigRes>.and_then(in_state(GameState::Menu))
));
```

## 系统参数

### Commands

```rust
fn spawn_system(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, PlayerMarker));
    commands.insert_resource(MyResource::default());
    commands.trigger(MyEvent);
}
```

### Resource

```rust
// 只读
fn read_resource(config: Res<Config>) { println!("{}", config.value); }

// 可变
fn write_resource(mut config: ResMut<Config>) { config.value = 42; }

// 可选
fn optional_resource(config: Option<Res<Config>>) {
    if let Some(config) = config { println!("{}", config.value); }
}
```

## 查询模式

### 基本查询

```rust
// 读取
fn read_system(query: Query<&Position>) {
    for position in &query { println!("{}, {}", position.x, position.y); }
}

// 修改
fn write_system(mut query: Query<&mut Position>) {
    for mut position in &mut query { position.x += 1.0; }
}
```

### 多组件查询

```rust
fn multi_system(query: Query<(&Position, &Velocity)>) {
    for (position, velocity) in &query {
        println!("Position: ({}, {}), Velocity: ({}, {})",
            position.x, position.y, velocity.x, velocity.y);
    }
}
```

### 带过滤的查询

```rust
// 包含特定组件
fn with_marker(query: Query<&Position, With<PlayerMarker>>) {
    for position in &query { println!("Player: ({}, {})", position.x, position.y); }
}

// 不包含特定组件
fn without_marker(query: Query<&Position, Without<PlayerMarker>>) {
    for position in &query { println!("NPC: ({}, {})", position.x, position.y); }
}
```

### 变更检测

```rust
fn changed_system(query: Query<&Position, Changed<Position>>) {
    for position in &query { println!("Changed: ({}, {})", position.x, position.y); }
}

fn added_system(query: Query<&Position, Added<Position>>) {
    for position in &query { println!("Added: ({}, {})", position.x, position.y); }
}
```

### 单个实体查询

```rust
fn single_system(query: Query<&Position, With<PlayerMarker>>) {
    let position = query.single();
    println!("Player: ({}, {})", position.x, position.y);
}
```

## 资源管理

### 定义资源

```rust
#[derive(Resource, Clone, Debug)]
pub struct MyResource { pub value: i32 }

impl Default for MyResource {
    fn default() -> Self { Self { value: 0 } }
}
```

### 注册资源

```rust
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyResource>();  // 使用默认值
        app.insert_resource(MyResource { value: 42 });  // 自定义值
    }
}
```

## 事件系统

### 定义和注册事件

```rust
#[derive(Event, Clone, Debug)]
pub struct MyEvent { pub data: String }

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MyEvent>();
    }
}
```

### 发送和读取事件

```rust
// 发送
commands.trigger(MyEvent { data: "Hello".to_string() });

// 读取
fn handle_event(mut events: EventReader<MyEvent>) {
    for event in events.read() { println!("{}", event.data); }
}
```

## 常见模式

### 标记模式

```rust
#[derive(Component)]
struct PlayerMarker;

fn player_system(query: Query<&Position, With<PlayerMarker>>) {
    for position in &query { println!("Player: ({}, {})", position.x, position.y); }
}
```

### 定时器模式

```rust
#[derive(Component)]
struct TimerComponent { timer: Timer }

fn timer_system(mut query: Query<&mut TimerComponent>, time: Res<Time>) {
    for mut timer_component in &mut query {
        timer_component.timer.tick(time.delta());
        if timer_component.timer.just_finished() { println!("Timer finished!"); }
    }
}
```

## 性能优化

### 使用过滤器

```rust
// 好
fn optimized(query: Query<&Position, With<PlayerMarker>>) { }

// 不好
fn unoptimized(query: Query<(&Position, Option<&PlayerMarker>)>) {
    for (position, marker) in &query {
        if marker.is_some() { }
    }
}
```

### 使用 Changed 过滤器

```rust
fn changed(query: Query<&mut Position, Changed<Position>>) {
    for mut position in &query { position.x += 1.0; }
}
```

### 批量操作

```rust
fn batch(mut query: Query<&mut Position>) {
    for mut position in query.iter_mut() { position.x += 1.0; }
}
```

## 常见错误

### 查询冲突

```rust
// 错误：同时可变查询同一组件
fn conflict(mut query1: Query<&mut Position>, mut query2: Query<&mut Position>>) { }

// 正确
fn correct(mut query: Query<&mut Position>) {
    for mut position in &mut query { position.x += 1.0; }
}
```

### 忘记注册资源

```rust
// 错误：资源未注册
fn use_resource(resource: Res<MyResource>) { }

// 正确：在插件中注册
app.init_resource::<MyResource>();
```

## 参考资料

- [Bevy ECS 官方文档](https://bevyengine.org/learn/book/getting-started/ecs/)
- [Bevy Cheatbook - ECS](https://bevy-cheatbook.github.io/basics/ec.html)
