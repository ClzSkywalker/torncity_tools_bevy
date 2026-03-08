# Claude Code 项目指南

**Torn Trade** 是一个为 Torn City 商人打造的智能交易工具，基于 Rust 和 Bevy 游戏引擎开发。该项目使用 ECS (Entity Component System) 架构，提供实时交易数据刷新、智能利润计算和商品筛选功能。

## 项目概述

### 核心技术栈

- **语言**: Rust 2024 edition
- **引擎**: Bevy 0.18.1 (ECS 架构)
- **UI 框架**: Bevy UI + Bevy Feathers
- **HTTP 客户端**: ehttp
- **跨平台**: macOS, Windows, Android

### 项目结构

```
torncity_tools_bevy/
├── crates/              # 自定义 Bevy 插件
│   ├── bevy_theme/     # 主题系统插件
│   ├── bevy_http/      # HTTP 请求封装
│   ├── bevy_clipboard/ # 剪贴板操作
│   ├── bevy_storage/   # 本地存储管理
│   ├── bevy_tab/       # 标签页 UI 组件
│   ├── bevy_remote_image/ # 远程图片加载
│   └── bevy_ui_fonts/  # UI 字体管理
├── src/
│   ├── components/     # 可复用 UI 组件
│   ├── http/          # HTTP 请求处理
│   ├── model/         # 数据模型和业务逻辑
│   ├── resource/      # Bevy 资源
│   ├── tools/         # 工具函数和算法
│   ├── view/          # 视图层和 UI 逻辑
│   └── weav3r/        # Weav3r 数据处理
├── assets/            # 静态资源 (字体、图标等)
└── build/             # 构建脚本和平台配置
```

## 快速开始

### 环境要求

- Rust 2024 edition
- Cargo 包管理器

### 构建项目

```bash
git clone <repository-url>
cd torncity_tools_bevy
cargo build --release
```

### 运行项目

```bash
cargo run                    # 开发模式
cargo run --release          # 发布模式
RUST_LOG=debug cargo run      # 带日志运行
```

## 代码风格和约定

### 命名约定

- **模块名**: `snake_case`，如 `trader_card`
- **结构体/枚举**: `PascalCase`，如 `GamePlugin`, `GameState`
- **函数/方法**: `snake_case`，如 `handle_weav3r_resp`
- **常量**: `SCREAMING_SNAKE_CASE`，如 `DEFAULT_TEXT_FONT_PATH`
- **资源**: 使用 `Res` 后缀，如 `SettingConfigRes`, `ItemsDatabase`

### Bevy ECS 基础

**核心概念**:
- **Entity**: 游戏实体（ID）
- **Component**: 组件（数据）
- **System**: 系统（逻辑）
- **Resource**: 资源（全局状态）

详细 ECS 模式请参考 [docs/CLAUDE_ECS.md](docs/CLAUDE_ECS.md)

### 错误处理

- 使用 `thiserror` 定义错误类型
- 使用 `Result<T, E>` 处理可恢复错误
- 使用 `?` 操作符传播错误
- 在 UI 系统中使用 `bevy::log::error!` 记录错误

### 异步处理

- 使用 `ehttp` 进行 HTTP 请求
- 使用 `crossbeam-channel` 进行线程间通信
- HTTP 响应通过事件系统传递到主线程

## 架构概览

### GameState 状态系统

项目使用 Bevy 的状态系统管理应用生命周期。

**状态定义**:
```rust
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Asset,      // 资源加载阶段
    InitConfig, // 初始化配置阶段
    Menu,       // 主菜单/运行阶段
}
```

**状态流转**: `启动应用 → [Asset] → [InitConfig] → [Menu]`

详细架构说明请参考 [docs/CLAUDE_ARCHITECTURE.md](docs/CLAUDE_ARCHITECTURE.md)

### 插件系统

项目采用模块化插件架构:

1. **GamePlugin**: 核心插件，管理游戏状态
2. **ViewPlugin**: 视图插件，管理 UI 和相机
3. **ComponentsPlugin**: UI 组件插件
4. **自定义插件**: 各功能模块的专用插件

### 数据流

```
HTTP Request → HTTP Response → Data Processing → Resource Update → UI Update
```

## 开发指南

### 添加新功能

1. **创建模块**: 在 `src/` 相应目录下创建新模块
2. **定义数据模型**: 在 `src/model/` 中添加数据结构
3. **实现系统**: 在新模块中实现 Bevy 系统
4. **注册插件**: 在 `main.rs` 或相应插件中注册
5. **添加测试**: 在模块内添加测试

详细开发指南请参考 [docs/CLAUDE_DEV.md](docs/CLAUDE_DEV.md)

### 添加 UI 组件

1. 在 `src/components/` 中创建组件模块
2. 定义组件结构体和插件
3. 实现生成和更新系统
4. 在 `ComponentsPlugin` 中注册

### 使用 bevy_theme 主题系统

项目使用 `bevy_theme` 插件提供主题支持。新添加的 UI 组件应使用主题系统来保持一致性。

**正确用法：必须同时添加主题组件和颜色组件**

