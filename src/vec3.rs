use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    simd::Simd,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        let c = self.mul(other);
        c.x + c.y + c.z
    }

    pub fn cross(&self, other: Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        self.div(self.len())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let a = std::simd::Simd::from([self.x, self.y, self.z, 0.]);
        let b = std::simd::Simd::from([other.x, other.y, other.z, 0.]);

        let c: Simd<f32, 4> = a.add(b);

        Self {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let a = std::simd::Simd::from([self.x, self.y, self.z, 0.]);
        let b = std::simd::Simd::from([other.x, other.y, other.z, 0.]);

        let c: Simd<f32, 4> = a.sub(b);

        Self {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let a = std::simd::Simd::from([self.x, self.y, self.z, 0.]);
        let b = std::simd::Simd::from([other.x, other.y, other.z, 0.]);

        let c: Simd<f32, 4> = a.mul(b);

        Self {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        let a = std::simd::Simd::from([self.x, self.y, self.z, 0.]);
        let b = std::simd::Simd::from([scalar, scalar, scalar, 0.]);

        let c: Simd<f32, 4> = a.mul(b);

        Self {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> Self::Output {
        let a = std::simd::Simd::from([self.x, self.y, self.z, 1.]);
        let b = std::simd::Simd::from([other.x, other.y, other.z, 1.]);

        let c: Simd<f32, 4> = a.div(b);

        Self {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        let a = std::simd::Simd::from([self.x, self.y, self.z, 0.]);
        let b = std::simd::Simd::from([scalar, scalar, scalar, 0.]);

        let c: Simd<f32, 4> = a.div(b);

        Self {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[must_use = "operator returns a new vector without mutating the input"]
    fn neg(self) -> Self::Output {
        let c = -std::simd::Simd::from([self.x, self.y, self.z, 0.]);

        Self {
            x: c[0],
            y: c[1],
            z: c[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn sum() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(1., 2., 3.);

        assert_eq!(a + b, Vec3::new(2., 4., 6.));
    }

    #[test]
    fn sub() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(1., 2., 3.);

        assert_eq!(a - b, Vec3::new(0., 0., 0.));
    }

    #[test]
    fn neg() {
        let a = Vec3::new(1., 2., 3.);

        assert_eq!(-a, Vec3::new(-1., -2., -3.));
        assert_eq!(a, Vec3::new(1., 2., 3.));
    }

    #[test]
    fn mul() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(2., 4., 6.);

        assert_eq!(a * b, Vec3::new(2., 8., 18.));
    }

    #[test]
    fn mul_scalar() {
        let a = Vec3::new(1., 2., 3.);
        let b = 3.;

        assert_eq!(a * b, Vec3::new(3., 6., 9.));
    }

    #[test]
    fn div() {
        let a = Vec3::new(2., 4., 8.);
        let b = Vec3::new(2., 2., 4.);

        assert_eq!(a / b, Vec3::new(1., 2., 2.));
    }

    #[test]
    fn div_scalar() {
        let a = Vec3::new(2., 4., 8.);
        let b = Vec3::new(1., 2., 4.);

        assert_eq!(a / 2., b);
    }

    #[test]
    fn dot() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., -5., 6.);

        assert_eq!(a.dot(b), 12.);
    }

    #[test]
    fn len() {
        let a = Vec3::new(1., -3., 4.);

        assert_eq!(a.len(), (26.0_f32).sqrt());
    }

    #[test]
    fn unit() {
        let a = Vec3::new(1., -3., 4.);
        let len = a.len();

        assert_eq!(a.unit_vector(), Vec3::new(1. / len, -3. / len, 4. / len));
    }

    #[test]
    fn cross() {
        let a = Vec3::new(2., 3., 4.);
        let b = Vec3::new(5., 6., 7.);

        assert_eq!(a.cross(b), Vec3::new(-3., 6., -3.));
    }
}
