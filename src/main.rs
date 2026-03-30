use rray::camera::Camera;
use rray::loader::get_mesh;
use rray::mesh::Mesh;
use rray::pump::Pump;
use rray::render::draw_frame;
use rray::window::WindowCanvas;
use rray::{args, ppm::PpmCanvas};
use std::path::PathBuf;
use std::{thread::sleep, time::Duration};

const SCALE: u32 = 16;
const DEPTH: u32 = 64;
const WIDTH: u32 = 16*4;
const HEIGHT: u32 = 16*4;

fn render_to_ppm(file: PathBuf, mut camera: Camera, mut mesh: Mesh) {
    let mut canvas = PpmCanvas::new(file, WIDTH, HEIGHT);
    draw_frame(&mut canvas, &mut camera, &mut mesh);
}

fn render_to_screen(mut camera: Camera, mut mesh: Mesh) {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = WindowCanvas::init(&sdl_context, "rusty rays", SCALE, WIDTH, HEIGHT);
    let mut pump = Pump::init(&sdl_context);

    draw_frame(&mut canvas, &mut camera, &mut mesh);

    while !pump.terminated() {
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main() {
    let args = args::parse();
    let camera = Camera::new(WIDTH, HEIGHT, DEPTH);
    let mesh = get_mesh();

    if let Some(file) = args.out {
        render_to_ppm(file, camera, mesh);
        return;
    }

    render_to_screen(camera, mesh);
}
