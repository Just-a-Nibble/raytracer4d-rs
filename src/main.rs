use std::path::Path;

use image::{Rgb, RgbImage};

use raytracer4d_rs::{
    hittable::{Hittable, Hyperplane, Hypersphere},
    Point4, Ray, Vec4,
};

fn main() {
    let (width, height) = (512, 512);

    let mut image = RgbImage::new(width, height);

    let hypersphere = Hypersphere::new(Point4::new(0.0, 0.0, 0.0, 0.0), 5.0);

    let hyperplane = Hyperplane::new_from_points(
        Point4::new(0.0, 5.0, 0.0, 0.0),
        Point4::new(1.0, 5.0, 0.0, 0.0),
        Point4::new(0.0, 5.0, 1.0, 0.0),
        Point4::new(0.0, 5.0, 0.0, 1.0),
    );

    let (x_step, y_step) = (2.0 / width as f32, 2.0 / height as f32);

    let (mut u, mut v) = (-1.0, -1.0);
    for y in 0..height {
        for x in 0..width {
            let ray = Ray::new(Point4::new(0.0, 0.0, -10.0, 0.0), Vec4::new(u, v, 1.0, 0.0));

            let hit = match (hypersphere.hit(&ray), hyperplane.hit(&ray)) {
                (None, None) => None,
                (Some(hit), None) => Some(hit),
                (None, Some(hit)) => Some(hit),
                (Some(hit0), Some(hit1)) => {
                    if hit0.t < hit1.t {
                        Some(hit0)
                    } else {
                        Some(hit1)
                    }
                }
            };

            image.put_pixel(
                x,
                y,
                if hit.is_some() {
                    Rgb([255, 255, 255])
                } else {
                    Rgb([0, 0, 0])
                },
            );

            u += x_step;
        }

        u = -1.0;
        v += y_step;
    }

    if let Err(err) = image.save(Path::new("./output.png")) {
        println!("Failed to save output image: {}", err);
    }
}
