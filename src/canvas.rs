use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const CLEAR_COLOR: Color = Color::RGB(255, 255, 255);

pub struct WindowCanvas {
    canvas: Canvas<Window>,
    scale: u32,
}

pub trait RenderCanvas {
    fn size(&self) -> (u32, u32);
    fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8);
    fn clear(&mut self);
    fn present(&mut self);
}

impl WindowCanvas {
    pub fn init(
        sdl_context: &Sdl,
        window_title: &str,
        scale: u32,
        width: u32,
        height: u32,
    ) -> WindowCanvas {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(window_title, width * scale, height * scale)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        WindowCanvas { canvas, scale }
    }
}

impl RenderCanvas for WindowCanvas {
    fn size(&self) -> (u32, u32) {
        let (w, h) = self.canvas.window().size();

        (w / self.scale, h / self.scale)
    }

    fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        let iscale = self.scale as i32;
        let rect = Rect::new(x * iscale, y * iscale, self.scale, self.scale);

        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.fill_rect(rect).unwrap();

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.draw_rect(rect).unwrap();
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(CLEAR_COLOR);
        self.canvas.clear();
    }

    fn present(&mut self) {
        self.canvas.present();
    }
}
