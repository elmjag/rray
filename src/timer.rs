use sdl2::{Sdl, TimerSubsystem};

struct Stats {
    frame_times: Vec<u32>,
}

impl Stats {
    fn new(fps_target: f32) -> Self {
        let slots = (fps_target as usize) + 1;
        Self {
            frame_times: Vec::with_capacity(slots),
        }
    }

    fn print_statistics(&self, first_ts: u32) {
        let mut prev_ts = first_ts;
        for ts in self.frame_times.iter().skip(1).copied() {
            print!(" {}", ts - prev_ts);
            prev_ts = ts;
        }

        let total_delta = (prev_ts - first_ts) as f32;
        let avarage_delta = total_delta / (self.frame_times.len() - 1) as f32;
        let ca_fps = 1000.0 / avarage_delta;

        println!("\n{avarage_delta:>6.2} ({ca_fps:>5.2} fps)");
    }

    fn rendering_finished(&mut self, now: u32) {
        self.frame_times.push(now);

        let first_ts = *self.frame_times.first().unwrap();
        if now - first_ts > 1000 {
            self.print_statistics(first_ts);
            self.frame_times.clear();
        }
    }
}

pub struct Timer {
    sdl_timer: TimerSubsystem,
    /// when rendering of last frame started
    frame_start: Option<u32>,
    /// time budget for rendering a frame,
    /// given our FPS target
    frame_time: u32,
    // store rendering times for frames
    stats: Option<Stats>,
}

impl Timer {
    pub fn new(sdl_context: &Sdl, fps_target: f32, show_fps_stats: bool) -> Self {
        let frame_time = (1.0 / fps_target) * 1000.0;

        let stats = if show_fps_stats {
            Some(Stats::new(fps_target))
        } else {
            None
        };

        Self {
            sdl_timer: sdl_context.timer().unwrap(),
            frame_start: None,
            frame_time: frame_time as u32,
            stats,
        }
    }

    /// get current global rendering time
    pub fn current_time(&self) -> u32 {
        self.sdl_timer.ticks()
    }

    pub fn start_frame(&mut self) {
        self.frame_start = Some(self.sdl_timer.ticks());
    }

    pub fn rendering_finished(&mut self) {
        if self.stats.is_none() {
            // printing FPS stats is disabled
            return;
        }

        let now = self.current_time();
        self.stats.as_mut().unwrap().rendering_finished(now);
    }

    pub fn remaining_frame_time(&self) -> u32 {
        if self.frame_start.is_none() {
            return 0;
        }

        let frame_finished = self.frame_start.unwrap() + self.frame_time;
        let now = self.current_time();

        if frame_finished <= now {
            0
        } else {
            frame_finished - now
        }
    }
}
