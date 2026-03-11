# Torn Trade

Torn City 智能交易工具，基于 Rust + Bevy ECS。

## 技术栈

- Rust 2024 / Bevy 0.18.1
- ehttp / crossbeam-channel
- bevy_theme 主题系统

## 运行

```bash
cargo run --features=dev   # 开发
```

## 核心概念

- **GameState**: Asset → InitConfig → Menu
- **插件**: GamePlugin / ViewPlugin / ComponentsPlugin
- **数据流**: HTTP → Event → Resource → UI

## 命名规范

- 模块: `snake_case`
- 结构体/枚举: `PascalCase`
- 资源: `Res` 后缀
- Event 事件: `Event` 后缀
- Message 事件: `Msg` 后缀
- 状态: `State` 后缀

## 文档

- [项目架构](docs/CLAUDE_ARCHITECTURE.md)
- [开发指南](docs/ai/dev_guide.md)
- [调试指南](docs/ai/debug_guide.md)
- [Android 签名](docs/ANDROID_SIGNING_GUIDE.md)
