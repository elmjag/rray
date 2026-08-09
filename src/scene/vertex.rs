use crate::vector::Vector;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { x, y, z }
    }

    /// Create vertex from a tuple of (x, y, z) coordinates.
    pub fn from(coordinates: (f32, f32, f32)) -> Vertex {
        let (x, y, z) = coordinates;
        Vertex { x, y, z }
    }

    //
    // x, y, z coordinates 'reader' methods
    //
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

///
/// subtracting vertices a - b gives you a vector for moving
/// from b to a
///
impl Sub for &Vertex {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
