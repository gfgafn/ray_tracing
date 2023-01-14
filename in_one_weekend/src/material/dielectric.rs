use rand::random;

use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::{reflect, refract, Attenuation, Scatter, ScatterRecord};

pub struct Dielectric {
    // Index of Refraction
    ior: f32,
}

impl Dielectric {
    pub fn new(ior: f32) -> Self {
        Self { ior }
    }

    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let refract_ration: f32 = match hit_record.is_front_face() {
            // IOR of air is 1.0
            true => 1.0 / self.ior,
            false => self.ior,
        };

        let ray_in_dir_unit: Vec3 = ray_in.direction().unit_vector();
        let cos_theta: f32 = f32::min((-ray_in_dir_unit).dot(hit_record.normal()), 1.0);
        let sin_theta: f32 = (1.0 - cos_theta.powi(2)).abs().sqrt();

        let direction: Vec3 = match 1.0 < refract_ration * sin_theta
            || random::<f32>() < Self::reflectance(cos_theta, refract_ration)
        {
            true => reflect(ray_in_dir_unit, hit_record.normal()),
            false => refract(
                ray_in.direction().unit_vector(),
                hit_record.normal(),
                refract_ration,
            ),
        };

        Some(ScatterRecord::new(
            Ray::new(hit_record.position(), direction),
            Attenuation::new(Vec3::new(1.0, 1.0, 1.0)),
        ))
    }
}
