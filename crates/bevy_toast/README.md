# bevy_toast

Toast 通知插件 for Bevy 0.18

## 安装

```toml
# Cargo.toml
[dependencies]
bevy_toast = { path = "crates/bevy_toast" }
```

## 快速开始

```rust
use bevy::prelude::*;
use bevy_toast::{ToastPlugin, ToastEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ToastPlugin)
        .run();
}

fn show_message(mut commands: Commands) {
    // 简单调用
    commands.trigger(ToastEvent::success("保存成功"));
    commands.trigger(ToastEvent::error("网络错误"));
    commands.trigger(ToastEvent::warning("背包已满"));
    commands.trigger(ToastEvent::info("新任务解锁"));

    // 自定义配置
    commands.trigger(
        ToastEvent::text("自定义消息")
            .with_position(ToastPosition::TopCenter)
            .with_duration(Duration::from_secs(3))
            .with_tap_to_dismiss(true)
    );
}
```

## API 概览

### ToastEvent 构造函数

| 方法 | 说明 |
|------|------|
| `ToastEvent::text("...")` | 纯文本 |
| `ToastEvent::success("...")` | 成功 (绿色) |
| `ToastEvent::error("...")` | 错误 (红色) |
| `ToastEvent::warning("...")` | 警告 (黄色) |
| `ToastEvent::info("...")` | 信息 (蓝色) |

### 构建器方法

| 方法 | 说明 |
|------|------|
| `.with_kind(kind)` | 设置类型 |
| `.with_position(pos)` | 设置位置 |
| `.with_channel(channel)` | 设置频道 |
| `.with_priority(prio)` | 设置优先级 |
| `.with_duration(dur)` | 设置显示时长 |
| `.with_tap_to_dismiss(bool)` | 点击关闭 |

### 位置

```rust
ToastPosition::TopLeft
ToastPosition::TopCenter
ToastPosition::TopRight
ToastPosition::Center
ToastPosition::BottomLeft
ToastPosition::BottomCenter  // 默认
ToastPosition::BottomRight
```

### 频道

```rust
ToastChannel::System   // 系统消息
ToastChannel::Combat   // 战斗消息
ToastChannel::Economy  // 经济消息
ToastChannel::Custom("quest")  // 自定义
```

### 优先级

```rust
ToastPriority::Low      // 低
ToastPriority::Normal   // 普通 (默认)
ToastPriority::High     // 高
ToastPriority::Critical // 紧急
```

## 配置

```rust
// 修改默认配置
app.insert_resource(ToastConfig {
    default_duration: Duration::from_secs(2),
    max_visible: 5,
    max_queue_size: 20,
    queue_strategy: QueueStrategy::Queue,
    ..default()
});
```

## 事件监听

```rust
fn handle_dismiss(mut events: EventReader<ToastDismissEvent>) {
    for event in events.read() {
        info!("Toast dismissed: {:?}", event.reason);
    }
}

fn handle_action(mut events: EventReader<ToastActionEvent>) {
    for event in events.read() {
        info!("Toast action: {:?}", event.action);
    }
}
```

## 特性

- ✅ 非阻塞提示
- ✅ 9 个位置布局
- ✅ 4 种内置类型
- ✅ 队列管理
- ✅ 去重机制
- ✅ 限速保护
- ✅ 频道控制
- ✅ 优先级调度

## License

MIT
