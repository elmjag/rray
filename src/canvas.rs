use sdl2::pixels::Color;

pub trait RenderCanvas {
    fn set_pixel(&mut self, x: i32, y: i32, color: Color);
    fn clear(&mut self, clear_color: Color);
    fn present(&mut self);
}
