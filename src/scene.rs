use crate::{
    camera::Camera,
    mesh::{Face, Mesh},
    transform::Transform,
};

pub struct Scene<'a> {
    camera: &'a Camera,
    object: Object<'a>,
}

pub struct Object<'a> {
    mesh: &'a Mesh,
    transform: Transform,
}

impl<'a> Scene<'a> {
    pub fn new(camera: &'a Camera, object: Object<'a>) -> Self {
        Self { camera, object }
    }

    //
    // getters
    //

    pub fn object(&self) -> &Object<'a> {
        &self.object
    }

    pub fn camera(&self) -> &Camera {
        self.camera
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
