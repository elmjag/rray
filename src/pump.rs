use crate::timer::FpsTimer;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl, event::Event};

pub struct Pump {
    event_pump: EventPump,
}

impl Pump {
    pub fn init(sdl_context: &Sdl) -> Pump {
        let event_pump = sdl_context.event_pump().unwrap();
        Pump { event_pump }
    }

    pub fn terminated(&mut self, fps_timer: &FpsTimer) -> bool {
        loop {
            let sleep = fps_timer.get_fps_sleep();
            let maybe_event = self.event_pump.wait_event_timeout(sleep);

            if let Some(event) = maybe_event {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        return true;
                    }
                    _ => {}
                }
            } else if sleep == 0 {
                return false;
            }
        }
    }
}
