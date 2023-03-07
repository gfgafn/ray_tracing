use in_one_weekend::vec3::Vec3;

use crate::{hittable::HitRecord, ray::Ray};

use super::{reflect, Attenuation, Emit, Scatter, ScatterRecord};

pub struct Metal {
    albedo: Attenuation,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Attenuation, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Emit for Metal {}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflect_direction: Vec3 =
            reflect(ray_in.direction().unit_vector(), hit_record.normal());

        if reflect_direction.dot(hit_record.normal()) <= 0. {
            return None;
        }

        Some(ScatterRecord::new(
            Ray::new(
                hit_record.position(),
                reflect_direction + self.fuzz * Vec3::random_in_unit_sphere(),
                ray_in.time(),
            ),
            self.albedo,
        ))
    }
}
