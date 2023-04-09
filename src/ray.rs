use crate::{
    color::Color,
    hittable::{Hittable, HittableList},
    material::{Material, Materials},
    vec3::Vec3,
};

use std::{f32::INFINITY, ops::Mul};

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

/// How to Ray Trace
/// (1) calculate the ray from the eye to the pixel.
/// (2) determine which objects the ray intersects.
/// (3) compute a color for that intersection point.
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// P(t)=A+tb. Here P is a 3D position along a line in 3D. A is the ray origin and b is the ray
    /// direction. The ray parameter t is a real number (double in the code). Plug in a different t
    /// and P(t) moves the point along the ray. Add in negative t values and you can go anywhere on
    /// the 3D line. For positive t, you get only the parts in front of A, and this is what is
    /// often called a half-line or ray.
    pub fn at(&self, t: f32) -> Point3 {
        self.direction.mul_add_vec(t, self.origin)
    }
}

impl Ray {
    pub fn color(&self, world: &HittableList, depth: u8) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Color::new(0., 0., 0.);
        }

        if let Some(hit) = world.hit(self, 0.001, INFINITY) {
            if let Some((scattered, attenuation)) = match hit.material.as_ref() {
                Materials::Lambertian(l) => l.scatter(self, hit),
                Materials::Metal(m) => m.scatter(self, hit),
            } {
                attenuation * scattered.color(world, depth - 1)
            } else {
                Color::default()
            }
        } else {
            let unit_direction = self.direction.unit_vector();
            let t = (unit_direction.y() + 1.) * 0.5;
            Color::new(1., 1., 1.).mul_add_vec(1. - t, Color::new(0.5, 0.7, 1.).mul(t))
        }
    }
}
