use rray::{
    intersect::ray_triangle_intersection,
    ray::Ray,
    scene::{Face, Vertex},
    vector::Vector,
};
use sdl2::pixels::Color;

fn get_face() -> Face {
    Face::new(
        Vertex::new(-0.5, 0.0, 5.0),
        Vertex::new(0.0, 1.0, 5.0),
        Vertex::new(0.5, 0.0, 5.0),
        Color::GRAY,
    )
}

#[test]
fn hit() {
    let face = get_face();

    let ray = Ray::new(Vertex::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face, f32::MAX).is_some());

    let ray = Ray::new(Vertex::new(-0.2, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face, f32::MAX).is_some());
}

#[test]
fn miss() {
    let face = get_face();

    let ray = Ray::new(Vertex::new(0.0, 2.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face, f32::MAX).is_none());

    let ray = Ray::new(Vertex::new(1.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face, f32::MAX).is_none());

    let ray = Ray::new(Vertex::new(0.0, -0.5, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face, f32::MAX).is_none());
}

#[test]
fn backface_culling() {
    // face with a backside as seen from origo
    let face = Face::new(
        Vertex::new(0.5, 0.0, 5.0),
        Vertex::new(0.0, 1.0, 5.0),
        Vertex::new(-0.5, 0.0, 5.0),
        Color::GRAY,
    );

    let ray = Ray::new(Vertex::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face, f32::MAX).is_none());
}

///
/// Test that ray direction with negative value on Z axis works.
/// Negative Z-axis requires special handling for Sven's Watertight algorithm.
///
#[test]
fn negative_z_direction() {
    let face = get_face();

    let ray = Ray::new(Vertex::new(0.0, 0.0, 6.0), Vector::new(0.0, 0.0, -1.0));
    assert!(ray_triangle_intersection(&ray, &face, f32::MAX).is_none());
}
