# bevy_storage

跨平台配置持久化存储库，基于 `sysdirs` 实现，为 Bevy 应用提供简单易用的配置管理功能。

## 特性

- ✅ **跨平台支持** - 自动适配 macOS/Linux/Windows/iOS/Android/WASM
- ✅ **类型安全** - 基于 serde 序列化，编译时类型检查
- ✅ **Bevy 集成** - 插件化架构，与 Bevy Resource 系统无缝对接
- ✅ **自动路径管理** - 遵循各平台标准存储规范
- ✅ **错误处理** - 完善的错误类型和优雅降级

## 快速开始

### 1. 添加依赖

```toml
[dependencies]
bevy_storage.workspace = true
```

### 2. 注册插件

```rust
use bevy::prelude::*;
use bevy_storage::StoragePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StoragePlugin::default())  // 添加存储插件
        .run();
}
```

### 3. 使用存储

```rust
use bevy::prelude::*;
use bevy_storage::{StorageManager, Serialize, Deserialize};

#[derive(Resource, Serialize, Deserialize, Default)]
struct GameSettings {
    volume: f32,
    difficulty: u8,
}

// 保存配置
fn save_settings(
    storage: Res<StorageManager>,
    settings: Res<GameSettings>,
) {
    match storage.save_app_config(settings.as_ref()) {
        Ok(_) => info!("Settings saved!"),
        Err(e) => error!("Failed to save: {}", e),
    }
}

// 加载配置
fn load_settings(
    storage: Res<StorageManager>,
    mut settings: ResMut<GameSettings>,
) {
    match storage.load_app_config::<GameSettings>() {
        Ok(loaded) => *settings = loaded,
        Err(_) => warn!("No saved settings found, using defaults"),
    }
}
```

## 存储位置

配置文件会自动存储到各平台的标准位置：

### 桌面平台

| 平台 | 存储路径 |
|------|---------|
| **macOS** | `~/Library/Application Support/<Application>/app_config.json` |
| **Linux** | `~/.config/<Application>/app_config.json` |
| **Windows** | `C:\Users\<用户名>\AppData\Local\<Application>\app_config.json` |

**示例（默认配置）：**
```
macOS:   ~/Library/Application Support/torncity_tool/app_config.json
Linux:   ~/.config/torncity_tool/app_config.json
Windows: C:\Users\<用户名>\AppData\Local\torncity_tool\app_config.json
```

### 移动平台

| 平台 | 存储位置 |
|------|---------|
| **iOS** | App Sandbox 内 `Library/Application Support/` |
| **Android** | `/data/data/<包名>/files/` |

- 系统自动沙盒化，仅本应用可访问
- 卸载应用时自动清理

### Web 平台

| 平台 | 存储方式 | 容量限制 |
|------|---------|---------|
| **WASM** | `IndexedDB` | 50MB+ |

- 基于浏览器同源策略隔离
- 可在开发者工具中查看/编辑

## 自定义配置

### 自动应用名检测

应用名称会自动从可执行文件名检测：

```rust
// AppPaths::detect_app_name() 会自动从 std::env::current_exe() 获取
// 如果无法检测，默认使用 "app"
```

### 手动创建 StorageManager

```rust
use bevy_storage::StorageManager;

let storage = StorageManager::new();
```

## API 文档

### `StorageManager`

核心存储管理器，实现为 Bevy `Resource`。

#### 方法

```rust
// 创建新的存储管理器
pub fn new() -> Self

// 获取应用路径
pub fn app_paths(&self) -> &AppPaths

// 获取配置文件完整路径
pub fn app_config_path(&self) -> &PathBuf

// 保存应用配置（需要 Serialize trait）
pub fn save_app_config<T: Serialize>(&self, value: &T) -> Result<()>

// 加载应用配置（需要 Deserialize trait）
pub fn load_app_config<T: DeserializeOwned>(&self) -> Result<T>

// 加载应用配置，如果不存在则返回默认值
pub fn load_app_config_or_default<T: DeserializeOwned + Default>(&self) -> Result<T>
```

### `AppPaths`

应用目录路径结构。

```rust
pub struct AppPaths {
    pub config_dir: PathBuf,   // 配置文件目录
    pub data_dir: PathBuf,     // 数据文件目录
    pub cache_dir: PathBuf,    // 缓存目录
    pub temp_dir: PathBuf,     // 临时文件目录
}
```

### 错误类型

