use bevy_ecs::observer::On;
use bevy_ecs::system::Commands;

use crate::clipboard::ClipboardProvider;
use crate::events::{ClipboardReadResult, ReadClipboardEvent};

/// 剪贴板读取系统
///
/// 监听 `ReadClipboardEvent`,读取剪贴板内容并触发 `ClipboardReadResult` 事件
pub fn clipboard_read_system(_: On<ReadClipboardEvent>, mut cmd: Commands) {
    // 创建剪贴板提供者
    let mut provider = match ClipboardProvider::new() {
        Ok(p) => p,
        Err(e) => {
            cmd.trigger(ClipboardReadResult::error(e.to_string()));
            return;
        }
    };

    // 读取文本
    match provider.read_text() {
        Ok(content) => {
            cmd.trigger(ClipboardReadResult::success(content));
        }
        Err(e) => {
            cmd.trigger(ClipboardReadResult::error(e.to_string()));
        }
    }
}
