use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::{refract, Attenuation, Scatter, ScatterRecord};

pub struct Dielectric {
    // Index of Refraction
    ior: f32,
}

impl Dielectric {
    pub fn new(ior: f32) -> Self {
        Self { ior }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let refract_ration: f32 = match hit_record.is_front_face() {
            // IOR of air is 1.0
            true => 1.0 / self.ior,
            false => self.ior,
        };

        let ray_refracted_direction: Vec3 = refract(
            ray_in.direction().unit_vector(),
            hit_record.normal(),
            refract_ration,
        );

        Some(ScatterRecord::new(
            Ray::new(hit_record.position(), ray_refracted_direction),
            Attenuation::new(Vec3::new(1.0, 1.0, 1.0)),
        ))
    }
}
