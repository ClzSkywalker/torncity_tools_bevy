# 开发指南

本文档提供开发相关的指南和最佳实践。

## 添加新功能

1. **创建模块**: 在 `src/` 相应目录下创建新模块
2. **定义数据模型**: 在 `src/model/` 中添加数据结构
3. **实现系统**: 在新模块中实现 Bevy 系统
4. **注册插件**: 在 `main.rs` 或相应插件中注册
5. **添加测试**: 在模块内添加测试

## 添加 UI 组件

1. 在 `src/components/` 中创建组件模块
2. 定义组件结构体和插件
3. 实现生成和更新系统
4. 在 `ComponentsPlugin` 中注册

```rust
#[derive(Component)]
pub struct MyComponent { pub value: String }

pub struct MyComponentPlugin;

impl Plugin for MyComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_my_component);
    }
}
```

## 添加 HTTP 端点

1. 在 `src/http/` 中创建请求模块
2. 定义请求和响应结构
3. 实现发送和响应处理系统
4. 在 `ViewPlugin` 中注册 HTTP 插件

```rust
#[derive(Event, Clone, Debug)]
pub struct MyReqEvent { pub param: String }

#[derive(Event, Clone, Debug)]
pub struct MyRespEvent { pub resp: Result<MyData, HttpError> }
```

## 添加资源

1. 在 `src/resource/` 中定义资源结构
2. 实现 `Resource` trait
3. 在插件中使用 `insert_resource` 或 `init_resource` 注册

```rust
#[derive(Resource, Clone, Debug)]
pub struct MyResource { pub value: i32 }

impl Default for MyResource {
    fn default() -> Self { Self { value: 0 } }
}
```

## 测试指南

### 运行测试

```bash
cargo test                           # 运行所有测试
cargo test tools::order_change         # 运行特定模块
cargo test test_no_change             # 运行特定测试
cargo test -- --nocapture            # 显示测试输出
```

### 编写测试

```rust
#[test]
fn test_basic_functionality() {
    let result = my_function(42);
    assert_eq!(result, 84);
}
```

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

1. **使用过滤器**: `With<T>` 和 `Without<T>` 过滤器
2. **缓存计算结果**: 使用资源存储计算结果
3. **批量操作**: 使用 `Query::iter_mut` 批量处理实体
4. **避免频繁分配**: 重用 Vec 和 String
5. **使用 Change Detection**: `Changed<T>` 只处理变更的组件

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

在开发模式下添加 Bevy Inspector 插件查看实体和组件状态。

## 参考资料

- [Bevy 官方文档](https://bevyengine.org/learn/book/)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
- [Rust 官方文档](https://doc.rust-lang.org/)
