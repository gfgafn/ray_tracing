use in_one_weekend::vec3::Vec3;

use crate::{hittable::HitRecord, ray::Ray, textures::Texture};

use super::{Scatter, ScatterRecord};

pub struct Lambertian<T: Texture> {
    texture: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Self { texture }
    }
}

impl<T: Texture + Send + Sync> Scatter for Lambertian<T> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction: Vec3 = hit_record.normal() + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal()
        };

        Some(ScatterRecord::new(
            Ray::new(hit_record.position(), scatter_direction, ray_in.time()),
            self.texture
                .value(hit_record.u(), hit_record.v(), &hit_record.position())
                .into(),
        ))
    }
}
