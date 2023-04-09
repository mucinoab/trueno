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
    Dielectric(Dielectric),
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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            index_of_refraction,
        }
    }

    /// Use Schlick's approximation for reflectance
    fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1. - ref_idx) / (1. + ref_idx).powi(2);

        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(self, ray_in: &Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction = if refraction_ratio * sin_theta > 1.0
            || self.reflectance(cos_theta, refraction_ratio) > fastrand::f32()
        {
            unit_direction.reflect(rec.normal) // No refraction
        } else {
            unit_direction.refract(rec.normal, refraction_ratio) // No reflection
        };

        Some((Ray::new(rec.point, direction), Color::new(1., 1., 1.)))
    }
}
