use rray::{
    canvas::RenderCanvas,
    render,
    scene::{Camera, Color, Mesh, Object, Rotation, Scene, Transform, Translation},
    vector::{Vector, Z_AXIS},
};
use sdl2::pixels::Color as SdlColor;
use std::collections::HashSet;

const WIDTH: u32 = 24;
const HEIGHT: u32 = 24;
const DEPTH: f32 = 32.0;

const GREEN_DEPTH: f32 = 8.0;
const RED_DEPTH: f32 = 8.0;

struct TestCanvas {
    pixels: HashSet<(i32, i32, SdlColor)>,
}

impl TestCanvas {
    fn new() -> TestCanvas {
        TestCanvas {
            pixels: HashSet::new(),
        }
    }
}

impl RenderCanvas for TestCanvas {
    fn set_pixel(&mut self, x: i32, y: i32, c: SdlColor) {
        self.pixels.insert((x, y, c));
    }

    fn clear(&mut self, _clear_color: SdlColor) {
        self.pixels.clear();
    }

    fn present(&mut self) {
        /* nop */
    }
}

fn get_mesh() -> Mesh {
    let colors = vec![Color::GREEN, Color::RED];
    let vertices = vec![
        (-2.0, 0.0, GREEN_DEPTH + 2.0),
        (1.0, 2.0, GREEN_DEPTH),
        (1.0, -2.0, GREEN_DEPTH),
        (-1.0, 2.0, RED_DEPTH),
        (2.0, 0.0, RED_DEPTH + 2.0),
        (-1.0, -2.0, RED_DEPTH),
    ];
    let triangles = vec![(0, 1, 2, 0), (3, 4, 5, 1)];

    Mesh::new(colors, vertices, triangles)
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
    let dir_light = Vector::new(0.0, 0.0, 1.0);
    let mesh = get_mesh();
    let transform = Transform::new(Translation::new(0.0, 0.0, 0.0), Rotation::new(0.0, &Z_AXIS));

    render::draw_frame(
        &mut canvas,
        Scene::new(&camera, &dir_light, Object::new(&mesh, transform)),
    );

    // shaded result colors, taking into account incidence with direct light
    let shaded_green = SdlColor::RGB(0, 212, 0);
    let shaded_red = SdlColor::RGB(212, 0, 0);

    let expected_pixels = HashSet::from([
        (13, 13, shaded_green),
        (13, 16, shaded_green),
        (9, 11, shaded_red),
        (9, 13, shaded_red),
        (10, 16, shaded_red),
        (14, 6, shaded_green),
        (14, 8, shaded_green),
        (14, 5, shaded_green),
        (16, 11, shaded_red),
        (15, 12, shaded_green),
        (15, 4, shaded_green),
        (8, 5, shaded_red),
        (9, 8, shaded_red),
        (14, 13, shaded_green),
        (9, 12, shaded_red),
        (10, 14, shaded_red),
        (10, 6, shaded_red),
        (11, 15, shaded_red),
        (8, 16, shaded_red),
        (11, 12, shaded_red),
        (8, 8, shaded_red),
        (8, 7, shaded_red),
        (14, 10, shaded_green),
        (12, 7, shaded_green),
        (15, 13, shaded_green),
        (8, 14, shaded_red),
        (8, 15, shaded_red),
        (9, 15, shaded_red),
        (10, 13, shaded_red),
        (13, 10, shaded_green),
        (12, 15, shaded_green),
        (11, 8, shaded_red),
        (15, 10, shaded_green),
        (13, 15, shaded_green),
        (14, 16, shaded_green),
        (15, 5, shaded_green),
        (8, 6, shaded_red),
        (10, 8, shaded_red),
        (9, 10, shaded_red),
        (11, 11, shaded_red),
        (10, 15, shaded_red),
        (15, 15, shaded_green),
        (13, 8, shaded_green),
        (9, 16, shaded_red),
        (11, 14, shaded_red),
        (8, 10, shaded_red),
        (13, 14, shaded_green),
        (10, 10, shaded_red),
        (8, 12, shaded_red),
        (17, 12, shaded_red),
        (12, 8, shaded_green),
        (12, 10, shaded_green),
        (15, 11, shaded_green),
        (12, 11, shaded_green),
        (17, 11, shaded_red),
        (14, 14, shaded_green),
        (14, 11, shaded_green),
        (9, 5, shaded_red),
        (6, 12, shaded_green),
        (12, 14, shaded_green),
        (8, 9, shaded_red),
        (14, 7, shaded_green),
        (8, 11, shaded_red),
        (14, 15, shaded_green),
        (11, 16, shaded_red),
        (8, 13, shaded_red),
        (7, 11, shaded_green),
        (9, 9, shaded_red),
        (14, 9, shaded_green),
        (8, 17, shaded_red),
        (11, 9, shaded_red),
        (13, 11, shaded_green),
        (10, 17, shaded_red),
        (13, 17, shaded_green),
        (15, 7, shaded_green),
        (14, 17, shaded_green),
        (15, 6, shaded_green),
        (11, 7, shaded_red),
        (13, 7, shaded_green),
        (15, 17, shaded_green),
        (9, 18, shaded_red),
        (8, 4, shaded_red),
        (10, 9, shaded_red),
        (15, 9, shaded_green),
        (8, 18, shaded_red),
        (11, 10, shaded_red),
        (15, 18, shaded_green),
        (8, 19, shaded_red),
        (15, 14, shaded_green),
        (11, 13, shaded_red),
        (10, 12, shaded_red),
        (10, 7, shaded_red),
        (13, 9, shaded_green),
        (12, 16, shaded_green),
        (9, 17, shaded_red),
        (6, 11, shaded_green),
        (12, 9, shaded_green),
        (10, 11, shaded_red),
        (7, 12, shaded_green),
        (13, 12, shaded_green),
        (14, 18, shaded_green),
        (15, 19, shaded_green),
        (15, 8, shaded_green),
        (9, 14, shaded_red),
        (15, 16, shaded_green),
        (13, 6, shaded_green),
        (12, 12, shaded_green),
        (12, 13, shaded_green),
        (16, 12, shaded_red),
        (9, 7, shaded_red),
        (9, 6, shaded_red),
        (14, 12, shaded_green),
    ]);

    assert_eq!(canvas.pixels, expected_pixels);
}
