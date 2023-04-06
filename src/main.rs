#![feature(portable_simd, lazy_cell)]

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use crate::{color::Color, hittable::Obj};
use camera::Camera;
use hittable::HittableList;
use ray::Point3;
use sphere::Sphere;

use std::{
    fs::File,
    io::Write,
    sync::{Arc, LazyLock},
};

use rayon::prelude::*;

fn _random_f32(min: f32, max: f32) -> f32 {
    let rng = fastrand::Rng::new();
    rng.seed(42);
    dbg!(rng.f32());

    min + (max - min) * rng.f32()
}

const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_WIDTH: f32 = 400.0;

static WORLD: LazyLock<HittableList> = LazyLock::new(|| {
    let mut world = HittableList::default();

    world.add(Arc::new(Obj::Sphere(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
    ))));

    world.add(Arc::new(Obj::Sphere(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.0,
    ))));

    world
});

fn main() {
    // Image
    let image_height = IMAGE_WIDTH / ASPECT_RATIO;
    let samples_per_pixel = 10;

    // camera
    let camera = Camera::new();

    // Write image
    let mut buf: Vec<u8> = Vec::with_capacity(100_000_000);
    write!(&mut buf, "P3\n{IMAGE_WIDTH} {image_height}\n255\n").unwrap();

    eprintln!(
        "Pixels to generate:{}x{} =  {}",
        IMAGE_WIDTH,
        image_height,
        IMAGE_WIDTH * image_height
    );

    let coords: Vec<_> = (0..(image_height as usize))
        .rev()
        .flat_map(|x| (0..(IMAGE_WIDTH as usize)).map(move |y| (x, y)))
        .collect();

    let pixels: Vec<_> = coords
        .into_par_iter()
        .map(|(j, i)| {
            let rng = fastrand::Rng::new();
            let mut pixel_color = Color::default();

            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.f32()) / (IMAGE_WIDTH - 1.);
                let v = (j as f32 + rng.f32()) / (image_height - 1.);
                let r = camera.ger_ray(u, v);

                pixel_color += r.color(&WORLD);
            }

            pixel_color
        })
        .collect();

    for pixel_color in pixels.iter() {
        pixel_color.write(&mut buf, samples_per_pixel);
    }

    let mut file = File::create("./image.ppm").unwrap();
    file.write_all(&buf).unwrap();
    file.flush().unwrap();
}
