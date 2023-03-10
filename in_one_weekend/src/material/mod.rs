mod dielectric;
mod lambertian;
mod metal;

pub use self::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

use std::ops;

use crate::{
    color::{ColorRGB, ColorRGBMapTo0_1},
    hittable::HitRecord,
    ray::Ray,
    vec3::Vec3,
};

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

pub trait Material: Scatter {}

impl<T: Scatter> Material for T {}

pub struct ScatterRecord {
    ray: Ray,
    albedo: Attenuation,
}

impl ScatterRecord {
    pub fn new(ray_scattered: Ray, attenuation: Attenuation) -> Self {
        Self {
            ray: ray_scattered,
            albedo: attenuation,
        }
    }

    #[inline]
    pub fn ray_scattered(&self) -> &Ray {
        &self.ray
    }

    #[inline]
    pub fn albedo(&self) -> Attenuation {
        self.albedo
    }
}

#[derive(Clone, Copy)]
pub struct Attenuation(Vec3);

impl Attenuation {
    pub fn new(val: Vec3) -> Self {
        assert!((0.0..=1.0).contains(&val.0));
        assert!((0.0..=1.0).contains(&val.1));
        assert!((0.0..=1.0).contains(&val.2));
        Self(val)
    }

    pub fn random() -> Self {
        Self(Vec3::random())
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        Self(Vec3::random_range(min, max))
    }
}

impl ops::Mul<ColorRGB> for Attenuation {
    type Output = ColorRGB;

    fn mul(self, rhs: ColorRGB) -> Self::Output {
        ColorRGB::new(
            (self.0 .0 * rhs.r() as f32) as u8,
            (self.0 .1 * rhs.g() as f32) as u8,
            (self.0 .2 * rhs.b() as f32) as u8,
        )
    }
}

impl ops::Mul<ColorRGBMapTo0_1> for Attenuation {
    type Output = ColorRGBMapTo0_1;

    fn mul(self, rhs: ColorRGBMapTo0_1) -> Self::Output {
        ColorRGBMapTo0_1::new(
            self.0 .0 * rhs.r(),
            self.0 .1 * rhs.g(),
            self.0 .2 * rhs.b(),
        )
    }
}

impl ops::Mul<Self> for Attenuation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

fn reflect(vec_in: Vec3, normal: Vec3) -> Vec3 {
    vec_in - 2.0 * Vec3::dot(vec_in, normal) * normal
}

fn refract(unit_vec_in: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta: f32 = f32::min((-unit_vec_in).dot(normal), 1.0);
    let ray_out_perp: Vec3 = etai_over_etat * (unit_vec_in + cos_theta * normal);
    let ray_out_parallel: Vec3 = -(1.0 - ray_out_perp.len_squared()).abs().sqrt() * normal;

    ray_out_perp + ray_out_parallel
}
