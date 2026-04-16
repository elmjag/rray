use std::ops::Mul;

pub struct Quaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Quaternion {
        Quaternion { w, x, y, z }
    }

    pub fn inverse(&self) -> Quaternion {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    //
    // x, y, z components 'reader' methods
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

impl Mul for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion::new(
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            self.w * rhs.x + self.x * rhs.w - self.y * rhs.z + self.z * rhs.y,
            self.w * rhs.y + self.x * rhs.z + self.y * rhs.w - self.z * rhs.x,
            self.w * rhs.z - self.x * rhs.y + self.y * rhs.x + self.z * rhs.w,
        )
    }
}
