use crate::{camera::Camera, mesh::Mesh};
use sdl2::pixels::Color;
use std::f32::consts::TAU;

const PYRAMID_SEGMENTS: usize = 8;
const PYRAMID_RADIUS: f32 = 2.0;
const PYRAMID_HEIGHT: f32 = 8.0;

const CANVAS_WIDTH: u32 = 64;
const CANVAS_HEIGHT: u32 = 64;
const CAMERA_DEPTH: f32 = 64.0;

fn vertices(segments: usize, radius: f32, height: f32) -> Vec<(f32, f32, f32)> {
    let angle_step = TAU / (segments as f32);

    let r = (0..segments).map(|n| {
        let angle = angle_step * (n as f32);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        (x, y, height)
    });

    let mut verts = vec![(0.0, 0.0, height)];
    verts.append(&mut r.collect());

    verts
}

fn triangles(num_verts: usize) -> Vec<(usize, usize, usize, usize)> {
    let triangles = (0..num_verts - 1).map(|n| {
        let last = if n == 0 { num_verts - 1 } else { n };
        let color = n % 4;
        (0, n + 1, last, color)
    });

    triangles.collect()
}

pub fn get_mesh() -> Mesh {
    let colors = vec![
        Color::RED.rgb(),
        Color::GREEN.rgb(),
        Color::BLUE.rgb(),
        Color::YELLOW.rgb(),
    ];
    let vertices = vertices(PYRAMID_SEGMENTS, PYRAMID_RADIUS, PYRAMID_HEIGHT);
    let triangles = triangles(vertices.len());

    Mesh::new(colors, vertices, triangles)
}

pub fn get_camera() -> Camera {
    Camera::new(CANVAS_WIDTH, CANVAS_HEIGHT, CAMERA_DEPTH)
}
