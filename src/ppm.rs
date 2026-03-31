use crate::canvas::RenderCanvas;
use sdl2::pixels::Color;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::PathBuf;

pub struct PpmCanvas {
    file_path: PathBuf,
    width: u32,
    height: u32,
    pixels: Box<[Color]>,
}

impl PpmCanvas {
    pub fn new(file_path: PathBuf, width: u32, height: u32) -> PpmCanvas {
        PpmCanvas {
            file_path,
            width,
            height,
            pixels: Box::new([]),
        }
    }

    fn write_ppm(&self) -> Result<()> {
        let mut w = BufWriter::new(File::create(&self.file_path)?);

        //
        // header
        //
        write!(w, "P3\n{} {}\n255\n", self.width, self.height)?;

        //
        // der pixelen
        //
        for pixel in &self.pixels {
            writeln!(w, "{} {} {}", pixel.r, pixel.g, pixel.b)?;
        }

        Ok(())
    }
}

impl RenderCanvas for PpmCanvas {
    fn set_pixel(&mut self, x: i32, y: i32, color: sdl2::pixels::Color) {
        let idx = y * (self.width as i32) + x;
        self.pixels[idx as usize] = color;
    }

    fn clear(&mut self, clear_color: Color) {
        let pixels: Vec<Color> = vec![clear_color; (self.width * self.width) as usize];
        self.pixels = pixels.into_boxed_slice();
    }

    fn present(&mut self) {
        self.write_ppm().unwrap();
    }
}
