use crate::{color::Color, vec3::Vec3};

use std::ops::Mul;

pub type Point3 = Vec3;

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
        self.origin + self.direction.mul(t)
    }
}

impl Ray {
    pub fn color(&self) -> Color {
        let t = self.hit_sphere(Point3::new(0., 0., -1.), 0.5);

        if t > 0. {
            let n = (self.at(t) - Vec3::new(0., 0., -1.)).unit_vector();
            return (n + Vec3::new(1., 1., 1.)) * 0.5;
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        Color::new(1.0, 1.0, 1.0).mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t)
    }

    /// t2b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r2=0
    fn hit_sphere(&self, center: Point3, radius: f32) -> f32 {
        let oc = self.origin - center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - radius.powi(2);

        // General formula
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            -1.
        } else {
            (-b - discriminant.sqrt()) / (2. * a)
        }
    }
}
