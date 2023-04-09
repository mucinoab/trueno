#![feature(portable_simd, lazy_cell)]

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::color::Color;
use camera::Camera;
use hittable::HittableList;
use material::{Dielectric, Lambertian, Materials, Metal};
use ray::Point3;
use sphere::Sphere;
use vec3::Vec3;

use std::{fs::File, io::Write, sync::LazyLock};

use rayon::prelude::*;

const ASPECT_RATIO: f32 = 16. / 9.;

#[cfg(debug_assertions)]
const IMAGE_WIDTH: f32 = 400.0;

#[cfg(not(debug_assertions))]
const IMAGE_WIDTH: f32 = 1920.0;

const IMAGE_HEIGHT: f32 = IMAGE_WIDTH / ASPECT_RATIO;

#[cfg(debug_assertions)]
const SAMPLES_PER_PIXEL: usize = 10;

#[cfg(not(debug_assertions))]
//const SAMPLES_PER_PIXEL: usize = 512;
const SAMPLES_PER_PIXEL: usize = 10;

const MULTIPLICATIVE_INVERSE_OF_SAMPLES_PER_PIXEL: f32 = 1. / SAMPLES_PER_PIXEL as f32;

#[cfg(debug_assertions)]
const MAX_DEPTH: u8 = 4;

#[cfg(not(debug_assertions))]
//const MAX_DEPTH: u8 = 64;
const MAX_DEPTH: u8 = 6;

static WORLD: LazyLock<HittableList> = LazyLock::new(random_scene);

fn main() {
    // Camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Point3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let camera = Camera::new(lookfrom, lookat, vup, 20., aperture, dist_to_focus);

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

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    world.add(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.0,
        Materials::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;

            let choose_mat = fastrand::f32();
            let center = Point3::new(a + 0.9 * fastrand::f32(), 0.2, b + 0.9 * fastrand::f32());

            if (center - Point3::new(4., 0.2, 0.)).len() <= 0.8 {
                continue;
            }

            let material = if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random() * Color::random();
                Materials::Lambertian(Lambertian::new(albedo))
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random_in_range(0.5, 1.);
                let fuzz = vec3::random_f32(0., 0.5, &fastrand::Rng::new());
                Materials::Metal(Metal::new(albedo, fuzz))
            } else {
                // Glass
                Materials::Dielectric(Dielectric::new(1.5))
            };

            world.add(Sphere::new(center, 0.2, material));
        }
    }

    world.add(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.0,
        Materials::Dielectric(Dielectric::new(1.5)),
    ));

    world.add(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.0,
        Materials::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    ));

    world.add(Sphere::new(
        Vec3::new(3.5, 1., 0.),
        1.0,
        Materials::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    ));

    world
}
