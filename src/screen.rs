use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Screen {
    canvas: Canvas<Window>,
}

impl Screen {
    pub fn init(sdl_context: &Sdl, window_title: &str, width: u32, height: u32) -> Screen {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(window_title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Screen { canvas }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas
            .draw_point(sdl2::rect::Point::new(x, y))
            .unwrap();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
