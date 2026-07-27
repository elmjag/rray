use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::canvas::RenderCanvas;

pub struct WindowCanvas {
    canvas: Canvas<Window>,
    scale: u32,
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

    fn set_unscaled_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);
        let _ = self.canvas.draw_point(Point::new(x, y));
    }

    fn set_scaled_pixel(&mut self, x: i32, y: i32, color: Color) {
        let iscale = self.scale as i32;
        let rect = Rect::new(x * iscale, y * iscale, self.scale, self.scale);

        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();

        if self.scale > 6 {
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.draw_rect(rect).unwrap();
        }
    }
}

impl RenderCanvas for WindowCanvas {
    fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if self.scale == 1 {
            self.set_unscaled_pixel(x, y, color);
        } else {
            self.set_scaled_pixel(x, y, color);
        }
    }

    fn clear(&mut self, clear_color: Color) {
        self.canvas.set_draw_color(clear_color);
        self.canvas.clear();
    }

    fn present(&mut self) {
        self.canvas.present();
    }
}
