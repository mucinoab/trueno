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
    pub fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.
                - vertical / 2.
                - Vec3::new(0., 0., focal_length),
        }
    }
    pub fn ger_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.horizontal.mul_add_vec(u, self.lower_left_corner)
            + self.vertical.mul_add_vec(v, -self.origin);

        Ray::new(self.origin, direction)
    }
}
