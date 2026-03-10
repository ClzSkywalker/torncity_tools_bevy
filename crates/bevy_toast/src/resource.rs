use std::time::Duration;

use bevy_ecs::prelude::*;
use bevy_color::prelude::*;
use bevy_ui::prelude::*;

use crate::layout::ToastPosition;
use crate::queue::{DedupConfig, QueueStrategy, RateLimitConfig};
use crate::style::ToastAnimation;

#[derive(Resource)]
pub struct ToastConfig {
    pub default_duration: Duration,
    pub max_visible: usize,
    pub max_queue_size: usize,
    pub default_position: ToastPosition,
    pub queue_strategy: QueueStrategy,
    pub dedup_config: DedupConfig,
    pub rate_limit: RateLimitConfig,
    pub debug_mode: bool,
}

impl Default for ToastConfig {
    fn default() -> Self {
        Self {
            default_duration: Duration::from_secs(2),
            max_visible: 5,
            max_queue_size: 20,
            default_position: ToastPosition::BottomCenter,
            queue_strategy: QueueStrategy::Queue,
            dedup_config: DedupConfig::default(),
            rate_limit: RateLimitConfig::default(),
            debug_mode: false,
        }
    }
}

#[derive(Resource, Clone)]
pub struct ToastTheme {
    pub success_color: Color,
    pub error_color: Color,
    pub warning_color: Color,
    pub info_color: Color,
    pub background: Color,
    pub text_color: Color,
    pub corner_radius: Val,
    pub padding: Val,
    pub max_width: Val,
    pub max_lines: usize,
}

impl Default for ToastTheme {
    fn default() -> Self {
        Self {
            success_color: Color::srgb(0.2, 0.8, 0.2),
            error_color: Color::srgb(0.9, 0.2, 0.2),
            warning_color: Color::srgb(0.9, 0.8, 0.2),
            info_color: Color::srgb(0.2, 0.5, 0.9),
            background: Color::srgba(0.15, 0.15, 0.15, 0.95),
            text_color: Color::WHITE,
            corner_radius: Val::Px(8.0),
            padding: Val::Px(12.0),
            max_width: Val::Percent(80.0),
            max_lines: 2,
        }
    }
}

#[derive(Resource, Default)]
pub struct ToastThemeExtension(pub ToastTheme);

#[derive(Resource)]
pub struct ToastAnimationConfig {
    pub animation: ToastAnimation,
    pub default_duration: Duration,
}

impl Default for ToastAnimationConfig {
    fn default() -> Self {
        Self {
            animation: ToastAnimation::default(),
            default_duration: Duration::from_secs(2),
        }
    }
}
