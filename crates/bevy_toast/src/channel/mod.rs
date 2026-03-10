use bevy_ecs::prelude::*;
use std::collections::HashMap;

use crate::layout::ToastPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToastChannel {
    System,
    Combat,
    Economy,
    Custom(&'static str),
}

impl ToastChannel {
    pub fn custom(name: &'static str) -> Self {
        Self::Custom(name)
    }

    pub fn is_enabled(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToastPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for ToastPriority {
    fn default() -> Self {
        ToastPriority::Normal
    }
}

#[derive(Clone, Debug)]
pub struct ChannelConfig {
    pub enabled: bool,
    pub position: ToastPosition,
    pub max_visible: usize,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            position: ToastPosition::BottomCenter,
            max_visible: 3,
        }
    }
}

#[derive(Resource, Default)]
pub struct ToastChannels {
    channels: HashMap<ToastChannel, ChannelConfig>,
}

impl ToastChannels {
    pub fn new() -> Self {
        let mut channels = HashMap::new();
        channels.insert(ToastChannel::System, ChannelConfig::default());
        channels.insert(ToastChannel::Combat, ChannelConfig::default());
        channels.insert(ToastChannel::Economy, ChannelConfig::default());
        Self { channels }
    }

    pub fn is_enabled(&self, channel: &ToastChannel) -> bool {
        self.channels
            .get(channel)
            .map(|c| c.enabled)
            .unwrap_or(true)
    }

    pub fn set_enabled(&mut self, channel: ToastChannel, enabled: bool) {
        if let Some(config) = self.channels.get_mut(&channel) {
            config.enabled = enabled;
        } else {
            self.channels.insert(channel, ChannelConfig {
                enabled,
                ..Default::default()
            });
        }
    }

    pub fn get_config(&self, channel: &ToastChannel) -> ChannelConfig {
        self.channels.get(channel).cloned().unwrap_or_default()
    }

    pub fn set_position(&mut self, channel: ToastChannel, position: ToastPosition) {
        if let Some(config) = self.channels.get_mut(&channel) {
            config.position = position;
        }
    }
}
