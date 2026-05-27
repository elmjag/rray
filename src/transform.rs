use crate::{rotation::Rotation, space::Vertex, translation::Translation};

pub struct Transform {
    translation: Translation,
    rotation: Rotation,
}

impl Transform {
    pub fn new(translation: Translation, rotation: Rotation) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn apply(&self, vertex: Vertex) -> Vertex {
        self.translation.apply(self.rotation.apply(&vertex))
    }
}
