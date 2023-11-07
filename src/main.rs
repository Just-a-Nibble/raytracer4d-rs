use raytracer4d_rs::{
    hittable::{Hittable, Hypersphere},
    Point4, Ray, Vec4,
};

fn main() {
    let (width, height) = (100, 50);

    let hypersphere = Hypersphere::new(Point4::new(0.0, 0.0, 0.0, 0.0), 5.0);

    let (x_step, y_step) = (2.0 / width as f32, 2.0 / height as f32);

    let (mut u, mut v) = (-1.0, -1.0);
    for _y in 0..height {
        for _x in 0..width {
            let ray = Ray::new(Point4::new(0.0, 0.0, -10.0, 0.0), Vec4::new(u, v, 1.0, 0.0));

            print!(
                "{}",
                if hypersphere.hit(&ray).is_some() {
                    'X'
                } else {
                    ' '
                }
            );

            u += x_step;
        }

        print!("\n");

        u = -1.0;
        v += y_step;
    }
}
