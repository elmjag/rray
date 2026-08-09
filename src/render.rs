use crate::{
    canvas::RenderCanvas, intersect::ray_triangle_intersection, mesh::Face, ray::Ray, scene::Scene,
    space::Vector,
};
use core::f32;
use sdl2::pixels::Color;

const CLEAR_COLOR: Color = Color::WHITE;

///
/// Find nearest face the ray hits, if any.
///
fn get_ray_face<'a>(ray: &Ray, faces: &'a Vec<Face>) -> Option<&'a Face> {
    let mut last_t = f32::MAX;
    let mut nearest_face = None;

    for face in faces {
        let r = ray_triangle_intersection(ray, face, last_t);
        if let Some(t) = r {
            last_t = t;
            nearest_face = Some(face);
        }
    }

    nearest_face
}

fn scale_color(scale: f32, original: Color) -> Color {
    fn calc(scale: f32, val: u8) -> u8 {
        ((val as f32) * scale) as u8
    }

    let r = calc(scale, original.r);
    let g = calc(scale, original.g);
    let b = calc(scale, original.b);

    Color::RGB(r, g, b)
}

fn get_ray_color(ray: &Ray, directional_light: &Vector, faces: &Vec<Face>) -> Option<Color> {
    match get_ray_face(ray, faces) {
        Some(face) => {
            let norm = face.normal();
            // calculate direction light angle incidence on the face
            let incidence = norm.dot(directional_light) * -1.0;
            Some(scale_color(incidence, face.color()))
        }
        None => None,
    }
}

///
/// Draw a mesh to provided canvas.
///
pub fn draw_frame(canvas: &mut impl RenderCanvas, scene: Scene) {
    let camera = scene.camera();
    let directional_light = scene.directional_light();
    let faces = scene.object().get_faces();
    let (w, h) = camera.canvas_size();

    canvas.clear(CLEAR_COLOR);

    for y in 0..h {
        for x in 0..w {
            let ray = camera.get_pixel_ray(x, y);
            if let Some(color) = get_ray_color(&ray, directional_light, &faces) {
                canvas.set_pixel(x as i32, y as i32, color);
            }
        }
    }

    canvas.present();
}
