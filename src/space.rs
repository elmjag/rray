use std::ops::Sub;

#[derive(Debug, PartialEq)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    ///
    /// cross product between self and rhs
    ///
    pub fn cross(&self, rhs: &Vector) -> Vector {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    ///
    /// dot product between self and rhs
    ///
    pub fn dot(&self, rhs: &Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

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
