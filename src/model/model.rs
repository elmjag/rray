use super::{
    pump::Pump,
    slices::{self, application::ApplicationSlice, rotation::slice::RotationSlice},
};
use crate::{
    redux::{State, Store},
    rotation::Rotation,
    space::Vector,
    timer::Timer,
    transform::Transform,
    translation::Translation,
};
use sdl2::Sdl;

pub struct Model {
    pump: Pump,
    store: Store,
    terminated: bool,
}

fn get_rotation(state: &State) -> Rotation {
    let slice_z = state.get_slice::<RotationSlice>(slices::ROTATION_Z);
    let rotation_z = Rotation::new(slice_z.angle(), &Vector::new(0.0, 0.0, -1.0));

    let slice_x = state.get_slice::<RotationSlice>(slices::ROTATION_X);
    let rotation_x = Rotation::new(slice_x.angle(), &Vector::new(1.0, 0.0, 0.0));

    &rotation_x * &rotation_z
}

fn get_transform(state: &State) -> Transform {
    Transform::new(Translation::new(0.0, 0.0, -8.0), get_rotation(state))
}

fn get_terminated(state: &State) -> bool {
    state
        .get_slice::<ApplicationSlice>(slices::APPLICATION)
        .is_terminated()
}

impl Model {
    pub fn new(sdl_context: &Sdl) -> Self {
        let store = Store::new(vec![
            (slices::ROTATION_X, RotationSlice::new()),
            (slices::ROTATION_Z, RotationSlice::new()),
            (slices::APPLICATION, ApplicationSlice::new()),
        ]);

        Self {
            pump: Pump::new(sdl_context),
            terminated: false,
            store,
        }
    }

    pub fn process_events(&mut self, fps_timer: &Timer) -> Transform {
        self.pump.pump_events(&mut self.store, fps_timer);

        let snapshot = self.store.process(fps_timer.elapsed_time());
        self.terminated = get_terminated(&snapshot);

        get_transform(&snapshot)
    }

    pub fn application_terminated(&self) -> bool {
        self.terminated
    }
}
