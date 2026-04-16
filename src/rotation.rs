use crate::{
    quaternion::Quaternion,
    space::{Vector, Vertex},
};

pub struct Rotation {
    rotor: Quaternion,
    inv_rotor: Quaternion,
}

impl From<&Vertex> for Quaternion {
    fn from(v: &Vertex) -> Self {
        Quaternion::new(0.0, v.x(), v.y(), v.z())
    }
}

impl From<Quaternion> for Vertex {
    fn from(q: Quaternion) -> Self {
        Vertex::new(q.x(), q.y(), q.z())
    }
}

impl Rotation {
    pub fn new(angle: f32, axis: &Vector) -> Rotation {
        assert!(axis.is_unit(), "non-unit rotation axis vector");

        let half_angle = angle / 2.0;
        let sin = half_angle.sin();

        let rotor = Quaternion::new(
            half_angle.cos(),
            axis.x() * sin,
            axis.y() * sin,
            axis.z() * sin,
        );
        let inv_rotor = rotor.inverse();

        Rotation {
            rotor: rotor,
            inv_rotor: inv_rotor,
        }
    }

    pub fn apply(&self, vertex: &Vertex) -> Vertex {
        let q = Quaternion::from(vertex);
        let t = &self.rotor * &q;
        let r = &t * &self.inv_rotor;

        r.into()
    }
}
