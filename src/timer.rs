use std::time::{Duration, Instant};

pub struct FpsTimer {
    /// when rendering of last frame started
    last_frame: Option<Instant>,
    /// time budget for rendering a frame,
    /// given our FPS target
    frame_time: Duration,
}

impl FpsTimer {
    pub fn new(fps_target: f32) -> Self {
        let frame_time = (1.0 / fps_target) * 1000.0;

        Self {
            last_frame: None,
            frame_time: Duration::from_millis(frame_time as u64),
        }
    }

    pub fn get_frame_delta(&mut self) -> Duration {
        let time_delta = match self.last_frame {
            Some(last) => last.elapsed(),
            None => Duration::from_secs(0),
        };

        self.last_frame = Some(Instant::now());

        time_delta
    }

    pub fn get_fps_sleep(&self) -> u32 {
        if self.last_frame.is_none() {
            return 0;
        }

        let elapsed = self.last_frame.unwrap().elapsed();

        let sleep_time = if elapsed >= self.frame_time {
            0
        } else {
            (self.frame_time - elapsed).as_millis() as u32
        };

        sleep_time
    }
}
