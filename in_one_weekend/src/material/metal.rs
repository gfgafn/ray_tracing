use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::{reflect, Attenuation, Scatter, ScatterRecord};

pub struct Metal {
    albedo: Attenuation,
}

impl Metal {
    pub fn new(albedo: Attenuation) -> Self {
        Self { albedo }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflect_direction: Vec3 =
            reflect(ray_in.direction().unit_vector(), hit_record.normal());

        if reflect_direction.dot(hit_record.normal()) <= 0. {
            return None;
        }

        Some(ScatterRecord::new(
            Ray::new(hit_record.position(), reflect_direction),
            self.albedo,
        ))
    }
}
