use crate::{
    canvas::RenderCanvas,
    intersect::ray_triangle_intersection,
    ray::Ray,
    scene::{Color, Face, Scene},
    vector::Vector,
};
use core::f32;
use sdl2::pixels::Color as SdlColor;

const CLEAR_COLOR: SdlColor = SdlColor::WHITE;

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

///
/// Calculate direction light angle incidence on the face
///
fn get_light_incidence(face: &Face, directional_light: &Vector) -> f32 {
    let norm = face.normal();
    let incidence = norm.dot(directional_light) * -1.0;

    if incidence < 0.0 {
        // the face is with the back-side twords the light, no illumination
        return 0.0;
    }

    incidence
}

fn get_ray_color(ray: &Ray, directional_light: &Vector, faces: &Vec<Face>) -> Option<Color> {
    match get_ray_face(ray, faces) {
        Some(face) => {
            let incidence = get_light_incidence(face, directional_light);
            Some(face.color().scale(incidence))
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
                canvas.set_pixel(x as i32, y as i32, color.into());
            }
        }
    }

    canvas.present();
}
