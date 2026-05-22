use crate::{rotation::Rotation, space::Vertex};

pub struct Transform {
    // currently only rotation supported
    rotation: Rotation,
}

impl Transform {
    pub fn new(rotation: Rotation) -> Self {
        Self { rotation }
    }

    pub fn apply(&self, vertex: Vertex) -> Vertex {
        self.rotation.apply(&vertex)
    }
}
