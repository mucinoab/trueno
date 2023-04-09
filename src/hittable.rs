use crate::{
    material::Materials,
    ray::{Point3, Ray},
    sphere::Sphere,
    vec3::Vec3,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Materials,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f32, m: Materials) -> Self {
        Self {
            point,
            normal,
            t,
            front_face: false,
            material: m,
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

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Sphere>,
}

impl HittableList {
    pub fn add(&mut self, o: Sphere) {
        self.objects.push(o);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|s| {
            if let Some(hit) = s.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        });

        hit_record
    }
}
