pub mod dedup;
pub mod strategy;

use bevy_ecs::prelude::*;
use std::collections::{BinaryHeap, VecDeque};
use std::time::{Duration, Instant};

use crate::channel::ToastChannel;
use crate::events::ToastEvent;

pub use strategy::{DedupBy, DedupConfig, QueueStrategy, RateLimitConfig};
pub use dedup::DedupTracker;

#[derive(Resource)]
pub struct ToastQueue {
    waiting: VecDeque<ToastEvent>,
    visible_count: usize,
    max_visible: usize,
    max_queue_size: usize,
    strategy: QueueStrategy,
    dedup_config: DedupConfig,
    dedup_tracker: DedupTracker,
    rate_limit: RateLimitConfig,
    last_spawn_time: Option<Instant>,
    recent_spawn_count: usize,
}

impl Default for ToastQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl ToastQueue {
    pub fn new() -> Self {
        Self {
            waiting: VecDeque::new(),
            visible_count: 0,
            max_visible: 5,
            max_queue_size: 20,
            strategy: QueueStrategy::Queue,
            dedup_config: DedupConfig::default(),
            dedup_tracker: DedupTracker::new(
                Duration::from_secs(2),
                DedupBy::Content,
            ),
            rate_limit: RateLimitConfig::default(),
            last_spawn_time: None,
            recent_spawn_count: 0,
        }
    }

    pub fn push(&mut self, event: ToastEvent) -> bool {
        if !self.should_accept(&event) {
            return false;
        }

        if self.waiting.len() >= self.max_queue_size {
            match self.strategy {
                QueueStrategy::Drop => return false,
                QueueStrategy::Replace => {
                    self.waiting.pop_back();
                }
                QueueStrategy::Queue | QueueStrategy::Priority => {}
            }
        }

        self.waiting.push_back(event);
        true
    }

    pub fn pop(&mut self) -> Option<ToastEvent> {
        if self.visible_count >= self.max_visible {
            return None;
        }

        if !self.check_rate_limit() {
            return None;
        }

        let event = match self.strategy {
            QueueStrategy::Priority => {
                let mut heap: BinaryHeap<_> = self.waiting.iter().cloned().collect();
                heap.pop().and_then(|e| {
                    let idx = self
                        .waiting
                        .iter()
                        .position(|x| std::ptr::eq(x as *const _, &e as *const _));
                    idx.and_then(|i| self.waiting.remove(i))
                })
            }
            _ => self.waiting.pop_front(),
        };

        if event.is_some() {
            self.visible_count += 1;
            self.last_spawn_time = Some(Instant::now());
            self.recent_spawn_count += 1;
        }

        event
    }

    pub fn mark_shown(&mut self) {
        if self.visible_count > 0 {
            self.visible_count -= 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.waiting.is_empty()
    }

    pub fn len(&self) -> usize {
        self.waiting.len()
    }

    fn should_accept(&mut self, event: &ToastEvent) -> bool {
        if !event.channel.is_enabled() {
            return false;
        }

        if self.dedup_config.enabled {
            let content = match &event.content {
                crate::style::ToastContent::Text(s) => s.as_str(),
                crate::style::ToastContent::IconText { text, .. } => text.as_str(),
                crate::style::ToastContent::Custom(_) => return true,
            };
            let channel_name = match event.channel {
                ToastChannel::System => "system",
                ToastChannel::Combat => "combat",
                ToastChannel::Economy => "economy",
                ToastChannel::Custom(s) => s,
            };

            if !self.dedup_tracker.should_show(content, channel_name) {
                return false;
            }
        }

        true
    }

    fn check_rate_limit(&self) -> bool {
        if let Some(last_time) = self.last_spawn_time {
            let elapsed = Instant::now().duration_since(last_time);
            if elapsed.as_secs_f32() >= 1.0 {
                return true;
            }
            return self.recent_spawn_count < self.rate_limit.burst;
        }
        true
    }

    pub fn reset_rate_limit(&mut self) {
        self.recent_spawn_count = 0;
    }

    pub fn set_max_visible(&mut self, max: usize) {
        self.max_visible = max;
    }

    pub fn set_strategy(&mut self, strategy: QueueStrategy) {
        self.strategy = strategy;
    }

    pub fn set_dedup_config(&mut self, config: DedupConfig) {
        self.dedup_config = config.clone();
        self.dedup_tracker = DedupTracker::new(config.time_window, config.dedup_by);
    }
}

impl PartialEq for ToastEvent {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for ToastEvent {}

impl PartialOrd for ToastEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ToastEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