```rust
use bevy_theme::prelude::*;

// 错误示例：只添加主题组件（颜色不会生效）
commands.spawn((
    ThemedBackground::primary(),  // 缺少 BackgroundColor！
    Node { ... },
));

// 正确示例：同时添加主题组件和颜色组件
commands.spawn((
    ThemedBackground::primary(),  // 主题组件
    BackgroundColor(Color::BLACK), // 颜色组件（必须添加！）
    Node { ... },
));

// 文字主题示例
commands.spawn((
    ThemedText::primary(),
    TextColor(Color::BLACK),  // 颜色组件（必须添加！）
    Text::new("Hello"),
));
```

**可用的主题组件：**

| 组件 | 颜色组件 | 可选层 |
|------|----------|--------|
| `ThemedBackground` | `BackgroundColor` | `primary()`, `secondary()`, `tertiary()`, `deep()`, `elevated()` |
| `ThemedBorder` | `BorderColor` | `subtle()`, `default()`, `active()` |
| `ThemedText` | `TextColor` | `primary()`, `secondary()`, `muted()` |
| `ThemedState` | `BackgroundColor` | `success()`, `warning()`, `error()`, `info()`, `primary()` |

**动态切换主题层：**

如果需要在运行时切换主题层，可以修改组件的 `layer` 字段：

```rust
fn switch_theme_layer(
    mut query: Query<&mut ThemedBackground>,
) {
    for mut themed_bg in &mut query {
        themed_bg.layer = ThemedBackgroundLayer::Elevated; // 切换到高亮层
    }
}
```

详细文档请参考 [crates/bevy_theme/README.md](crates/bevy_theme/README.md)

### 添加 HTTP 端点

1. 在 `src/http/` 中创建请求模块
2. 定义请求和响应结构
3. 实现发送和响应处理系统
4. 在 `ViewPlugin` 中注册 HTTP 插件

### 添加资源

1. 在 `src/resource/` 中定义资源结构
2. 实现 `Resource` trait
3. 在插件中使用 `insert_resource` 或 `init_resource` 注册

## 测试指南

### 运行测试

```bash
cargo test                           # 运行所有测试
cargo test tools::order_change         # 运行特定模块
cargo test test_no_change             # 运行特定测试
cargo test -- --nocapture            # 显示测试输出
```

### 编写测试

- 测试文件与源文件同名，位于同一目录
- 使用 `#[test]` 属性标记测试函数
- 使用 `assert!`、`assert_eq!` 等宏进行断言
- 使用 `eprintln!` 输出调试信息

## 构建和部署

### 开发构建

```bash
cargo run                    # 运行开发版本
RUST_LOG=debug cargo run      # 带日志运行
```

### 发布构建

```bash
cargo build --release         # 构建发布版本
cargo run --release          # 运行发布版本
```

### 跨平台构建

```bash
# macOS
cargo build --release

# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Android
cd build/android && ./package.sh
```

### 代码检查

```bash
cargo clippy                 # 运行 clippy
cargo clippy --fix          # 修复问题
cargo fmt                   # 格式化代码
```

## 常见问题

### Clippy 类型复杂度警告

项目配置了 `type-complexity-threshold = 999`，禁用了类型复杂度检查。如需启用，修改 `clippy.toml`。

### Android 构建问题

- 确保 Android SDK 和 NDK 已正确安装
- 检查 `build/android/keystore/` 中的签名配置
- 参考 `docs/ANDROID_SIGNING_GUIDE.md`

### HTTP 请求失败

- 检查网络连接
- 确认 weav3r.dev 服务可用
- 查看日志中的错误信息

### UI 不更新

- 确保系统在正确的状态下运行
- 检查查询条件是否正确
- 验证资源是否已更新

## 性能优化

1. **减少查询开销**: 使用 `With<T>` 和 `Without<T>` 过滤器
2. **缓存计算结果**: 使用资源存储计算结果
3. **批量操作**: 使用 `Query::iter_mut` 批量处理实体
4. **避免频繁分配**: 重用 Vec 和 String
5. **使用 Change Detection**: 使用 `Changed<T>` 只处理变更的组件

## 调试技巧

### 启用日志

```rust
use bevy::log;
log::info!("Info message");
log::warn!("Warning message");
log::error!("Error message");
```

### 运行时调试

```bash
RUST_LOG=debug cargo run      # 启用调试日志
cargo run --features dev     # 启用帧时间诊断
```

### Bevy Inspector

在开发模式下可以添加 Bevy Inspector 插件查看实体和组件状态。

## 相关文档

**重要：当 Claude Code 需要详细信息时，请明确指定以下文档：**

### 架构文档
- **[项目架构详细说明](docs/CLAUDE_ARCHITECTURE.md)** - GameState 状态系统、插件系统、数据流、UI 组件层次

### ECS 模式
- **[Bevy ECS 模式指南](docs/CLAUDE_ECS.md)** - ECS 基础概念、系统注册模式、查询模式、资源管理、事件系统

### 开发指南
- **[开发指南](docs/CLAUDE_DEV.md)** - 添加新功能、UI 组件、HTTP 端点、测试、构建部署、性能优化、调试技巧

### 其他文档
- **[Android 签名指南](docs/ANDROID_SIGNING_GUIDE.md)** - Android 构建和签名配置

## 外部资源

- [Bevy 官方文档](https://bevyengine.org/learn/book/)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
- [Rust 官方文档](https://doc.rust-lang.org/)
- [Torn City](https://www.torn.com/)
- [weav3r](https://weav3r.dev/)
