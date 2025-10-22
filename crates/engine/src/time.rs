use std::time::{Duration, Instant};
use std::thread;

pub struct Clock {
    last_frame_instant: Instant,
    target_frame_duration: Duration,
}

impl Clock {
    pub fn new(target_fps: f32) -> Self {
        let target_frame_duration = Duration::from_secs_f32(1.0 / target_fps);
        Self {
            last_frame_instant: Instant::now(),
            target_frame_duration,
        }
    }

    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        let mut elapsed = now.duration_since(self.last_frame_instant);

        // If we rendered faster than target, sleep to cap FPS
        if elapsed < self.target_frame_duration {
            let sleep_duration = self.target_frame_duration - elapsed;
            thread::sleep(sleep_duration);
            elapsed = self.target_frame_duration; // Cap elapsed to target duration
        }

        self.last_frame_instant = Instant::now(); // Update for next frame

        // Return the actual elapsed time after potential sleeping
        elapsed.as_secs_f32()
    }
}
