use crate::space::{Vector, Vertex};
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
    #[serde(skip)]
    faces: Option<Vec<Face>>,
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
            faces: None,
        }
    }

    fn build_faces(&self) -> Vec<Face> {
        let colors: Vec<Color> = self
            .colors
            .iter()
            .map(|v| {
                let (r, g, b) = *v;
                Color::RGB(r, g, b)
            })
            .collect();

        let mut faces = Vec::with_capacity(self.triangles.len());

        for indices in &self.triangles {
            let face = Face::new(
                Vertex::from(self.vertices[indices.0]),
                Vertex::from(self.vertices[indices.1]),
                Vertex::from(self.vertices[indices.2]),
                colors[indices.3],
            );
            faces.push(face);
        }

        faces
    }

    pub fn get_faces(&mut self) -> &Vec<Face> {
        if self.faces.is_none() {
            self.faces = Some(self.build_faces());
        }

        self.faces.as_ref().unwrap()
    }
}
