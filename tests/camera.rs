use rray::{camera::Camera, point::Point, ray::Ray, vector::Vector};

fn expected_rays() -> Vec<Ray> {
    let mut rays = Vec::new();

    for y in -2..=3 {
        for x in -2..=1 {
            let fx = (x as f32) + 0.5;
            let fy = 0.5 - (y as f32);

            rays.push(Ray::new(
                Point::new(fx, fy, -10.0),
                Vector::new(-fx, -fy, 10.0),
            ));
        }
        println!();
    }

    rays
}

#[test]
fn get_rays() {
    let mut camera = Camera::new(4, 6, 10);
    let rays = camera.get_rays();

    assert_eq!(rays, &expected_rays());
}
