use crate::{
    scene::{Camera, Object},
    vector::Vector,
};

pub struct Scene<'a> {
    camera: &'a Camera,
    directional_light: &'a Vector,
    object: Object<'a>,
}

impl<'a> Scene<'a> {
    pub fn new(camera: &'a Camera, directional_light: &'a Vector, object: Object<'a>) -> Self {
        Self {
            camera,
            directional_light,
            object,
        }
    }

    //
    // getters
    //

    pub fn camera(&self) -> &Camera {
        self.camera
    }

    pub fn directional_light(&self) -> &Vector {
        &self.directional_light
    }

    pub fn object(&self) -> &Object<'a> {
        &self.object
    }
}
