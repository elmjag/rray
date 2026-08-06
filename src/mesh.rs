use crate::{
    space::{Vector, Vertex},
    transform::Transform,
};
use sdl2::pixels::Color;
use serde::Deserialize;

#[derive(Debug)]
pub struct Face(Vertex, Vertex, Vertex, Color);

#[derive(Debug, Deserialize)]
pub struct Mesh {
    vertices: Vec<(f32, f32, f32)>,
    colors: Vec<(u8, u8, u8)>,
    /// triangle as indices into `vertices` and `colors` vectors, as follows:
    /// (vertex0, vertex1, vertex2, color)
    triangles: Vec<(usize, usize, usize, usize)>,
}

impl Face {
    pub fn new(vertex0: Vertex, vertex1: Vertex, vertex2: Vertex, color: Color) -> Face {
        Face(vertex0, vertex1, vertex2, color)
    }

    ///
    /// vertex 0
    ///
    pub fn v0(&self) -> &Vertex {
        &self.0
    }

    ///
    /// vertex 1
    ///
    pub fn v1(&self) -> &Vertex {
        &self.1
    }

    ///
    /// vertex 2
    ///
    pub fn v2(&self) -> &Vertex {
        &self.2
    }

    ///
    /// side A vector
    ///
    pub fn v0v1(&self) -> Vector {
        &self.1 - &self.0
    }

    ///
    /// side B vector
    ///
    pub fn v0v2(&self) -> Vector {
        &self.2 - &self.0
    }

    pub fn color(&self) -> Color {
        self.3
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
