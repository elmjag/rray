use rray::{
    args, model::Model, ppm::PpmCanvas, render::draw_frame, timer::Timer, window::WindowCanvas,
};
use sdl2::Sdl;
use std::path::PathBuf;
use std::process::ExitCode;

const FPS: f32 = 25.0;

fn render_to_ppm(file: PathBuf, mut model: Model) {
    let (width, height) = model.canvas_size();
    let mut canvas = PpmCanvas::new(file, width, height);
    let (scene, _) = model.get_scene(0);

    draw_frame(&mut canvas, scene);
}

fn render_to_screen(sdl_context: Sdl, mut model: Model, scale: u32, show_fps_stats: bool) {
    let (width, height) = model.canvas_size();
    let mut canvas = WindowCanvas::init(&sdl_context, "rusty rays", scale, width, height);
    let mut timer = Timer::new(&sdl_context, FPS, show_fps_stats);

    loop {
        timer.start_frame();
        let (scene, terminated) = model.get_scene(timer.current_time());

        draw_frame(&mut canvas, scene);
        timer.rendering_finished();

        model.process_events(&timer);

        if terminated {
            break;
        }
    }
}

pub fn main() -> ExitCode {
    let args = args::parse();

    let sdl_context = sdl2::init().unwrap();

    let model = match Model::new(&sdl_context, args.scene) {
        Ok(model) => model,
        Err(err) => {
            eprintln!("Failed to load scene file: {err}");
            return ExitCode::FAILURE;
        }
    };

    if let Some(file) = args.out {
        render_to_ppm(file, model);
    } else {
        render_to_screen(sdl_context, model, args.scale, args.fps_stats);
    }

    ExitCode::SUCCESS
}
