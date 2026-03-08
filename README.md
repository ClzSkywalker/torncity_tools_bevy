# Torn Trade

> 为 [Torn City](https://www.torn.com/) 商人打造的智能交易工具，基于 [weav3r](https://weav3r.dev/) 数据源，实时刷新交易数据，智能筛选高利润商品，助您轻松把握商机。

## 功能特性

- **实时数据刷新** - 自动从 weav3r.dev 获取最新交易数据
- **智能利润计算** - 自动计算商品利润率，识别高利润机会
- **自定义筛选** - 支持自定义筛选条件，精准定位目标商品
- **Bazaar 链接直达** - 一键跳转到高利润商品的 Bazaar 店铺
- **跨平台支持** - 支持 macOS、Windows 和 Android 平台
- **本地数据缓存** - 智能缓存机制，减少网络请求，提升响应速度
- **用户收藏管理** - 支持收藏关注的商品和用户

## 技术栈

- **Rust** - 高性能、内存安全的系统编程语言
- **Bevy 0.18.1** - 现代化的游戏引擎，提供 ECS 架构和丰富的 UI 组件
- **Bevy Feathers** - 为 Bevy UI 提供的高级 UI 组件库
- **ehttp** - 轻量级 HTTP 客户端，支持异步请求
- **自定义 Bevy 插件**:
  - `bevy_http` - HTTP 请求封装
  - `bevy_clipboard` - 剪贴板操作支持
  - `bevy_storage` - 本地存储管理
  - `bevy_tab` - 标签页 UI 组件
  - `bevy_remote_image` - 远程图片加载
  - `bevy_ui_fonts` - UI 字体管理

## 快速开始

### 环境要求

- Rust 2024 edition
- Cargo 包管理器

### 运行项目

```bash
# 开发模式运行
cargo run

# 发布模式运行
cargo run --release
```

## 项目结构

```
torncity_tools_bevy/
├── crates/              # 自定义 Bevy 插件
│   ├── bevy_http/      # HTTP 请求插件
│   ├── bevy_clipboard/ # 剪贴板插件
│   ├── bevy_storage/   # 存储插件
│   ├── bevy_tab/       # 标签页插件
│   └── ...
├── src/
│   ├── components/     # UI 组件
│   ├── http/          # HTTP 请求处理
│   ├── model/         # 数据模型
│   ├── resource/      # 资源管理
│   ├── tools/         # 工具函数
│   ├── view/          # 视图层
│   └── weav3r/        # Weav3r 数据处理
├── assets/            # 资源文件
└── build/             # 构建脚本和配置
```

## 使用说明

1. **启动应用** - 运行程序后，应用会自动从 weav3r.dev 加载交易数据
2. **查看利润商品** - 主界面显示按利润排序的商品列表
3. **筛选商品** - 使用筛选条件过滤商品，找到符合需求的交易机会
4. **跳转购买** - 点击商品卡片可直接跳转到对应的 Bazaar 店铺

## 开发指南

### 添加新功能

1. 在 `src/` 相应目录下添加模块
2. 在 `main.rs` 中注册模块
3. 使用 Bevy 的 ECS 系统实现逻辑

### 构建特定平台

```bash
# macOS
cargo build --release

# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Android
cd build/android
./package.sh
```

## 致谢

- [Torn City](https://www.torn.com/) - 游戏平台
- [weav3r](https://weav3r.dev/) - 数据源
- [Bevy](https://bevyengine.org/) - 游戏引擎

