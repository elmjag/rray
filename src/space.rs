use crate::math::is_close;
use std::ops::{Index, Sub};

pub const Z_AXIS: Vector = Vector::new(0.0, 0.0, 1.0);
pub const ZERO_VECTOR: Vector = Vector::new(0.0, 0.0, 0.0);

#[derive(Debug, PartialEq)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Index<Axis> for Vector {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        self.get_axis_val(index)
    }
}

impl Index<Axis> for &Vector {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        self.get_axis_val(index)
    }
}

impl Vector {
    pub const fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    fn get_axis_val(&self, index: Axis) -> &f32 {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
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

    pub fn normilize(&self) -> Vector {
        let magnitude = self.magnitude();

        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    ///
    /// dot product between self and rhs
    ///
    pub fn dot(&self, rhs: &Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    ///
    /// vector's magnitude (aka length, norm)
    ///
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    ///
    /// is true if this a unit vector, that is it have norm/magnitude of 1.0
    ///
    pub fn is_unit(&self) -> bool {
        is_close(self.magnitude(), 1.0)
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
