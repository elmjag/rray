use crate::{
    camera::Camera,
    canvas::RenderCanvas,
    intersect::ray_triangle_intersection,
    mesh::{Face, Mesh},
    ray::Ray,
    rotation::Rotation,
    space::Z_AXIS,
    transform::Transform,
    translation::Translation,
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
    let elapsed_ms = elapsed_time.as_millis();

    let rotation_angle = (elapsed_ms % 6283) as f32 / 1000.0;
    let x_translation = ((elapsed_ms % 2000) as i64) as f32 / 1000.0;

    Transform::new(
        Translation::new(x_translation, 0.0, 0.0),
        Rotation::new(rotation_angle, &Z_AXIS),
    )
}

///
/// Draw a mesh to provided canvas.
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
