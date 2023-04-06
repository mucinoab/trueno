use std::rc::Rc;

use crate::{
    ray::{Point3, Ray},
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

#[derive(Default, Clone)]
pub struct HittableList {
    // Dyn or an enum???
    objects: Vec<Rc<dyn Hittable>>, // TODO shared_ptr =? or arc and do it multithread
}

impl HittableList {
    pub fn add(&mut self, o: Rc<dyn Hittable>) {
        self.objects.push(o);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = HitRecord::default();
        let mut closest_so_far = t_max;
        let mut hitted = false;

        for obj in self.objects.iter() {
            // Rayon?
            if let Some(hit) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = hit;
                hitted = true;
            }
        }

        if hitted {
            Some(hit_record)
        } else {
            None
        }
    }
}
