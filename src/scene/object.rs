use crate::{
    scene::Face,
    scene::{Mesh, Transform},
};

pub struct Object<'a> {
    mesh: &'a Mesh,
    transform: Transform,
}

impl<'a> Object<'a> {
    pub fn new(mesh: &'a Mesh, transform: Transform) -> Self {
        Self { mesh, transform }
    }

    pub fn get_faces(&self) -> Vec<Face> {
        self.mesh.get_faces(&self.transform)
    }
}
