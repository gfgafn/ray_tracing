use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::{Attenuation, Scatter, ScatterRecord};

pub struct Lambertian {
    albedo: Attenuation,
}

impl Lambertian {
    pub fn new(albedo: Attenuation) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction: Vec3 = hit_record.normal() + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal()
        };

        Some(ScatterRecord::new(
            Ray::new(hit_record.position(), scatter_direction),
            self.albedo,
        ))
    }
}
