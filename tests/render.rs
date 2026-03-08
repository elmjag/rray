use std::collections::HashSet;

use rray::{camera::Camera, canvas::RenderCanvas, mesh::Mesh, render, space::Vertex};

const DEPTH: u32 = 12;
const WIDTH: u32 = 12;
const HEIGHT: u32 = 12;

struct TestCanvas {
    pixels: HashSet<(i32, i32)>,
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

    fn set_pixel(&mut self, x: i32, y: i32, _r: u8, _g: u8, _b: u8) {
        self.pixels.insert((x, y));
    }

    fn clear(&mut self) {
        self.pixels.clear();
    }

    fn present(&mut self) {
        /* nop */
    }
}

pub fn get_mesh() -> Mesh {
    let vertices = vec![
        Vertex::new(-2.0, -2.0, 8.0),
        Vertex::new(0.0, 2.0, 8.0),
        Vertex::new(2.0, -2.0, 8.0),
    ];
    let triangles = vec![(0, 1, 2)];

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
        (5, 4),
        (6, 4),
        (5, 5),
        (6, 5),
        (4, 6),
        (5, 6),
        (6, 6),
        (7, 6),
        (4, 7),
        (5, 7),
        (6, 7),
        (7, 7),
        (3, 8),
        (4, 8),
        (5, 8),
        (6, 8),
        (7, 8),
        (8, 8),
    ]);

    render::draw_frame(&mut canvas, &camera, &mut mesh);

    assert_eq!(canvas.pixels, expected_pixels);
}
