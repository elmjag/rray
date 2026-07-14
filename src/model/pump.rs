use super::slices::{
    application::QuitApplicationAction,
    keyboard::{KeyInputAction, KeyMotion},
};
use crate::{redux::Store, timer::Timer};
use sdl2::{EventPump, Sdl, event::Event, keyboard::Keycode};

pub struct Pump {
    event_pump: EventPump,
}

impl Pump {
    pub fn new(sdl_context: &Sdl) -> Pump {
        let event_pump = sdl_context.event_pump().unwrap();
        Pump { event_pump }
    }

    pub fn pump_events(&mut self, store: &mut Store, fps_timer: &Timer) {
        loop {
            let sleep = fps_timer.remaining_frame_time();
            let maybe_event = self.event_pump.wait_event_timeout(sleep);

            if let Some(event) = maybe_event {
                handle_event(event, store);
            } else if sleep == 0 {
                return;
            }
        }
    }
}

fn handle_key_event(event: Event, store: &mut Store) {
    let (timestamp, key, repeat, motion) = match event {
        Event::KeyDown {
            timestamp,
            keycode,
            repeat,
            ..
        } => (timestamp, keycode, repeat, KeyMotion::Down),
        Event::KeyUp {
            timestamp,
            keycode,
            repeat,
            ..
        } => (timestamp, keycode, repeat, KeyMotion::Up),
        _ => panic!("unexpected event"),
    };

    if repeat {
        return;
    }

    let key = key.unwrap();
    match key {
        Keycode::Left | Keycode::Right | Keycode::Up | Keycode::Down => {
            store.dispatch(timestamp, KeyInputAction::new(key, motion))
        }
        Keycode::Escape => store.dispatch(timestamp, QuitApplicationAction::new()),
        _ => {
            /* ignore other keys */
            return;
        }
    };
}

pub fn handle_event(event: Event, store: &mut Store) {
    match event {
        Event::KeyDown { .. } | Event::KeyUp { .. } => handle_key_event(event, store),
        Event::Quit { timestamp } => store.dispatch(timestamp, QuitApplicationAction::new()),
        _ => { /* ignore */ }
    }
}
