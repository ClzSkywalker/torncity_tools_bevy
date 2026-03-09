use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Animation {
    Fade { duration: Duration },
    Slide {
        direction: SlideDirection,
        distance: f32,
        duration: Duration,
    },
    Scale {
        from: f32,
        to: f32,
        duration: Duration,
    },
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlideDirection {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ToastAnimation {
    pub appear: Animation,
    pub disappear: Animation,
}

impl Default for ToastAnimation {
    fn default() -> Self {
        Self {
            appear: Animation::Fade {
                duration: Duration::from_millis(200),
            },
            disappear: Animation::Fade {
                duration: Duration::from_millis(150),
            },
        }
    }
}

fn ms(ms: u64) -> Duration {
    Duration::from_millis(ms)
}
