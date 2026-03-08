use crate::{mesh::Mesh, space::Vertex};

pub fn get_mesh() -> Mesh {
    let vertices = vec![
        Vertex::new(-2.0, -2.0, 8.0),
        Vertex::new(0.0, 2.0, 8.0),
        Vertex::new(2.0, -2.0, 8.0),
    ];
    let triangles = vec![(0, 1, 2)];

    Mesh::new(vertices, triangles)
}
