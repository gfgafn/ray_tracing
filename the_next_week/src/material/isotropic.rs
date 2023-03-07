use in_one_weekend::vec3::Vec3;

use crate::{hittable::HitRecord, ray::Ray, textures::Texture};

use super::{Emit, Scatter, ScatterRecord};

pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(texture: T) -> Self {
        Self { albedo: texture }
    }
}

impl<T: Texture> Emit for Isotropic<T> {}

impl<T: Texture> Scatter for Isotropic<T> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            ray: Ray::new(
                hit_record.position(),
                Vec3::random_in_unit_sphere(),
                ray_in.time(),
            ),
            albedo: self
                .albedo
                .value(hit_record.u(), hit_record.v(), &hit_record.position())
                .into(),
        })
    }
}
