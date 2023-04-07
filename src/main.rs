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

#[cfg(debug_assertions)]
const IMAGE_WIDTH: f32 = 400.0;

#[cfg(not(debug_assertions))]
const IMAGE_WIDTH: f32 = 1920.0;

#[cfg(debug_assertions)]
const SAMPLES_PER_PIXEL: usize = 64;

#[cfg(not(debug_assertions))]
const SAMPLES_PER_PIXEL: usize = 128;

const MULTIPLICATIVE_INVERSE_OF_SAMPLES_PER_PIXEL: f32 = 1. / SAMPLES_PER_PIXEL as f32;

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

    // camera
    let camera = Camera::new();

    eprintln!(
        "Pixels to generate:{}x{} =  {}",
        IMAGE_WIDTH,
        image_height,
        IMAGE_WIDTH * image_height
    );

    let coords: Vec<_> = (0..(image_height as u32))
        .rev()
        .flat_map(|x| (0..(IMAGE_WIDTH as u32)).map(move |y| (x, y)))
        .collect();

    let rng = fastrand::Rng::new();
    let pixels: Vec<_> = coords
        .into_par_iter()
        .map_with(rng, |r, (x, y)| {
            let x = x as f32;
            let y = y as f32;

            let mut rndm = [(0., 0.); SAMPLES_PER_PIXEL];

            for (dx, dy) in rndm.iter_mut() {
                *dx = y + r.f32();
                *dy = x + r.f32();
            }

            rndm
        })
        .map(|rndm| {
            let mut pixel_color = Color::default();

            for (di, dj) in rndm {
                let u = di / IMAGE_WIDTH;
                let v = dj / image_height;
                let r = camera.ger_ray(u, v);

                pixel_color += r.color(&WORLD);
            }

            // Translate to [0,255] value of each color component.
            (pixel_color * MULTIPLICATIVE_INVERSE_OF_SAMPLES_PER_PIXEL).clamp(0., 0.999) * 256.0
        })
        .collect();

    // Write image
    let mut buf: Vec<u8> = Vec::with_capacity(40_000_000);
    writeln!(&mut buf, "P3\n{IMAGE_WIDTH} {image_height}\n255").unwrap();

    for pixel in pixels.iter() {
        let r = pixel.x() as u8;
        let g = pixel.y() as u8;
        let b = pixel.z() as u8;

        writeln!(&mut buf, "{r} {g} {b}").unwrap();
    }

    let mut file = File::create("./image.ppm").unwrap();
    file.write_all(&buf).unwrap();
}
