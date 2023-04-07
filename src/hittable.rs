use std::sync::Arc;

use crate::{
    ray::{Point3, Ray},
    sphere::Sphere,
    vec3::Vec3,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f32) -> Self {
        Self {
            point,
            normal,
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = r.direction.dot(self.normal) < 0.;

        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub enum Obj {
    Sphere(Sphere),
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<Obj>>,
}

impl HittableList {
    pub fn add(&mut self, o: Arc<Obj>) {
        self.objects.push(o);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            let possible_hit = match obj.as_ref() {
                Obj::Sphere(s) => s.hit(r, t_min, closest_so_far),
            };

            if let Some(hit) = possible_hit {
                closest_so_far = hit.t;
                hit_record = possible_hit;
            }
        }

        hit_record
    }
}
