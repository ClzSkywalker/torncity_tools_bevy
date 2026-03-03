use thiserror::Error;

/// 剪贴板错误类型
#[derive(Error, Debug, Clone)]
pub enum ClipboardError {
    /// 剪贴板为空
    #[error("Clipboard is empty")]
    Empty,

    /// 剪贴板内容不是文本格式
    #[error("Clipboard content is not text")]
    NotText,

    /// 没有访问剪贴板的权限
    #[error("Permission denied to access clipboard")]
    PermissionDenied,

    /// 系统级错误
    #[error("System error: {0}")]
    SystemError(String),
}

/// 剪贴板操作提供者 trait
pub trait ClipboardBackend {
    /// 读取剪贴板文本内容
    fn read_text(&mut self) -> Result<String, ClipboardError>;

    /// 写入文本到剪贴板
    fn write_text(&mut self, text: &str) -> Result<(), ClipboardError>;
}

// ==================== Desktop 实现 ====================
#[cfg(not(target_os = "android"))]
mod desktop {
    use super::*;
    use arboard::Clipboard;

    pub struct DesktopClipboard {
        clipboard: Clipboard,
    }

    impl DesktopClipboard {
        pub fn new() -> Result<Self, ClipboardError> {
            Clipboard::new()
                .map(|clipboard| Self { clipboard })
                .map_err(|e| {
                    let error_msg = e.to_string().to_lowercase();
                    if error_msg.contains("permission") || error_msg.contains("access denied") {
                        ClipboardError::PermissionDenied
                    } else {
                        ClipboardError::SystemError(e.to_string())
                    }
                })
        }
    }

    impl ClipboardBackend for DesktopClipboard {
        fn read_text(&mut self) -> Result<String, ClipboardError> {
            self.clipboard.get_text().map_err(|e| {
                let error_msg = e.to_string().to_lowercase();

                if error_msg.contains("empty") || error_msg.contains("no content") {
                    ClipboardError::Empty
                } else if error_msg.contains("not text")
                    || error_msg.contains("not a string")
                    || error_msg.contains("format")
                {
                    ClipboardError::NotText
                } else if error_msg.contains("permission") || error_msg.contains("access denied") {
                    ClipboardError::PermissionDenied
                } else {
                    ClipboardError::SystemError(e.to_string())
                }
            })
        }

        fn write_text(&mut self, text: &str) -> Result<(), ClipboardError> {
            self.clipboard
                .set_text(text)
                .map_err(|e| ClipboardError::SystemError(e.to_string()))
        }
    }
}

// ==================== Android 实现 ====================
#[cfg(target_os = "android")]
mod android {
    use super::*;
    use jni::objects::{JObject, JString};
    use jni::{AttachGuard, JavaVM};

    pub struct AndroidClipboard {
        vm: JavaVM,
    }

    impl AndroidClipboard {
        pub fn new() -> Result<Self, ClipboardError> {
            let ctx = ndk_context::android_context();
            let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }
                .map_err(|e| ClipboardError::SystemError(format!("Failed to get JavaVM: {}", e)))?;
            Ok(Self { vm })
        }

        fn get_jni_env(&self) -> Result<AttachGuard<'_>, ClipboardError> {
            self.vm
                .attach_current_thread()
                .map_err(|e| ClipboardError::SystemError(format!("Failed to attach thread: {}", e)))
        }
    }

    impl ClipboardBackend for AndroidClipboard {
        fn read_text(&mut self) -> Result<String, ClipboardError> {
            let mut env = self.get_jni_env()?;
            let ctx = ndk_context::android_context();
            let activity = unsafe { JObject::from_raw(ctx.context().cast()) };

            // 获取 ClipboardManager
            let clipboard_service = env
                .new_string("clipboard")
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            let clipboard_manager = env
                .call_method(
                    &activity,
                    "getSystemService",
                    "(Ljava/lang/String;)Ljava/lang/Object;",
                    &[(&clipboard_service).into()],
                )
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?
                .l()
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            // 获取 PrimaryClip
            let primary_clip = env
                .call_method(
                    clipboard_manager,
                    "getPrimaryClip",
                    "()Landroid/content/ClipData;",
                    &[],
                )
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?
                .l()
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            if primary_clip.is_null() {
                return Err(ClipboardError::Empty);
            }

            // 获取第一个 ClipItem
            let item = env
                .call_method(primary_clip, "getItemAt", "(I)Landroid/content/ClipData$Item;", &[0.into()])
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?
                .l()
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            // 获取文本
            let text = env
                .call_method(item, "getText", "()Ljava/lang/CharSequence;", &[])
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?
                .l()
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            if text.is_null() {
                return Err(ClipboardError::NotText);
            }

            let text_string: JString = text.into();
            let rust_string: String = env
                .get_string(&text_string)
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?
                .into();

            Ok(rust_string)
        }

        fn write_text(&mut self, text: &str) -> Result<(), ClipboardError> {
            let mut env = self.get_jni_env()?;
            let ctx = ndk_context::android_context();
            let activity = unsafe { JObject::from_raw(ctx.context().cast()) };

            // 获取 ClipboardManager
            let clipboard_service = env
                .new_string("clipboard")
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            let clipboard_manager = env
                .call_method(
                    &activity,
                    "getSystemService",
                    "(Ljava/lang/String;)Ljava/lang/Object;",
                    &[(&clipboard_service).into()],
                )
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?
                .l()
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            // 创建 ClipData
            let label = env
                .new_string("text")
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            let text_jstring = env
                .new_string(text)
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            let clip_data = env
                .call_static_method(
                    "android/content/ClipData",
                    "newPlainText",
                    "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/content/ClipData;",
                    &[(&label).into(), (&text_jstring).into()],
                )
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?
                .l()
                .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            // 设置 PrimaryClip
            env.call_method(
                clipboard_manager,
                "setPrimaryClip",
                "(Landroid/content/ClipData;)V",
                &[(&clip_data).into()],
            )
            .map_err(|e| ClipboardError::SystemError(e.to_string()))?;

            Ok(())
        }
    }
}

// ==================== 统一导出 ====================
#[cfg(not(target_os = "android"))]
pub use desktop::DesktopClipboard as PlatformClipboard;

#[cfg(target_os = "android")]
pub use android::AndroidClipboard as PlatformClipboard;

/// 剪贴板操作提供者（平台无关）
pub struct ClipboardProvider {
    backend: PlatformClipboard,
}

impl ClipboardProvider {
    /// 创建新的剪贴板访问实例
    pub fn new() -> Result<Self, ClipboardError> {
        Ok(Self {
            backend: PlatformClipboard::new()?,
        })
    }

    /// 读取剪贴板文本内容
    pub fn read_text(&mut self) -> Result<String, ClipboardError> {
        self.backend.read_text()
    }

    /// 写入文本到剪贴板
    pub fn write_text(&mut self, text: &str) -> Result<(), ClipboardError> {
        self.backend.write_text(text)
    }
}

impl Default for ClipboardProvider {
    fn default() -> Self {
        Self::new().expect("Failed to initialize clipboard")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_provider_creation() {
        let result = ClipboardProvider::new();
        assert!(result.is_ok(), "Should be able to create clipboard provider");
    }

    #[test]
    #[cfg(not(target_os = "android"))]
    fn test_write_and_read() {
        let mut provider = ClipboardProvider::new().expect("Failed to create provider");

        let test_text = "Hello, Clipboard!";
        provider
            .write_text(test_text)
            .expect("Failed to write text");

        let read_text = provider.read_text().expect("Failed to read text");
        assert_eq!(read_text, test_text);
    }
}
