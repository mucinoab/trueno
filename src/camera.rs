use crate::{
    ray::{Point3, Ray},
    vec3::Vec3,
    ASPECT_RATIO,
};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    // vertical field-of-view in degrees
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f32) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = ASPECT_RATIO * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn ger_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
