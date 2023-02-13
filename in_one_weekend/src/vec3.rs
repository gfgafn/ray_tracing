use std::{fmt, ops};

use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::point::Point3;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Vec3 {
    pub fn unit_vector(&self) -> Self {
        self / self.len()
    }

    pub fn random_unit_vector() -> Self {
        Point3::random_in_unit_sphere().unit_vector()
    }

    pub fn dot(self, v: Self) -> f32 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    pub fn cross(self, v: Self) -> Self {
        Self(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        Self::dot(*self, *self)
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere: Vec3 = Point3::random_in_unit_sphere();
        if 0. < in_unit_sphere.dot(normal) {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Return true if the vector is close to zero in all dimensions.
    pub fn is_near_zero(&self) -> bool {
        const S: f32 = 10E-8;
        self.0.abs() < S && self.1.abs() < S && self.2.abs() < S
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng: ThreadRng = thread_rng();
        loop {
            let p: Vec3 = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if 1.0 > p.len_squared() {
                break p;
            }
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl ops::AddAssign<&Self> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub<&Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::Index<u8> for Vec3 {
    type Output = f32;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("can not index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_should_work() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(2.0, v[1]);
        assert_eq!(Vec3(-1.0, -2.0, -3.0), -v);
        assert_eq!(Vec3(2.0, 4.0, 6.0), v * 2f32);
        assert_eq!(Vec3(3.0, 4.0, 3.0), v * Vec3(3.0, 2.0, 1.0));
        v += Vec3(-1.0, -2.0, -3.0);
        v += &Vec3(0.0, 0.0, 0.0);
        assert_eq!(Vec3(0.0, 0.0, 0.0), v);
        v -= Vec3(-1.0, -2.0, -3.0);
        assert_eq!(Vec3(1.0, 2.0, 3.0), v);
        v *= 2f32;
        assert_eq!(Vec3(2.0, 4.0, 6.0), v);
        v /= 2f32;
        assert_eq!(Vec3(1.0, 2.0, 3.0), v);
        assert_eq!(14_f32, v.len_squared());
        assert_eq!(14_f32.sqrt(), v.len());
        assert_eq!(14_f32, v.dot(v));
        assert_eq!(14_f32, Vec3::dot(v, v));
        assert_eq!(Vec3(-4.0, 8.0, -4.0), v.cross(Vec3(3.0, 2.0, 1.0)));
        assert_eq!(Vec3(-4.0, 8.0, -4.0), Vec3::cross(v, Vec3(3.0, 2.0, 1.0)));
        const EPSILON: f32 = 0.0000001;
        assert!(1.0 - EPSILON <= v.unit_vector().len() && v.unit_vector().len() <= 1.0 + EPSILON);
    }
}
