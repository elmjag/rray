use crate::scene::{Camera, Color, Mesh};
use std::f32::consts::TAU;

const PYRAMID_SEGMENTS: usize = 8;
const PYRAMID_RADIUS: f32 = 2.0;
const PYRAMID_HEIGHT: f32 = 4.0;

const CANVAS_WIDTH: u32 = 256;
const CANVAS_HEIGHT: u32 = 192;
const CAMERA_DEPTH: f32 = 128.0;

fn vertices(segments: usize, radius: f32, height: f32) -> Vec<(f32, f32, f32)> {
    let angle_step = TAU / (segments as f32);
    let bottom_z = height / -2.0;
    let top_z = height / 2.0;

    let r = (0..segments).map(|n| {
        let angle = angle_step * (n as f32);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        (x, y, bottom_z)
    });

    let mut verts = vec![(0.0, 0.0, bottom_z), (0.0, 0.0, top_z)];
    verts.append(&mut r.collect());

    verts
}

fn triangles(num_verts: usize) -> Vec<(usize, usize, usize, usize)> {
    let top_triangles = (2..num_verts).map(|n| {
        let color = n % 4;
        let prev = if n == 2 { num_verts - 1 } else { n - 1 };
        let verts = (1, n, prev, color);

        verts
    });

    let bottom_triangles = (2..num_verts).map(|n| {
        let color = 4 + n % 2;
        let prev = if n == 2 { num_verts - 1 } else { n - 1 };
        let verts = (0, prev, n, color);

        verts
    });

    top_triangles.chain(bottom_triangles).collect()
}

pub fn get_mesh() -> Mesh {
    let colors = vec![
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::BLACK,
        Color::GRAY,
    ];
    let vertices = vertices(PYRAMID_SEGMENTS, PYRAMID_RADIUS, PYRAMID_HEIGHT);
    let triangles = triangles(vertices.len());

    Mesh::new(colors, vertices, triangles)
}

pub fn get_camera() -> Camera {
    Camera::new(CANVAS_WIDTH, CANVAS_HEIGHT, CAMERA_DEPTH)
}
