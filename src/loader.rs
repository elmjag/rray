use crate::{mesh::Mesh, space::Vertex};
use sdl2::pixels::Color;

const GREEN_DEPTH: f32 = 8.0;
const RED_DEPTH: f32 = 8.0;

pub fn get_mesh() -> Mesh {
    let vertices = vec![
        Vertex::new(-2.0, 0.0, GREEN_DEPTH + 2.0),
        Vertex::new(1.0, 2.0, GREEN_DEPTH),
        Vertex::new(1.0, -2.0, GREEN_DEPTH),
        Vertex::new(-1.0, 2.0, RED_DEPTH),
        Vertex::new(2.0, 0.0, RED_DEPTH + 2.0),
        Vertex::new(-1.0, -2.0, RED_DEPTH),
    ];
    let triangles = vec![(0, 1, 2, Color::GREEN), (3, 4, 5, Color::RED)];

    Mesh::new(vertices, triangles)
}
