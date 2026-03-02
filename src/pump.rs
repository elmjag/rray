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

    pub fn terminated(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
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
        }

        return false;
    }
}
