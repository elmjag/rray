use crate::{
    scene::{Color, Vertex},
    vector::Vector,
};

#[derive(Debug)]
pub struct Face {
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    // store normal, to avoid recalulating it
    normal: Vector,
    color: Color,
}

fn calculate_face_normal(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Vector {
    let side_a = v1 - v0;
    let side_b = v2 - v0;

    side_a.cross(&side_b).normilize()
}

impl Face {
    pub fn new(vertex0: Vertex, vertex1: Vertex, vertex2: Vertex, color: Color) -> Face {
        let normal = calculate_face_normal(&vertex0, &vertex1, &vertex2);
        Face {
            v0: vertex0,
            v1: vertex1,
            v2: vertex2,
            normal,
            color,
        }
    }

    ///
    /// vertex 0
    ///
    pub fn v0(&self) -> &Vertex {
        &self.v0
    }

    ///
    /// vertex 1
    ///
    pub fn v1(&self) -> &Vertex {
        &self.v1
    }

    ///
    /// vertex 2
    ///
    pub fn v2(&self) -> &Vertex {
        &self.v2
    }

    pub fn normal(&self) -> &Vector {
        &self.normal
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
