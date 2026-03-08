use crate::space::{Vector, Vertex};

#[derive(Debug)]
pub struct Face(Vertex, Vertex, Vertex);

pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<(usize, usize, usize)>,
    faces: Option<Vec<Face>>,
}

impl Face {
    pub fn new(vertex0: Vertex, vertex1: Vertex, vertex2: Vertex) -> Face {
        Face(vertex0, vertex1, vertex2)
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
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<(usize, usize, usize)>) -> Mesh {
        Mesh {
            vertices,
            triangles,
            faces: None,
        }
    }

    fn build_faces(&self) -> Vec<Face> {
        let mut faces = Vec::with_capacity(self.triangles.len());

        for (idx0, idx1, idx2) in &self.triangles {
            let face = Face::new(
                self.vertices[*idx0].clone(),
                self.vertices[*idx1].clone(),
                self.vertices[*idx2].clone(),
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
