use bevy_ecs::event::Event;

/// 请求读取剪贴板内容的事件
///
/// 用户通过快捷键或 UI 按钮触发此事件来读取剪贴板
#[derive(Event, Debug, Clone)]
pub struct ReadClipboardEvent;

/// 剪贴板读取结果事件
///
/// 包含读取的文本内容或错误信息
#[derive(Event, Debug, Clone)]
pub struct ClipboardReadResult {
    /// 剪贴板文本内容 (如果读取成功)
    pub content: Option<String>,
    /// 错误信息 (如果读取失败)
    pub error: Option<String>,
}

impl ClipboardReadResult {
    /// 创建一个成功的结果
    pub fn success(content: String) -> Self {
        Self {
            content: Some(content),
            error: None,
        }
    }

    /// 创建一个失败的结果
    pub fn error(error: String) -> Self {
        Self {
            content: None,
            error: Some(error),
        }
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.content.is_some()
    }

    /// 检查是否失败
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
}
