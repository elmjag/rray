use rray::camera::Camera;
use rray::loader::load_scene;
use rray::mesh::Mesh;
use rray::pump::Pump;
use rray::render::draw_frame;
use rray::window::WindowCanvas;
use rray::{args, ppm::PpmCanvas};
use std::path::PathBuf;
use std::process::ExitCode;
use std::{thread::sleep, time::Duration};

const SCALE: u32 = 16;

fn render_to_ppm(file: PathBuf, mut camera: Camera, mut mesh: Mesh) {
    let (width, height) = camera.canvas_size();
    let mut canvas = PpmCanvas::new(file, width, height);
    draw_frame(&mut canvas, &mut camera, &mut mesh);
}

fn render_to_screen(mut camera: Camera, mut mesh: Mesh) {
    let sdl_context = sdl2::init().unwrap();
    let (width, height) = camera.canvas_size();
    let mut canvas = WindowCanvas::init(&sdl_context, "rusty rays", SCALE, width, height);
    let mut pump = Pump::init(&sdl_context);

    draw_frame(&mut canvas, &mut camera, &mut mesh);

    while !pump.terminated() {
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main() -> ExitCode {
    let args = args::parse();

    let (camera, mesh) = match load_scene(args.scene) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("Failed to load scene file: {err}");
            return ExitCode::FAILURE;
        }
    };

    if let Some(file) = args.out {
        render_to_ppm(file, camera, mesh);
    } else {
        render_to_screen(camera, mesh);
    }

    ExitCode::SUCCESS
}
