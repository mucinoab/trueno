use crate::{
    hittable::{HitRecord, Hittable},
    material::Materials,
    ray::{Point3, Ray},
    vec3::Vec3,
};

use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Arc<Materials>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, m: Materials) -> Self {
        Self {
            center,
            radius,
            material: Arc::new(m),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.len_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.len_squared() - self.radius.powi(2);

        let discriminant = half_b.mul_add(half_b, -a * c);

        if discriminant < 0. {
            return None;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = r.at(root);

        let mut hr = HitRecord::new(
            point,
            (point - self.center) / self.radius,
            root,
            *self.material,
        );
        hr.set_face_normal(r);

        Some(hr)
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in_range(-1., 1.);

        if p.len_squared() < 1. {
            return p;
        }
    }
}

pub fn _random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}
