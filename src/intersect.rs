use crate::{
    mesh::Face,
    ray::Ray,
    space::{Axis, Vector},
};

///
/// Panic if Z-axis value is not the biggest one
///
fn assert_max_z(ray_direction: &Vector) {
    let (ax, ay, az) = (
        ray_direction.x().abs(),
        ray_direction.y().abs(),
        ray_direction.z().abs(),
    );
    assert!(az > ax && az > ay, "axis flipping not implemented");
}

fn get_axes(ray_direction: &Vector) -> (Axis, Axis, Axis) {
    if ray_direction.z() < 0.0 {
        // swap kx and ky dimension to
        // preserve winding direction of triangles
        return (Axis::Y, Axis::X, Axis::Z);
    }

    (Axis::X, Axis::Y, Axis::Z)
}
///
/// find possible intersection between a ray and a triangle face,
/// using Sven Woop's et al 'Watertight Ray/Triangle Intersection' algorithm
///
pub fn ray_triangle_intersection(ray: &Ray, face: &Face, last_t: f32) -> Option<f32> {
    let ray_direction = ray.direction();

    // for now, we assume z-axis is the maximal
    assert_max_z(ray.direction());

    let (kx, ky, kz) = get_axes(ray_direction);

    // calculate X- and Y-axis shear constants
    let sx = ray_direction[kx] / ray_direction[kz];
    let sy = ray_direction[ky] / ray_direction[kz];

    // calculate vertices relative to ray origin
    let a = face.v0() - ray.orig();
    let b = face.v1() - ray.orig();
    let c = face.v2() - ray.orig();

    // perform shear and scale of vertices
    let ax = a[kx] - sx * a[kz];
    let ay = a[ky] - sy * a[kz];
    let bx = b[kx] - sx * b[kz];
    let by = b[ky] - sy * b[kz];
    let cx = c[kx] - sx * c[kz];
    let cy = c[ky] - sy * c[kz];

    // calculate scaled barycentric coordinates
    let mut u = cx * by - cy * bx;
    let mut v = ax * cy - ay * cx;
    let mut w = bx * ay - by * ax;

    // perform edge tests
    if u < 0.0 || v < 0.0 || w < 0.0 {
        return None;
    }

    // fallback to test against edges using double precision
    if u == 0.0 || v == 0.0 || w == 0.0 {
        let u64 = cx as f64 * by as f64 - cy as f64 * bx as f64;
        let v64 = ax as f64 * cy as f64 - ay as f64 * cx as f64;
        let w64 = bx as f64 * ay as f64 - by as f64 * ax as f64;

        (u, v, w) = (u64 as f32, v64 as f32, w64 as f32)
    }

    // calculate determinant
    let det = u + v + w;
    if det == 0.0 {
        return None;
    }

    // calculate Z-axis shear constant
    let sz = 1.0 / ray.direction().z();

    // calculate scaled z−coordinates of vertices
    let az = sz * a[kz];
    let bz = sz * b[kz];
    let cz = sz * c[kz];

    // calculate the hit distance
    let t = u * az + v * bz + w * cz;

    // ignore this hit, if it's behind
    // or we already found one closer
    if t < 0.0 || t > last_t * det {
        return None;
    }

    // return normilized hit distance
    Some(t * (1.0 / det))
}
