use crate::{
    camera::Camera,
    mesh::{Face, Mesh},
    space::Vector,
    transform::Transform,
};

pub struct Scene<'a> {
    camera: &'a Camera,
    directional_light: &'a Vector,
    object: Object<'a>,
}

pub struct Object<'a> {
    mesh: &'a Mesh,
    transform: Transform,
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

impl<'a> Object<'a> {
    pub fn new(mesh: &'a Mesh, transform: Transform) -> Self {
        Self { mesh, transform }
    }

    pub fn get_faces(&self) -> Vec<Face> {
        self.mesh.get_faces(&self.transform)
    }
}