```rust
pub enum StorageError {
    SerializationError(String),    // 序列化失败
    DeserializationError(String),  // 反序列化失败
    NotFound(String),              // 文件不存在
    BackendError(String),          // 后端错误
}
```

## 完整示例

### 游戏设置保存/加载

```rust
use bevy::prelude::*;
use bevy_storage::{StorageManager, StorageError, Serialize, Deserialize};

#[derive(Resource, Serialize, Deserialize, Clone)]
struct GameConfig {
    audio_volume: f32,
    graphics_quality: String,
    player_name: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            audio_volume: 0.8,
            graphics_quality: "High".to_string(),
            player_name: "Player".to_string(),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StoragePlugin::default())
        .init_resource::<GameConfig>()
        .add_systems(Startup, load_config)
        .add_systems(Update, save_on_button_press)
        .run();
}

// 启动时加载配置
fn load_config(
    storage: Res<StorageManager>,
    mut config: ResMut<GameConfig>,
) {
    match storage.load_app_config::<GameConfig>() {
        Ok(loaded) => {
            *config = loaded;
            info!("✅ Configuration loaded");
        }
        Err(StorageError::NotFound(_)) => {
            warn!("⚠️  No saved config found, using defaults");
        }
        Err(StorageError::DeserializationError(_)) => {
            warn!("⚠️  Config format error, using defaults");
        }
        Err(e) => error!("❌ Failed to load config: {}", e),
    }
}

// 按下保存按钮时保存配置
fn save_on_button_press(
    keyboard: Res<ButtonInput<KeyCode>>,
    storage: Res<StorageManager>,
    config: Res<GameConfig>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        match storage.save_app_config(config.as_ref()) {
            Ok(_) => info!("✅ Configuration saved"),
            Err(e) => error!("❌ Failed to save: {}", e),
        }
    }
}
```

## 数据格式

### 桌面/移动平台

- **格式：** JSON（文本）
- **文件：** `app_config.json`
- **优点：** 人类可读、易于调试、跨平台兼容
- **缺点：** 相比二进制格式占用空间稍大

### Web 平台

- **格式：** JSON（文本）
- **存储：** `localStorage`
- **键格式：** `<Organization>#<Application>#<key>`
- **优点：** 可在浏览器开发者工具中查看/编辑

## 注意事项

### 安全性

⚠️ **重要：** 当前实现不提供加密功能。敏感数据（如 token、密码）以明文形式存储。

**建议：**
1. **应用层加密** - 存储前手动加密敏感字段
2. **系统密钥链** - 使用平台原生密钥链存储敏感数据：
   - macOS/iOS: Keychain
   - Windows: Credential Manager
   - Linux: Secret Service

```rust
// 示例：简单的字段加密（需要额外的加密库）
use chacha20poly1305::ChaCha20Poly1305;

// 保存前加密
let encrypted_token = encrypt(&config.token, &key);
storage.save("token", &encrypted_token)?;

// 加载后解密
let encrypted: Vec<u8> = storage.load("token")?;
let token = decrypt(&encrypted, &key);
```

### 数据迁移

如果修改数据结构，旧配置可能无法反序列化。建议：

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(default)]  // 新字段使用默认值
    new_field: bool,

    #[serde(default = "default_value")]  // 自定义默认值
    another_field: i32,
}

fn default_value() -> i32 { 42 }
```

### 性能考虑

- **读取：** 每次读取都需要反序列化，适合低频访问
- **写入：** 高性能键值存储，适合频繁保存
- **建议：** 将热数据保存在内存（Resource），定期批量写入

## 依赖

- `sysdirs` - 跨平台目录检测
- `serde` - 序列化支持
- `serde_json` - JSON 支持
- `thiserror` - 错误处理

## 故障排查

### 运行时错误：无法创建存储目录

- **原因：** 权限不足或路径不存在
- **解决：** 确保应用有写入权限，或手动创建父目录

### 数据丢失

- **原因：** 修改了数据结构，反序列化失败
- **解决：** 使用 `#[serde(default)]` 或实现数据迁移逻辑

## 更新日志

### v0.1.0
- 初始版本
- 支持 macOS/Linux/Windows/iOS/Android
- 基于 sysdirs 跨平台目录检测
- JSON 格式配置存储
- 完整的错误处理
- 类型安全的 API

## 许可证

与父项目保持一致。

## 贡献

欢迎提交 Issue 和 Pull Request！
