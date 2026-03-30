use core::f32;

use crate::{
    camera::Camera,
    canvas::RenderCanvas,
    intersect::ray_triangle_intersection,
    mesh::{Face, Mesh},
    ray::Ray,
};
use sdl2::pixels::Color;

const CLEAR_COLOR: Color = Color::WHITE;

fn find_first_ray_hit(ray: &Ray, faces: &Vec<Face>) -> Option<Color> {
    let mut last_t = f32::MAX;
    let mut nearest_face = None;

    for face in faces {
        let r = ray_triangle_intersection(ray, face);
        if let Some(t) = r {
            if t < last_t {
                last_t = t;
                nearest_face = Some(face);
            }
        }
    }

    match nearest_face {
        Some(face) => Some(face.color()),
        None => None,
    }
}

pub fn draw_frame(canvas: &mut impl RenderCanvas, camera: &Camera, mesh: &mut Mesh) {
    let faces = mesh.get_faces();

    let (w, h) = canvas.size();

    canvas.clear(CLEAR_COLOR);

    for y in 0..h {
        for x in 0..w {
            let ray = camera.get_pixel_ray(x, y);
            if let Some(color) = find_first_ray_hit(&ray, &faces) {
                canvas.set_pixel(x as i32, y as i32, color);
            }
        }
    }

    canvas.present();
}
