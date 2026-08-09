use crate::scene::{Face, Transform, Vertex};
use sdl2::pixels::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Mesh {
    vertices: Vec<(f32, f32, f32)>,
    colors: Vec<(u8, u8, u8)>,
    /// triangle as indices into `vertices` and `colors` vectors, as follows:
    /// (vertex0, vertex1, vertex2, color)
    triangles: Vec<(usize, usize, usize, usize)>,
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
