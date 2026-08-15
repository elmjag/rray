use sdl2::pixels::Color as SdlColor;
use serde::Deserialize;
use std::convert::Into;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Color {
    // color channel range is 0.0 to 1.0
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
    pub const GRAY: Color = Color::new(0.5, 0.5, 0.5);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);
    pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0);
    pub const CYAN: Color = Color::new(0.0, 1.0, 1.0);

    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn scale(&mut self, scale: f32) -> Self {
        assert!(0.0 <= scale && scale <= 1.0);
        Self::new(self.red * scale, self.green * scale, self.blue * scale)
    }
}

impl Into<SdlColor> for Color {
    fn into(self) -> SdlColor {
        fn to_u8(v: f32) -> u8 {
            (v * 255.0) as u8
        }
        SdlColor::RGB(to_u8(self.red), to_u8(self.green), to_u8(self.blue))
    }
}
