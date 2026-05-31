use sdl2::{Sdl, TimerSubsystem};

pub struct Timer {
    sdl_timer: TimerSubsystem,
    /// when rendering of last frame started
    frame_start: Option<u32>,
    /// time budget for rendering a frame,
    /// given our FPS target
    frame_time: u32,
}

impl Timer {
    pub fn new(sdl_context: &Sdl, fps_target: f32) -> Self {
        let frame_time = (1.0 / fps_target) * 1000.0;

        Self {
            sdl_timer: sdl_context.timer().unwrap(),
            frame_start: None,
            frame_time: frame_time as u32,
        }
    }

    /// get current global rendering time
    pub fn elapsed_time(&self) -> u32 {
        self.sdl_timer.ticks()
    }

    pub fn start_frame(&mut self) {
        self.frame_start = Some(self.sdl_timer.ticks());
    }

    pub fn remaining_frame_time(&self) -> u32 {
        if self.frame_start.is_none() {
            return 0;
        }

        let frame_finished = self.frame_start.unwrap() + self.frame_time;
        let now = self.elapsed_time();

        if frame_finished <= now {
            0
        } else {
            frame_finished - now
        }
    }
}
