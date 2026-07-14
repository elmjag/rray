use super::slice::{Direction, Motion, RotationSlice};
use crate::{
    model::slices,
    redux::{ActionVariant, BoxedActionVariant, Dispatcher, State},
};

pub enum Axis {
    X,
    Z,
}

pub enum Change {
    Start,
    Stop,
}

pub struct RotationAction {
    axis: Axis,
    direction: Direction,
    change: Change,
}

fn handle_stop_rotation(timestamp: u32, direction: Direction, slice: &mut RotationSlice) {
    if let Motion::Rotating {
        direction: current_direction,
        ..
    } = slice.motion()
        && current_direction == direction
    {
        // only stop rotation, when released key is the one the started rotation
        slice.stop_rotation(timestamp);
    }
}

impl ActionVariant for RotationAction {
    fn reduce(&self, timestamp: u32, state: &mut State, _: &mut Dispatcher) {
        let slice_id = match self.axis {
            Axis::X => slices::ROTATION_X,
            Axis::Z => slices::ROTATION_Z,
        };
        let slice = state.get_slice_mut::<RotationSlice>(slice_id);

        match self.change {
            Change::Start => slice.start_rotation(timestamp, self.direction),
            Change::Stop => handle_stop_rotation(timestamp, self.direction, slice),
        }
    }
}

impl RotationAction {
    pub fn new(axis: Axis, direction: Direction, change: Change) -> BoxedActionVariant {
        Box::new(Self {
            axis,
            direction,
            change,
        })
    }
}
