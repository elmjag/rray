use crate::{
    camera::Camera,
    canvas::RenderCanvas,
    intersect::ray_triangle_intersection,
    mesh::{Face, Mesh},
    ray::Ray,
};

fn find_first_ray_hit(ray: &Ray, faces: &Vec<Face>) -> Option<(f32, f32, f32)> {
    for face in faces {
        let r = ray_triangle_intersection(ray, face);
        if r.is_some() {
            return r;
        }
    }

    None
}

pub fn draw_frame(canvas: &mut impl RenderCanvas, camera: &Camera, mesh: &mut Mesh) {
    let faces = mesh.get_faces();

    let (w, h) = canvas.size();

    canvas.clear();

    for y in 0..h {
        for x in 0..w {
            let ray = camera.get_pixel_ray(x, y);

            let r = find_first_ray_hit(&ray, &faces);

            if r.is_some() {
                let Some((u, v, t)) = r else { todo!() };
                let (r, g, b) = (
                    (u * 255.0) as u8,
                    (v * 255.0) as u8,
                    ((t - 1.0) * 255.0) as u8,
                );
                //screen.set_pixel(x as i32, y as i32, 100, 200, 233);
                canvas.set_pixel(x as i32, y as i32, r, g, b);
            }
        }
    }

    canvas.present();
}
