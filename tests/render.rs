use sdl2::pixels::Color;
use std::collections::HashSet;

use rray::{camera::Camera, canvas::RenderCanvas, mesh::Mesh, render, space::Vertex};

const DEPTH: u32 = 32;
const WIDTH: u32 = 24;
const HEIGHT: u32 = 24;

const GREEN_DEPTH: f32 = 8.0;
const RED_DEPTH: f32 = 8.0;

struct TestCanvas {
    pixels: HashSet<(i32, i32, Color)>,
}

impl TestCanvas {
    fn new() -> TestCanvas {
        TestCanvas {
            pixels: HashSet::new(),
        }
    }
}

impl RenderCanvas for TestCanvas {
    fn size(&self) -> (u32, u32) {
        (WIDTH, HEIGHT)
    }

    fn set_pixel(&mut self, x: i32, y: i32, c: Color) {
        self.pixels.insert((x, y, c));
    }

    fn clear(&mut self, _clear_color: Color) {
        self.pixels.clear();
    }

    fn present(&mut self) {
        /* nop */
    }
}

fn get_mesh() -> Mesh {
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

///
/// test render::draw_frame
///
#[test]
fn draw_frame() {
    //
    // render a pre-defined mesh,
    // and check that expected pixels where drawn
    //
    let mut canvas = TestCanvas::new();
    let camera = Camera::new(WIDTH, HEIGHT, DEPTH);
    let mut mesh = get_mesh();

    let expected_pixels = HashSet::from([
        (13, 13, Color::GREEN),
        (13, 16, Color::GREEN),
        (9, 11, Color::RED),
        (9, 13, Color::RED),
        (10, 16, Color::RED),
        (14, 6, Color::GREEN),
        (14, 8, Color::GREEN),
        (14, 5, Color::GREEN),
        (16, 11, Color::RED),
        (15, 12, Color::GREEN),
        (15, 4, Color::GREEN),
        (8, 5, Color::RED),
        (9, 8, Color::RED),
        (14, 13, Color::GREEN),
        (9, 12, Color::RED),
        (10, 14, Color::RED),
        (10, 6, Color::RED),
        (11, 15, Color::RED),
        (8, 16, Color::RED),
        (11, 12, Color::RED),
        (8, 8, Color::RED),
        (8, 7, Color::RED),
        (14, 10, Color::GREEN),
        (12, 7, Color::GREEN),
        (15, 13, Color::GREEN),
        (8, 14, Color::RED),
        (8, 15, Color::RED),
        (9, 15, Color::RED),
        (10, 13, Color::RED),
        (13, 10, Color::GREEN),
        (12, 15, Color::GREEN),
        (11, 8, Color::RED),
        (15, 10, Color::GREEN),
        (13, 15, Color::GREEN),
        (14, 16, Color::GREEN),
        (15, 5, Color::GREEN),
        (8, 6, Color::RED),
        (10, 8, Color::RED),
        (9, 10, Color::RED),
        (11, 11, Color::RED),
        (10, 15, Color::RED),
        (15, 15, Color::GREEN),
        (13, 8, Color::GREEN),
        (9, 16, Color::RED),
        (11, 14, Color::RED),
        (8, 10, Color::RED),
        (13, 14, Color::GREEN),
        (10, 10, Color::RED),
        (8, 12, Color::RED),
        (17, 12, Color::RED),
        (12, 8, Color::GREEN),
        (12, 10, Color::GREEN),
        (15, 11, Color::GREEN),
        (12, 11, Color::GREEN),
        (17, 11, Color::RED),
        (14, 14, Color::GREEN),
        (14, 11, Color::GREEN),
        (9, 5, Color::RED),
        (6, 12, Color::GREEN),
        (12, 14, Color::GREEN),
        (8, 9, Color::RED),
        (14, 7, Color::GREEN),
        (8, 11, Color::RED),
        (14, 15, Color::GREEN),
        (11, 16, Color::RED),
        (8, 13, Color::RED),
        (7, 11, Color::GREEN),
        (9, 9, Color::RED),
        (14, 9, Color::GREEN),
        (8, 17, Color::RED),
        (11, 9, Color::RED),
        (13, 11, Color::GREEN),
        (10, 17, Color::RED),
        (13, 17, Color::GREEN),
        (15, 7, Color::GREEN),
        (14, 17, Color::GREEN),
        (15, 6, Color::GREEN),
        (11, 7, Color::RED),
        (13, 7, Color::GREEN),
        (15, 17, Color::GREEN),
        (9, 18, Color::RED),
        (8, 4, Color::RED),
        (10, 9, Color::RED),
        (15, 9, Color::GREEN),
        (8, 18, Color::RED),
        (11, 10, Color::RED),
        (15, 18, Color::GREEN),
        (8, 19, Color::RED),
        (15, 14, Color::GREEN),
        (11, 13, Color::RED),
        (10, 12, Color::RED),
        (10, 7, Color::RED),
        (13, 9, Color::GREEN),
        (12, 16, Color::GREEN),
        (9, 17, Color::RED),
        (6, 11, Color::GREEN),
        (12, 9, Color::GREEN),
        (10, 11, Color::RED),
        (7, 12, Color::GREEN),
        (13, 12, Color::GREEN),
        (14, 18, Color::GREEN),
        (15, 19, Color::GREEN),
        (15, 8, Color::GREEN),
        (9, 14, Color::RED),
        (15, 16, Color::GREEN),
        (13, 6, Color::GREEN),
        (12, 12, Color::GREEN),
        (12, 13, Color::GREEN),
        (16, 12, Color::RED),
        (9, 7, Color::RED),
        (9, 6, Color::RED),
        (14, 12, Color::GREEN),
    ]);

    render::draw_frame(&mut canvas, &camera, &mut mesh);

    assert_eq!(canvas.pixels, expected_pixels);
}
