use std::time::{Duration, Instant};

pub struct Timer {
    pub last_instant: Instant,
    pub delta: Duration,
    pub period: Duration,
    pub coutdown: Duration,
    pub is_ready: bool,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(200),
            coutdown: Duration::default(),
            is_ready: true,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.coutdown = self.coutdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.is_ready = true;
            self.period
        });
    }
}
