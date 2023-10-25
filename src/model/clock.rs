use std::time::{Duration, Instant};

pub struct Clock {
    last_tick: Instant,
}

impl Clock {
    pub fn from_now() -> Self {
        Self {
            last_tick: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.last_tick.elapsed()
    }

    pub fn tick(&mut self) {
        self.last_tick = Instant::now();
    }
}
