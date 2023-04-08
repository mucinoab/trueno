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

const ASPECT_RATIO: f32 = 16. / 9.;

#[cfg(debug_assertions)]
const IMAGE_WIDTH: f32 = 400.0;

#[cfg(not(debug_assertions))]
const IMAGE_WIDTH: f32 = 1920.0;

const IMAGE_HEIGHT: f32 = IMAGE_WIDTH / ASPECT_RATIO;

#[cfg(debug_assertions)]
const SAMPLES_PER_PIXEL: usize = 64;

#[cfg(not(debug_assertions))]
const SAMPLES_PER_PIXEL: usize = 128;

const MULTIPLICATIVE_INVERSE_OF_SAMPLES_PER_PIXEL: f32 = 1. / SAMPLES_PER_PIXEL as f32;

#[cfg(debug_assertions)]
const MAX_DEPTH: u8 = 16;

#[cfg(not(debug_assertions))]
const MAX_DEPTH: u8 = 64;

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

    // camera
    let camera = Camera::new();

    eprintln!(
        "Pixels to generate:{}x{} =  {}",
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        IMAGE_WIDTH * IMAGE_HEIGHT
    );

    let coords: Vec<_> = (0..(IMAGE_HEIGHT as u32))
        .rev()
        .flat_map(|x| (0..(IMAGE_WIDTH as u32)).map(move |y| (x, y)))
        .collect();

    let rng = fastrand::Rng::new();
    let pixels: Vec<_> = coords
        .into_par_iter()
        .map_with(rng, |r, (x, y)| {
            let mut pixel_color = Color::default();
            let x = x as f32;
            let y = y as f32;

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (y + r.f32()) / IMAGE_WIDTH;
                let v = (x + r.f32()) / IMAGE_HEIGHT;
                let r = camera.ger_ray(u, v);

                pixel_color += r.color(&WORLD, MAX_DEPTH);
            }

            // Translate to [0,255] value of each color component.
            ((pixel_color * MULTIPLICATIVE_INVERSE_OF_SAMPLES_PER_PIXEL).sqrt()).clamp(0., 0.999)
                * 256.0
        })
        .collect();

    // Write image
    let mut buf: Vec<u8> = Vec::with_capacity(40_000_000);
    writeln!(&mut buf, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255").unwrap();

    for pixel in pixels.iter() {
        let r = pixel.x() as u8;
        let g = pixel.y() as u8;
        let b = pixel.z() as u8;

        writeln!(&mut buf, "{r} {g} {b}").unwrap();
    }

    let mut file = File::create("./image.ppm").unwrap();
    file.write_all(&buf).unwrap();
}
