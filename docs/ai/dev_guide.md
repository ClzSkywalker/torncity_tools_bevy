# 开发指南

添加新功能的步骤。

## 添加新功能

1. 创建模块 → `src/xxx/`
2. 定义数据模型 → `src/model/`
3. 实现 Bevy 系统
4. 注册插件

## 添加 UI 组件

1. 在 `src/components/` 创建组件模块
2. 定义组件结构体 + 插件
3. 实现生成和更新系统
4. 在 ComponentsPlugin 注册

## 添加 HTTP 请求

1. 在 `src/http/` 创建请求模块
2. 定义 Req/Resp 事件
3. 实现发送和处理系统
4. 在 ViewPlugin 注册

## 添加资源

1. 在 `src/resource/` 定义资源结构体
2. 实现 Default
3. 插件中使用 `insert_resource()` 或 `init_resource()`

## 测试

```bash
cargo test                    # 全部
cargo test module_name        # 特定模块
```

详见 [bevy_ecs_guide.md](bevy_ecs_guide.md) 性能优化部分

## 构建

```bash
cargo build --release

# Android
./build/package/package.sh -p android -r release
```

详见 [CLAUDE_ARCHITECTURE.md](../CLAUDE_ARCHITECTURE.md) 了解数据流和核心资源
