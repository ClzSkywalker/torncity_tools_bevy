# Bevy Theme

一个为 Bevy 引擎设计的通用主题插件，提供多主题支持和动态主题切换功能。

## 特性

- 4 个预设主题：MidnightEmerald、OceanFrost、SunsetOrange、RoyalPurple
- 自定义主题支持（Builder 模式）
- 运行时主题切换
- 主题化组件：背景、边框、文字、状态

## 快速开始

```rust
use bevy::prelude::*;
use bevy_theme::BevyThemePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyThemePlugin::new(bevy_theme::ThemePreset::MidnightEmerald))
        .run();
}
```

## 使用方式

### 方式一：使用主题组件（推荐）

主题组件会自动将颜色应用到 UI 元素上。**必须同时添加对应的颜色组件（如 BackgroundColor），否则主题系统无法工作。**

```rust
use bevy_theme::prelude::*;

// 在实体上同时添加主题组件和颜色组件
commands.spawn((
    // 主题组件 - 标记该元素使用主题
    ThemedBackground::primary(),
    
    // 颜色组件 - 必须添加！bevy_theme 会更新这个组件的颜色
    BackgroundColor(Color::BLACK),
    
    // 其他 UI 组件...
    Node { ... },
));
```

#### 可用的主题组件

| 组件 | 用途 | 可选层 |
|------|------|--------|
| `ThemedBackground` | 背景颜色 | `primary()`, `secondary()`, `tertiary()`, `deep()`, `elevated()` |
| `ThemedBorder` | 边框颜色 | `subtle()`, `default()`, `active()` |
| `ThemedText` | 文字颜色 | `primary()`, `secondary()`, `muted()` |
| `ThemedState` | 状态颜色 | `success()`, `warning()`, `error()`, `info()`, `primary()` |

#### 重要提示

1. **必须添加对应的颜色组件**：使用 `ThemedBackground` 时必须添加 `BackgroundColor`，使用 `ThemedText` 时必须添加 `TextColor`
2. **颜色初始值不重要**：初始颜色会被主题系统覆盖，通常使用 `Color::BLACK` 或任意颜色作为占位符
3. **主题切换时自动更新**：当主题改变时，`apply_theme_to_components` 系统会自动更新所有主题组件的颜色
4. **动态切换状态**：可以通过修改 `ThemedBackground.layer` 来切换不同的背景层

### 方式二：手动获取主题颜色

如果需要更精细的控制，可以直接获取主题颜色：

```rust
fn my_system(theme: Res<bevy_theme::Theme>) {
    let colors = theme.colors();
    
    // 使用颜色
    colors.primary;           // 主色
    colors.bg_primary;       // 主背景
    colors.text_secondary;   // 次要文字
}
```

### 主题切换

```rust
fn switch_to_ocean_theme(app: &mut App) {
    app.set_theme(bevy_theme::ThemePreset::OceanFrost);
}
```

### 自定义主题

```rust
let custom = bevy_theme::CustomTheme::builder()
    .name("My Theme")
    .primary(Color::srgb(1.0, 0.2, 0.4))
    .dark_mode()
    .build();

theme.set_custom(custom);
```

## 主题色板

| 颜色 | 用途 |
|------|------|
| `primary` | 主色（按钮、边框） |
| `primary_hover` | 主色悬停 |
| `primary_active` | 主色激活 |
| `secondary` | 次要色 |
| `bg_deep` | 最深背景 |
| `bg_primary` | 主背景 |
| `bg_secondary` | 卡片背景 |
| `bg_tertiary` | 输入框背景 |
| `bg_elevated` | 高亮背景 |
| `text_primary` | 主要文字 |
| `text_secondary` | 次要文字 |
| `text_muted` | 辅助文字 |
| `border_subtle` | 微弱边框 |
| `border_default` | 默认边框 |
| `border_active` | 激活边框 |
| `success` | 成功状态 |
| `warning` | 警告状态 |
| `error` | 错误状态 |
| `info` | 信息状态 |

## 预设主题预览

| 主题 | 主色 |
|------|------|
| MidnightEmerald | 🟢 #00D084 |
| OceanFrost | 🔵 #38BDF8 |
| SunsetOrange | 🟠 #FA7A21 |
| RoyalPurple | 🟣 #8C47FA |

## License

MIT
