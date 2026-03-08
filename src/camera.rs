use crate::{ray::Ray, space::Vertex};

pub struct Camera {
    depth: f32,
    x_offset: f32,
    y_offset: f32,
    focus_point: Vertex,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32, depth: u32) -> Camera {
        assert!(screen_width % 2 == 0, "odd width size not supported");
        assert!(screen_height % 2 == 0, "odd height size not supported");

        let x_offset = (screen_width as f32) / 2.0 - 0.5;
        let y_offset = (screen_height as f32) / 2.0 - 0.5;

        Camera {
            x_offset,
            y_offset,
            depth: depth as f32,
            focus_point: Vertex::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_pixel_ray(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        //
        // note that we flip-around X and Y axis here,
        // to avoid rendering 'upside-down'
        //
        let x = self.x_offset - (pixel_x as f32);
        let y = (pixel_y as f32) - self.y_offset;

        let orig = Vertex::new(x, y, -self.depth);
        let direction = &self.focus_point - &orig;

        Ray::new(orig, direction)
    }
}
