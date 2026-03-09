use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::queue::strategy::DedupBy;

pub struct DedupTracker {
    seen: VecDeque<(String, Instant)>,
    time_window: Duration,
    dedup_by: DedupBy,
}

impl DedupTracker {
    pub fn new(time_window: Duration, dedup_by: DedupBy) -> Self {
        Self {
            seen: VecDeque::new(),
            time_window,
            dedup_by,
        }
    }

    pub fn should_show(&mut self, content: &str, channel: &str) -> bool {
        let now = Instant::now();

        self.seen.retain(|(_, time)| now.duration_since(*time) < self.time_window);

        let key = match self.dedup_by {
            DedupBy::Content => content.to_string(),
            DedupBy::ContentPrefix => {
                content.split('\n').next().unwrap_or(content).to_string()
            }
            DedupBy::Channel => format!("{}:{}", channel, content),
        };

        if self.seen.iter().any(|(k, _)| k == &key) {
            return false;
        }

        self.seen.push_back((key, now));
        true
    }

    pub fn clear(&mut self) {
        self.seen.clear();
    }
}
