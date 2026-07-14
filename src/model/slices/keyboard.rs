use super::rotation::{
    action::{Axis, Change, RotationAction},
    slice::Direction,
};
use crate::redux::{ActionVariant, BoxedActionVariant, Dispatcher, State};
use sdl2::keyboard::Keycode;

pub enum KeyMotion {
    Up,
    Down,
}

pub struct KeyInputAction {
    motion: KeyMotion,
    key: Keycode,
}

impl ActionVariant for KeyInputAction {
    fn reduce(&self, timestamp: u32, _state: &mut State, dispatcher: &mut Dispatcher) {
        match self.motion {
            KeyMotion::Down => handle_key_down(dispatcher, timestamp, self.key),
            KeyMotion::Up => handle_key_up(dispatcher, timestamp, self.key),
        }
    }
}

impl KeyInputAction {
    pub fn new(key: Keycode, motion: KeyMotion) -> BoxedActionVariant {
        Box::new(Self { key, motion })
    }
}

fn key_to_axis_direction(key: Keycode) -> (Axis, Direction) {
    match key {
        Keycode::Left => (Axis::Z, Direction::Positive),
        Keycode::Right => (Axis::Z, Direction::Negative),
        Keycode::Up => (Axis::X, Direction::Positive),
        Keycode::Down => (Axis::X, Direction::Negative),
        _ => panic!("unexpected keycode"),
    }
}

fn handle_key_down(dispatcher: &mut Dispatcher, timestamp: u32, key: Keycode) {
    let (axis, direction) = key_to_axis_direction(key);

    dispatcher.dispatch(
        timestamp,
        RotationAction::new(axis, direction, Change::Start),
    );
}

fn handle_key_up(dispatcher: &mut Dispatcher, timestamp: u32, key: Keycode) {
    let (axis, direction) = key_to_axis_direction(key);

    dispatcher.dispatch(
        timestamp,
        RotationAction::new(axis, direction, Change::Stop),
    );
}
