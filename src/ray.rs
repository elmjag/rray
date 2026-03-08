use crate::space::{Vector, Vertex};

#[derive(Debug, PartialEq)]
pub struct Ray {
    orig: Vertex,
    direction: Vector,
}

impl Ray {
    pub fn new(orig: Vertex, direction: Vector) -> Ray {
        Ray { orig, direction }
    }

    pub fn orig(&self) -> &Vertex {
        &self.orig
    }

    pub fn direction(&self) -> &Vector {
        &self.direction
    }
}
