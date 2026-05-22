use crate::{
    camera::Camera,
    canvas::RenderCanvas,
    intersect::ray_triangle_intersection,
    mesh::{Face, Mesh},
    ray::Ray,
    rotation::Rotation,
    space::Vector,
    transform::Transform,
};
use core::f32;
use sdl2::pixels::Color;
use std::time::Duration;

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

fn get_transform(elapsed_time: Duration) -> Transform {
    let z_axis = Vector::new(0.0, 0.0, 1.0);
    let rotation_angle = (elapsed_time.as_millis() % 6283) as f32 / 1000.0;
    Transform::new(Rotation::new(rotation_angle, &z_axis))
}

///
/// Draw a mesh to provided canvas.
///
/// `frame_delta` is the time elapsed since last frame was drawn.
///
///
pub fn draw_frame(
    canvas: &mut impl RenderCanvas,
    camera: &Camera,
    mesh: &mut Mesh,
    elapsed_time: Duration,
) {
    let transform = get_transform(elapsed_time);
    let faces = mesh.get_faces(transform);
    let (w, h) = camera.canvas_size();

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
