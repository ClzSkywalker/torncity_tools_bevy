# 调试指南

调试技巧和常见问题。

## 日志

```bash
RUST_LOG=debug BEVY_BACKTRACE=full RUST_BACKTRACE=full cargo run --features=dev
```

## 常见问题

- **HTTP 请求失败**: 检查网络 / 确认 weav3r.dev 可用 / 查看日志
- **UI 不更新**: 检查系统状态 / 查询条件 / 资源是否更新
