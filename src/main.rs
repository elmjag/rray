use rray::{
    args, camera::Camera, loader::load_scene, mesh::Mesh, model::Model, ppm::PpmCanvas,
    render::draw_frame, rotation::Rotation, space::Z_AXIS, timer::Timer, transform::Transform,
    translation::Translation, window::WindowCanvas,
};
use std::path::PathBuf;
use std::process::ExitCode;

const FPS: f32 = 25.0;

fn render_to_ppm(file: PathBuf, mut camera: Camera, mut mesh: Mesh) {
    let (width, height) = camera.canvas_size();
    let mut canvas = PpmCanvas::new(file, width, height);
    let transform = Transform::new(Translation::new(0.0, 0.0, 0.0), Rotation::new(0.0, &Z_AXIS));

    draw_frame(&mut canvas, &mut camera, &mut mesh, transform);
}

fn render_to_screen(mut camera: Camera, mut mesh: Mesh, scale: u32, show_fps_stats: bool) {
    let sdl_context = sdl2::init().unwrap();
    let (width, height) = camera.canvas_size();
    let mut canvas = WindowCanvas::init(&sdl_context, "rusty rays", scale, width, height);
    let mut model = Model::new(&sdl_context);
    let mut timer = Timer::new(&sdl_context, FPS, show_fps_stats);

    loop {
        timer.start_frame();
        let (transform, terminated) = model.get_state(&timer);

        draw_frame(&mut canvas, &mut camera, &mut mesh, transform);
        timer.rendering_finished();

        model.process_events(&timer);

        if terminated {
            break;
        }
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
        render_to_screen(camera, mesh, args.scale, args.fps_stats);
    }

    ExitCode::SUCCESS
}
