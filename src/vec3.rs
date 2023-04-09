use std::{
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
    simd::{Simd, SimdFloat, StdFloat},
};

use fastrand::Rng;

pub type Color = Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    v: Simd<f32, 4>,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            v: Simd::from([x, y, z, 0.]),
        }
    }

    pub fn random() -> Self {
        let rng = fastrand::Rng::new();

        let x = rng.f32();
        let y = rng.f32();
        let z = rng.f32();

        Self {
            v: Simd::from([x, y, z, 0.]),
        }
    }

    pub fn random_in_range(min: f32, max: f32) -> Self {
        let rng = fastrand::Rng::new();

        let x = random_f32(min, max, &rng);
        let y = random_f32(min, max, &rng);
        let z = random_f32(min, max, &rng);

        Self {
            v: Simd::from([x, y, z, 0.]),
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let rng = fastrand::Rng::new();

        loop {
            let p = Vec3::new(random_f32(-1., 1., &rng), random_f32(-1., 1., &rng), 0.);
            if p.len_squared() < 1. {
                return p;
            }
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.v[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.v[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.v[2]
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        (self.v * self.v).reduce_sum()
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.mul(other).v.reduce_sum()
    }

    pub fn cross(&self, other: Vec3) -> Self {
        let x = self.y() * other.z() - self.z() * other.y();
        let y = self.z() * other.x() - self.x() * other.z();
        let z = self.x() * other.y() - self.y() * other.x();

        Self::new(x, y, z)
    }

    pub fn unit_vector(&self) -> Self {
        self.div(self.len())
    }

    pub fn clamp(&self, min: f32, max: f32) -> Self {
        let min = Simd::from([min, min, min, min]);
        let max = Simd::from([max, max, max, max]);

        let v: Simd<f32, 4> = self.v.simd_clamp(min, max);

        Self { v }
    }

    ///(self * a) + b
    pub fn mul_add_vec(&self, a: f32, b: Self) -> Self {
        let a = Simd::from([a, a, a, 0.]);

        Self {
            v: self.v.mul_add(a, b.v),
        }
    }

    pub fn sqrt(&self) -> Self {
        Self { v: self.v.sqrt() }
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

    pub fn reflect(&self, n: Self) -> Self {
        *self - n.mul(2.).mul(self.dot(n))
    }

    /// Solve Snell’s law
    pub fn refract(&self, n: Vec3, etai_over_etat: f32) -> Self {
        let cos_theta = (-*self).dot(n).min(1.0);

        let r_out_perp = etai_over_etat * (*self + cos_theta * n);

        let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * n;

        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            v: self.v.add(other.v),
        }
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs.add(self)
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, scalar: f32) -> Self::Output {
        let other = Simd::from([scalar, scalar, scalar, 0.]);

        Self {
            v: self.v.mul(other),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            v: self.v.sub(other.v),
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            v: self.v.mul(other.v),
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.mul(self)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        let other = Simd::from([scalar, scalar, scalar, 0.]);

        Self {
            v: self.v.mul(other),
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, mut other: Vec3) -> Self::Output {
        other.v[3] = 1.;
        let v = self.v.div(other.v);
        other.v[3] = 0.;

        Self { v }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        let other = Simd::from([scalar, scalar, scalar, 1.]);

        Self {
            v: self.v.div(other),
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { v: -self.v }
    }
}

pub fn random_f32(min: f32, max: f32, rng: &Rng) -> f32 {
    min + (max - min) * rng.f32()
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

        let a = Vec3::new(1., -3., 4.);
        assert_eq!(a.len_squared(), 26.0_f32);
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
