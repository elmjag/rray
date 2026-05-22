use std::time::{Duration, Instant};

pub struct Timer {
    /// global rendering time start
    start: Instant,
    /// when rendering of last frame started
    frame_start: Option<Instant>,
    /// time budget for rendering a frame,
    /// given our FPS target
    frame_time: Duration,
}

impl Timer {
    pub fn new(fps_target: f32) -> Self {
        let frame_time = (1.0 / fps_target) * 1000.0;

        Self {
            start: Instant::now(),
            frame_start: None,
            frame_time: Duration::from_millis(frame_time as u64),
        }
    }

    /// get current global rendering time
    pub fn elapsed_time(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn start_frame(&mut self) {
        self.frame_start = Some(Instant::now());
    }

    pub fn remaining_frame_time(&self) -> u32 {
        if self.frame_start.is_none() {
            return 0;
        }

        let elapsed = self.frame_start.unwrap().elapsed();

        let sleep_time = if elapsed >= self.frame_time {
            0
        } else {
            (self.frame_time - elapsed).as_millis() as u32
        };

        sleep_time
    }
}
