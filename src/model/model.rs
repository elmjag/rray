use super::{
    pump::Pump,
    slices::{self, application::ApplicationSlice, rotation::slice::RotationSlice},
};
use crate::{
    loader,
    redux::{State, Store},
    scene::{Camera, Mesh, Object, Rotation, Scene, Transform, Translation},
    timer::Timer,
    vector::Vector,
};
use sdl2::Sdl;
use std::path::PathBuf;

pub struct Model {
    pump: Pump,
    store: Store,
    camera: Camera,
    directional_light: Vector,
    mesh: Mesh,
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
    pub fn new(sdl_context: &Sdl, scene_file: Option<PathBuf>) -> Result<Self, String> {
        let store = Store::new(vec![
            (slices::ROTATION_X, RotationSlice::new(-0.8)),
            (slices::ROTATION_Z, RotationSlice::new(0.0)),
            (slices::APPLICATION, ApplicationSlice::new()),
        ]);

        let (camera, mesh) = loader::load_scene(scene_file)?;

        Ok(Self {
            pump: Pump::new(sdl_context),
            store,
            camera,
            // hard-coded directional light
            directional_light: Vector::new(0.1, -0.1, 1.0).normilize(),
            mesh,
        })
    }

    pub fn canvas_size(&self) -> (u32, u32) {
        self.camera.canvas_size()
    }

    pub fn get_scene(&mut self, timestamp: u32) -> (Scene<'_>, bool) {
        let snapshot = self.store.get_snapshot(timestamp);
        let transform = get_transform(&snapshot);

        let object = Object::new(&self.mesh, transform);
        let scene = Scene::new(&self.camera, &self.directional_light, object);

        (scene, get_terminated(&snapshot))
    }

    pub fn process_events(&mut self, fps_timer: &Timer) {
        self.pump.pump_events(&mut self.store, fps_timer);
        self.store.process();
    }
}
