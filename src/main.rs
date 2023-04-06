#![feature(portable_simd)]

mod color;
mod ray;
mod vec3;

use std::{fs::File, io::BufWriter, io::Write, ops::Mul};

use ray::{Point3, Ray};
use vec3::Vec3;

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 400.;
    let image_height = image_width / aspect_ratio;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    // Write image
    let mut file = BufWriter::with_capacity(10_000_000, File::create("./image.ppm").unwrap());
    write!(&mut file, "P3\n{image_width} {image_height}\n255\n").unwrap();

    for j in (0..(image_height as usize)).rev() {
        eprint!("\rScanlines remaining: {j}");

        for i in 0..(image_width as usize) {
            let u = i as f32 / (image_width - 1.);
            let v = j as f32 / (image_height - 1.);

            let r = Ray::new(
                origin,
                lower_left_corner + horizontal.mul(u) + vertical.mul(v) - origin,
            );

            r.color().write(&mut file);
        }
    }

    file.flush().unwrap();
}
