use crate::{mesh::Face, ray::Ray};

///
/// find possible intersection between a ray and a triangle face,
/// using Möller-Trumbore algorithm.
///
pub fn ray_triangle_intersection(ray: &Ray, face: &Face) -> Option<(f32, f32, f32)> {
    let v0v1 = face.v0v1();
    let v0v2 = face.v0v2();
    let pvec = ray.direction().cross(&v0v2);
    let det = v0v1.dot(&pvec);

    if det < f32::EPSILON {
        // the triangle is back-facing
        return None;
    }

    let inv_det = 1.0 / det;

    let tvec = ray.orig() - face.v0();
    let u = tvec.dot(&pvec) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let qvec = tvec.cross(&v0v1);
    let v = ray.direction().dot(&qvec) * inv_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = v0v2.dot(&qvec) * inv_det;
    Some((u, v, t))
}
