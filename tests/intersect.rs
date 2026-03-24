use rray::{
    intersect::ray_triangle_intersection,
    mesh::Face,
    ray::Ray,
    space::{Vector, Vertex},
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
    assert!(ray_triangle_intersection(&ray, &face).is_some());

    let ray = Ray::new(Vertex::new(-0.2, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face).is_some());
}

#[test]
fn miss() {
    let face = get_face();

    let ray = Ray::new(Vertex::new(0.0, 2.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face).is_none());

    let ray = Ray::new(Vertex::new(1.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face).is_none());

    let ray = Ray::new(Vertex::new(0.0, -0.5, 0.0), Vector::new(0.0, 0.0, 1.0));
    assert!(ray_triangle_intersection(&ray, &face).is_none());

    // test back-face culling
    let ray = Ray::new(Vertex::new(0.0, 0.0, 6.0), Vector::new(0.0, 0.0, -1.0));
    assert!(ray_triangle_intersection(&ray, &face).is_none());
}
