use crate::{scene::Vertex, space::Vector};

pub struct Translation(Vector);

impl Translation {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vector::new(x, y, z))
    }

    pub fn apply(&self, vertex: Vertex) -> Vertex {
        let v = &self.0;

        let x = vertex.x() + v.x();
        let y = vertex.y() + v.y();
        let z = vertex.z() + v.z();

        Vertex::new(x, y, z)
    }
}
