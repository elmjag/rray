use crate::{
    space::{Vector, Vertex},
    transform::Transform,
};
use sdl2::pixels::Color;
use serde::Deserialize;

#[derive(Debug)]
pub struct Face {
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    // store normal, to avoid recalulating it
    normal: Vector,
    color: Color,
}

#[derive(Debug, Deserialize)]
pub struct Mesh {
    vertices: Vec<(f32, f32, f32)>,
    colors: Vec<(u8, u8, u8)>,
    /// triangle as indices into `vertices` and `colors` vectors, as follows:
    /// (vertex0, vertex1, vertex2, color)
    triangles: Vec<(usize, usize, usize, usize)>,
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

impl Mesh {
    pub fn new(
        colors: Vec<(u8, u8, u8)>,
        vertices: Vec<(f32, f32, f32)>,
        triangles: Vec<(usize, usize, usize, usize)>,
    ) -> Mesh {
        Mesh {
            colors,
            vertices,
            triangles,
        }
    }

    pub fn get_faces(&self, transform: &Transform) -> Vec<Face> {
        let colors: Vec<Color> = self
            .colors
            .iter()
            .map(|v| {
                let (r, g, b) = *v;
                Color::RGB(r, g, b)
            })
            .collect();

        let mut faces = Vec::with_capacity(self.triangles.len());

        let vertices: Vec<Vertex> = self
            .vertices
            .iter()
            .map(|coords| transform.apply(Vertex::from(*coords)))
            .collect();

        for indices in &self.triangles {
            let face = Face::new(
                vertices[indices.0].clone(),
                vertices[indices.1].clone(),
                vertices[indices.2].clone(),
                colors[indices.3],
            );
            faces.push(face);
        }

        faces
    }
}
