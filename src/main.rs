use rray::camera::Camera;
use rray::canvas::WindowCanvas;
use rray::loader::get_mesh;
use rray::pump::Pump;
use rray::render::draw_frame;
use std::{thread::sleep, time::Duration};

const SCALE: u32 = 8;
const DEPTH: u32 = 32;
const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut canvas = WindowCanvas::init(&sdl_context, "rusty rays", SCALE, WIDTH, HEIGHT);
    let mut camera = Camera::new(WIDTH, HEIGHT, DEPTH);
    let mut pump = Pump::init(&sdl_context);

    let mut mesh = get_mesh();

    while !pump.terminated() {
        draw_frame(&mut canvas, &mut camera, &mut mesh);
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
