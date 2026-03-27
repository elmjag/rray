use crate::{mesh::Mesh, space::Vertex};
use sdl2::pixels::Color;
use std::f32::consts::TAU;

const SEGMENTS: usize = 8;
const RADIUS: f32 = 2.0;
const HEIGHT: f32 = 8.0;

fn vertices(segments: usize, radius: f32, height: f32) -> Vec<Vertex> {
    let angle_step = TAU / (segments as f32);

    let r = (0..segments).map(|n| {
        let angle = angle_step * (n as f32);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        Vertex::new(x, y, height)
    });

    let mut verts = vec![Vertex::new(0.0, 0.0, height)];
    verts.append(&mut r.collect());

    verts
}

fn triangles(vertices: &Vec<Vertex>) -> Vec<(usize, usize, usize, Color)> {
    let num_verts = vertices.len();
    let triangles = (0..num_verts - 1).map(|n| {
        let last = if n == 0 { num_verts - 1 } else { n };
        let color = match n % 4 {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            3 => Color::YELLOW,
            _ => panic!("wat"),
        };
        (0, n + 1, last, color)
    });

    triangles.collect()
}

pub fn get_mesh() -> Mesh {
    let vertices = vertices(SEGMENTS, RADIUS, HEIGHT);
    let triangles = triangles(&vertices);

    Mesh::new(vertices, triangles)
}
