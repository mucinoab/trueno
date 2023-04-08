use crate::{
    hittable::{HitRecord, Hittable},
    ray::{Point3, Ray},
    vec3::Vec3,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
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

        let mut hr = HitRecord::new(point, (point - self.center) / self.radius, root);
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
