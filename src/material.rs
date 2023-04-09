use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    sphere::{random_in_unit_sphere, random_unit_vector},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
}

pub trait Material {
    fn scatter(self, ray_in: &Ray, rec: HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(self, _ray_in: &Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((Ray::new(rec.point, scatter_direction), self.albedo))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(std::f32::MIN, 1.),
        }
    }
}

impl Material for Metal {
    fn scatter(self, ray_in: &Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
        let scattered = Ray::new(
            rec.point,
            random_in_unit_sphere().mul_add_vec(self.fuzz, reflected),
        );

        if scattered.direction.dot(rec.normal) > 0. {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
