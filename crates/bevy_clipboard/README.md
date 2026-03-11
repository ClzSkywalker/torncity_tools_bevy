# bevy_clipboard

桌面剪贴板读取插件,基于 Bevy 0.18 的 Observer 事件系统。

## 功能

- ✅ 读取系统剪贴板文本内容
- ✅ 完整的错误处理
- ✅ 桌面平台支持 (Windows, macOS, Linux)

## 使用方法

### 1. 添加插件

```rust
use bevy::prelude::*;
use bevy_clipboard::ClipboardPlugin;

fn main() {
    App::new()
        .add_plugins(ClipboardPlugin)
        .run();
}
```

### 2. 触发剪贴板读取

```rust
use bevy_clipboard::ReadClipboardEvent;

fn trigger_read(world: &mut World) {
    // 触发剪贴板读取
    world.trigger(ReadClipboardEvent);
}
```

### 3. 处理读取结果

```rust
use bevy_clipboard::ClipboardReadResult;

fn handle_result(trigger: On<ClipboardReadResult>) {
    if let Some(content) = &trigger.content {
        bevy::log::info!("读取成功: {}", content);
        // 在这里处理剪贴板内容,比如解析 curl 命令等
    } else if let Some(error) = &trigger.error {
        bevy::log::error!("读取失败: {}", error);
    }
}

// 在 App 中添加观察者
app.add_observer(handle_result);
```

## 事件流

```
ReadClipboardEvent (用户触发)
    ↓
ClipboardReadResult (剪贴板读取结果)
```

## 完整示例

```rust
use bevy::prelude::*;
use bevy_clipboard::{ClipboardPlugin, ClipboardReadResult, ReadClipboardEvent};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(ClipboardPlugin)
        .add_observer(handle_read_result)
        .add_systems(Startup, trigger_clipboard_read)
        .run();
}

fn trigger_clipboard_read(world: &mut World) {
    bevy::log::info!("正在读取剪贴板...");
    world.trigger(ReadClipboardEvent);
}

fn handle_read_result(trigger: On<ClipboardReadResult>) {
    if let Some(content) = &trigger.content {
        bevy::log::info!("✓ 读取成功!");
        bevy::log::info!("  内容: {}", content);
    } else if let Some(error) = &trigger.error {
        bevy::log::error!("✗ 读取失败: {}", error);
    }
}
```

## 运行示例

```bash
# 1. 复制一些文本到剪贴板

# 2. 运行示例
cargo run --example clipboard_test
```

## 依赖

- `bevy_app` 0.18
- `bevy_ecs` 0.18
- `arboard` 3.4 (跨平台剪贴板库)
- `thiserror` 2.0

## 架构

```
crates/base/bevy_clipboard/
  ├── src/
  │   ├── lib.rs           # 插件定义
  │   ├── events.rs        # 事件类型
  │   ├── clipboard.rs     # 剪贴板操作封装
  │   └── systems.rs       # 观察者系统
  └── Cargo.toml
```

## 注意事项

- 仅支持桌面平台 (Windows, macOS, Linux)
- 移动端支持计划中但未实现
- 剪贴板内容必须是文本格式
- 此插件只负责读取剪贴板,不包含任何业务逻辑(如 curl 解析等)

## 错误处理

所有错误通过事件传递,不会导致应用崩溃:

- **剪贴板为空**: `ClipboardReadResult` 包含错误信息
- **内容不是文本**: `ClipboardReadResult` 包含错误信息
- **系统权限问题**: `ClipboardReadResult` 包含错误信息

## License

与父项目相同
