use bevy::prelude::*;

pub struct ButtonClickEffectPlugin;

impl Plugin for ButtonClickEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_button_click_effect,
                animate_button_click_effect,
            ),
        );
    }
}

/// 按钮点击效果配置组件
/// 添加此组件到带有 Button 和 BackgroundColor 的实体上，即可自动获得点击效果
#[derive(Component, Debug, Clone)]
pub struct ButtonClickEffect {
    /// 颜色变浅的程度 (0.0 - 1.0)，值越大变得越浅
    pub fade_amount: f32,
    /// 动画持续时间（秒）
    pub duration: f32,
    /// 原始颜色（自动保存）
    original_color: Option<Srgba>,
}

impl ButtonClickEffect {
    /// 创建一个新的按钮点击效果
    ///
    /// # 参数
    /// * `fade_amount` - 颜色变浅程度 (0.0 - 1.0)，建议值 0.3-0.5
    /// * `duration` - 动画持续时间（秒），建议值 0.1-0.3
    pub fn new(fade_amount: f32, duration: f32) -> Self {
        Self {
            fade_amount: fade_amount.clamp(0.0, 1.0),
            duration: duration.max(0.01),
            original_color: None,
        }
    }

    /// 创建默认配置（变浅 40%，持续 0.2 秒）
    pub fn default_config() -> Self {
        Self::new(0.5, 0.3)
    }

    /// 创建快速点击效果（变浅 30%，持续 0.15 秒）
    pub fn quick() -> Self {
        Self::new(0.3, 0.15)
    }

    /// 创建慢速点击效果（变浅 50%，持续 0.3 秒）
    pub fn slow() -> Self {
        Self::new(0.5, 0.3)
    }
}

impl Default for ButtonClickEffect {
    fn default() -> Self {
        Self::default_config()
    }
}

/// 内部组件：跟踪正在进行的点击动画
#[derive(Component)]
struct ClickAnimation {
    /// 动画开始时间
    start_time: f32,
    /// 原始颜色
    original_color: Srgba,
    /// 目标颜色（变浅后的颜色）
    target_color: Srgba,
    /// 动画持续时间
    duration: f32,
}

/// 系统：检测按钮点击并启动动画
fn handle_button_click_effect(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut buttons: Query<
        (
            Entity,
            &Interaction,
            &BackgroundColor,
            &mut ButtonClickEffect,
        ),
        (Changed<Interaction>, Without<ClickAnimation>),
    >,
) {
    for (entity, interaction, bg_color, mut effect) in buttons.iter_mut() {
        // 只在按钮被按下时触发
        if *interaction == Interaction::Pressed {
            let original = bg_color.0.to_srgba();

            // 保存原始颜色（首次点击时）
            if effect.original_color.is_none() {
                effect.original_color = Some(original);
            }

            // 计算目标颜色（向白色混合以实现变浅效果）
            let target = lighten_color(original, effect.fade_amount);

            commands.entity(entity).insert(ClickAnimation {
                start_time: time.elapsed_secs(),
                original_color: original,
                target_color: target,
                duration: effect.duration,
            });
        }
    }
}

/// 系统：执行点击动画
fn animate_button_click_effect(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut animating_buttons: Query<(Entity, &mut BackgroundColor, &ClickAnimation)>,
) {
    let current_time = time.elapsed_secs();

    for (entity, mut bg_color, animation) in animating_buttons.iter_mut() {
        let elapsed = current_time - animation.start_time;
        let progress = (elapsed / animation.duration).clamp(0.0, 1.0);

        if progress >= 1.0 {
            // 动画结束，恢复原始颜色
            bg_color.0 = animation.original_color.into();
            commands.entity(entity).remove::<ClickAnimation>();
        } else {
            // 使用缓动函数计算当前颜色
            // 前半段：原始 -> 变浅
            // 后半段：变浅 -> 原始
            let eased_progress = if progress < 0.5 {
                // 前半段：快速变浅
                ease_out_cubic(progress * 2.0)
            } else {
                // 后半段：快速恢复
                1.0 - ease_out_cubic((progress - 0.5) * 2.0)
            };

            bg_color.0 = interpolate_color(
                animation.original_color,
                animation.target_color,
                eased_progress,
            )
            .into();
        }
    }
}

/// 工具函数：将颜色变浅
fn lighten_color(color: Srgba, amount: f32) -> Srgba {
    let r = color.red + (1.0 - color.red) * amount;
    let g = color.green + (1.0 - color.green) * amount;
    let b = color.blue + (1.0 - color.blue) * amount;
    Srgba::new(r, g, b, color.alpha)
}

/// 工具函数：在两个颜色之间插值
fn interpolate_color(from: Srgba, to: Srgba, t: f32) -> Srgba {
    Srgba::new(
        from.red + (to.red - from.red) * t,
        from.green + (to.green - from.green) * t,
        from.blue + (to.blue - from.blue) * t,
        from.alpha + (to.alpha - from.alpha) * t,
    )
}

/// 缓动函数：三次方缓出
fn ease_out_cubic(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_click_effect_creation() {
        let effect = ButtonClickEffect::new(0.5, 0.2);
        assert_eq!(effect.fade_amount, 0.5);
        assert_eq!(effect.duration, 0.2);
    }

    #[test]
    fn test_button_click_effect_clamping() {
        let effect = ButtonClickEffect::new(1.5, -0.1);
        assert_eq!(effect.fade_amount, 1.0); // 被限制在 0.0-1.0 范围内
        assert_eq!(effect.duration, 0.01); // 最小值
    }

    #[test]
    fn test_lighten_color() {
        let original = Srgba::new(0.5, 0.5, 0.5, 1.0);
        let lightened = lighten_color(original, 0.5);

        // 应该向白色移动 50%
        assert!((lightened.red - 0.75).abs() < 0.01);
        assert!((lightened.green - 0.75).abs() < 0.01);
        assert!((lightened.blue - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_color_interpolation() {
        let black = Srgba::new(0.0, 0.0, 0.0, 1.0);
        let white = Srgba::new(1.0, 1.0, 1.0, 1.0);

        let mid = interpolate_color(black, white, 0.5);
        assert!((mid.red - 0.5).abs() < 0.01);
        assert!((mid.green - 0.5).abs() < 0.01);
        assert!((mid.blue - 0.5).abs() < 0.01);
    }
}
