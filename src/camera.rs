use crate::{point::Point, ray::Ray, vector::Vector};
use std::ops::Range;

pub struct Camera {
    depth: f32,
    screen_width: u32,
    screen_height: u32,
    rays: Option<Vec<Ray>>,
}

struct ScreenCoords {
    x_range: Range<i32>,
    last_y: i32,
    cur_x: i32,
    cur_y: i32,
}

fn screen_coords(camera: &Camera) -> ScreenCoords {
    let w = camera.screen_width as i32;
    let h = camera.screen_height as i32;

    assert!(w % 2 == 0, "odd width size not supported");
    assert!(h % 2 == 0, "odd height size not supported");

    let half_w = w / 2;
    let halw_h = h / 2;

    ScreenCoords {
        x_range: -half_w..half_w,
        last_y: -halw_h,
        cur_x: -half_w,
        cur_y: halw_h,
    }
}

impl Iterator for ScreenCoords {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_y <= self.last_y {
            return None;
        }

        let res = ((self.cur_x as f32) + 0.5, (self.cur_y as f32) - 0.5);

        self.cur_x += 1;
        if self.cur_x >= self.x_range.end {
            self.cur_y -= 1;
            self.cur_x = self.x_range.start;
        }

        Some(res)
    }
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32, depth: u32) -> Camera {
        Camera {
            screen_width,
            screen_height,
            depth: depth as f32,
            rays: None,
        }
    }

    fn num_rays(&self) -> usize {
        (self.screen_width * self.screen_height) as usize
    }

    fn make_rays(&mut self) {
        let focus_point = Point::new(0.0, 0.0, 0.0);

        let mut rays = Vec::with_capacity(self.num_rays());

        for (x, y) in screen_coords(self) {
            let orig = Point::new(x, y, -self.depth);
            let direction = orig.vect_to(&focus_point);

            rays.push(Ray::new(orig, direction));
        }

        self.rays = Some(rays);
    }

    pub fn get_rays(&mut self) -> &Vec<Ray> {
        if self.rays.is_none() {
            self.make_rays();
        }

        self.rays.as_ref().unwrap()
    }
}
