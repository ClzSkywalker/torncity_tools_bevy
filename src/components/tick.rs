use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Reflect)]
pub enum CountDownType {
    // just once,support reset
    #[default]
    Once,
    // auto reset
    Repeat,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Reflect)]
pub enum CountDownState {
    #[default]
    Running,
    Paused,
    Ended,
    Restore,
}

#[derive(Debug, Clone, Copy, Reflect)]
struct CountDownBackup {
    count: f32,
    target: f32,
    pause: bool,
    count_type: CountDownType,
}

#[derive(Debug, Clone, Default, Resource, Component, Reflect)]
pub struct CountDownComp<T> {
    pub count: f32,
    pub target: f32,
    pub pause: bool,
    pub count_type: CountDownType,
    #[reflect(ignore)]
    phantom: core::marker::PhantomData<T>,
    #[reflect(ignore)]
    bak: Option<CountDownBackup>,
}

impl<T> CountDownComp<T> {
    pub fn new(target: f32) -> Self {
        CountDownComp {
            count: 0.0,
            pause: false,
            target,
            count_type: CountDownType::Once,
            phantom: core::marker::PhantomData,
            bak: None,
        }
    }

    pub fn set_type(mut self, count_type: CountDownType) -> Self {
        self.count_type = count_type;
        self
    }

    pub fn set_pause(mut self, pause: bool) -> Self {
        self.pause = pause;
        self
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Create a backup of current state
    pub fn backup(&mut self) {
        self.bak = Some(CountDownBackup {
            count: self.count,
            target: self.target,
            pause: self.pause,
            count_type: self.count_type,
        });
    }

    pub fn restore(&mut self) {
        if let Some(bak) = self.bak {
            self.count = bak.count;
            self.target = bak.target;
            self.pause = bak.pause;
            self.count_type = bak.count_type;
        }
    }

    pub fn ready(&self) -> bool {
        self.count == 0.0 && !self.pause
    }

    pub fn is_end(&self) -> bool {
        self.count >= self.target
    }

    pub fn is_pause(&self) -> bool {
        self.pause
    }

    pub fn pause(&mut self) {
        self.pause = true;
    }

    pub fn unpause(&mut self) {
        self.pause = false;
    }

    pub fn tick(&mut self, delta: f32) -> CountDownState {
        if self.is_pause() {
            return CountDownState::Paused;
        }
        if self.is_end() && self.count_type == CountDownType::Repeat {
            self.restore();
            return CountDownState::Restore;
        }
        if self.is_end() && self.count_type == CountDownType::Once {
            return CountDownState::Ended;
        }
        self.count += delta;
        CountDownState::Running
    }
}
