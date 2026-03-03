mod clipboard;
mod events;
mod systems;

pub use clipboard::{ClipboardError, ClipboardProvider};
pub use events::{ClipboardReadResult, ReadClipboardEvent};
pub use systems::clipboard_read_system;

use bevy_app::{App, Plugin};

/// 剪贴板插件
///
/// 提供剪贴板读取功能
///
/// # 功能
///
/// - 读取剪贴板文本内容
/// - 完整的错误处理
///
/// # 使用示例
///
/// ```rust,ignore
/// use bevy::prelude::*;
/// use bevy_clipboard::ClipboardPlugin;
///
/// fn main() {
///     App::new()
///         .add_plugins(ClipboardPlugin)
///         .run();
/// }
/// ```
///
/// # 事件流
///
/// ```text
/// ReadClipboardEvent -> ClipboardReadResult
/// ```
pub struct ClipboardPlugin;

impl Default for ClipboardPlugin {
    fn default() -> Self {
        Self
    }
}

impl Plugin for ClipboardPlugin {
    fn build(&self, app: &mut App) {
        // 注册观察者
        app.add_observer(clipboard_read_system);
    }
}
