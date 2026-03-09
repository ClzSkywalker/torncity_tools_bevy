use thiserror::Error;

/// 浏览器打开错误类型
#[derive(Error, Debug, Clone)]
pub enum BrowserError {
    /// 不支持的操作
    // #[error("Operation not supported on this platform")]
    // NotSupported,

    /// 无效的 URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// 系统级错误
    #[error("System error: {0}")]
    SystemError(String),
}

/// 浏览器操作 trait
pub trait BrowserBackend {
    /// 在浏览器中打开 URL
    fn open(&self, url: &str) -> Result<(), BrowserError>;
}

// ==================== Desktop 实现 ====================
#[cfg(not(target_os = "android"))]
mod desktop {
    use super::*;

    pub struct DesktopBrowser;

    impl DesktopBrowser {
        pub fn new() -> Self {
            Self
        }
    }

    impl BrowserBackend for DesktopBrowser {
        fn open(&self, url: &str) -> Result<(), BrowserError> {
            webbrowser::open(url)
                .map_err(|e| BrowserError::SystemError(e.to_string()))
        }
    }
}

// ==================== Android 实现 ====================
#[cfg(target_os = "android")]
mod android {
    use super::*;
    use jni::objects::{JObject, JValue};
    use jni::{AttachGuard, JavaVM};

    pub struct AndroidBrowser {
        vm: JavaVM,
    }

    impl AndroidBrowser {
        pub fn new() -> Self {
            let ctx = ndk_context::android_context();
            let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }
                .expect("Failed to get JavaVM");
            Self { vm }
        }

        fn get_jni_env(&self) -> Result<AttachGuard<'_>, BrowserError> {
            self.vm
                .attach_current_thread()
                .map_err(|e| BrowserError::SystemError(format!("Failed to attach thread: {}", e)))
        }
    }

    impl BrowserBackend for AndroidBrowser {
        fn open(&self, url: &str) -> Result<(), BrowserError> {
            let mut env = self.get_jni_env()?;
            let ctx = ndk_context::android_context();
            let activity = unsafe { JObject::from_raw(ctx.context().cast()) };

            // 验证 URL
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(BrowserError::InvalidUrl(url.to_string()));
            }

            // 创建 Uri
            let url_jstring = env
                .new_string(url)
                .map_err(|e| BrowserError::SystemError(e.to_string()))?;

            let uri = env
                .call_static_method(
                    "android/net/Uri",
                    "parse",
                    "(Ljava/lang/String;)Landroid/net/Uri;",
                    &[JValue::Object(&url_jstring)],
                )
                .map_err(|e| BrowserError::SystemError(e.to_string()))?
                .l()
                .map_err(|e| BrowserError::SystemError(e.to_string()))?;

            // 创建 Intent
            let intent_class = env
                .find_class("android/content/Intent")
                .map_err(|e| BrowserError::SystemError(e.to_string()))?;

            let action_view = env
                .new_string("android.intent.action.VIEW")
                .map_err(|e| BrowserError::SystemError(e.to_string()))?;

            let intent = env
                .new_object(
                    intent_class,
                    "(Ljava/lang/String;Landroid/net/Uri;)V",
                    &[JValue::Object(&action_view), JValue::Object(&uri)],
                )
                .map_err(|e| BrowserError::SystemError(e.to_string()))?;

            // 启动 Activity
            env.call_method(
                &activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[JValue::Object(&intent)],
            )
            .map_err(|e| BrowserError::SystemError(e.to_string()))?;

            Ok(())
        }
    }
}

// ==================== 统一导出 ====================
#[cfg(not(target_os = "android"))]
pub use desktop::DesktopBrowser as PlatformBrowser;

#[cfg(target_os = "android")]
pub use android::AndroidBrowser as PlatformBrowser;

/// 浏览器操作提供者（平台无关）
pub struct BrowserProvider {
    backend: PlatformBrowser,
}

impl BrowserProvider {
    /// 创建新的浏览器操作实例
    pub fn new() -> Self {
        Self {
            backend: PlatformBrowser::new(),
        }
    }

    /// 在浏览器中打开 URL
    pub fn open(&self, url: &str) -> Result<(), BrowserError> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(BrowserError::InvalidUrl(url.to_string()));
        }
        self.backend.open(url)
    }
}

impl Default for BrowserProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    
    #[test]
    // ai 别执行这个单测
    fn test_browser_provider_creation() {
        // let provider = BrowserProvider::new();
        // assert!(provider.open("https://www.example.com").is_ok() || cfg!(target_os = "android"));
    }
}
