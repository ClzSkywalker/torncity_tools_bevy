use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QueueStrategy {
    #[default]
    Queue,
    Replace,
    Drop,
    Priority,
}

#[derive(Clone, Debug)]
pub struct DedupConfig {
    pub enabled: bool,
    pub dedup_by: DedupBy,
    pub time_window: Duration,
}

impl Default for DedupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            dedup_by: DedupBy::Content,
            time_window: Duration::from_secs(2),
        }
    }
}

impl DedupConfig {
    pub fn exact_match(window_secs: f32) -> Self {
        Self {
            enabled: true,
            dedup_by: DedupBy::Content,
            time_window: Duration::from_secs_f32(window_secs),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DedupBy {
    Content,
    ContentPrefix,
    Channel,
}

#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    pub max_per_second: usize,
    pub burst: usize,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_per_second: 3,
            burst: 5,
        }
    }
}
