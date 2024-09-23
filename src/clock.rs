use std::time::{Duration, Instant};

pub struct Clock {
    start: Instant,
    hz: f64,
}

impl Clock {
    pub fn init(frequency: f64) -> Self {
        Self {
            start: Instant::now(),
            hz: frequency,
        }
    }

    fn tick_duration(&self) -> Duration {
        Duration::from_secs_f64(1f64/self.hz)
    }

    fn next_tick(&self) -> Instant {
        self.start
            .checked_add(self.tick_duration())
            .expect("Clock out of bounds")
    }

    pub fn check(&mut self) -> bool {
        if self.start >= self.next_tick() {
            self.start = self.next_tick();
            true
        } else {
            false
        }
    }
}
