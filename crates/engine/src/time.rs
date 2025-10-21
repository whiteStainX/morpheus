use std::time::Instant;

pub struct Clock {
    last: Instant,
    pub dt: f32,
}

impl Clock {
    pub fn new(dt: f32) -> Self {
        Self { last: Instant::now(), dt }
    }

    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        let elapsed = (now - self.last).as_secs_f32();
        self.last = now;
        elapsed
    }
}
