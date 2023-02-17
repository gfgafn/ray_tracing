use in_one_weekend::vec3::Vec3;
use rand::random;

use crate::{material::Isotropic, ray::Ray, textures::Texture};

use super::{HitRecord, Hittable};

pub struct ConstantMedium<H: Hittable, T: Texture> {
    boundary: H,
    phase_function: Isotropic<T>,
    neg_inv_density: f32,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, texture: T, density: f32) -> Self {
        Self {
            boundary,
            phase_function: Isotropic::new(texture),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl<H, T> Hittable for ConstantMedium<H, T>
where
    H: Hittable,
    T: Texture + Send + Sync,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rec1 = self.boundary.hit(ray, f32::NEG_INFINITY, f32::INFINITY)?;
        let rec2 = self.boundary.hit(ray, rec1.t + 0.0001, f32::INFINITY)?;

        let [mut t1, mut t2] = [rec1.t, rec2.t];

        if t1 < t_min {
            t1 = t_min
        };
        if t2 > t_max {
            t2 = t_max
        };
        if t1 >= t2 {
            return None;
        }

        if t1 < 0.0 {
            t1 = 0.0;
        }

        let length_per_unit = ray.direction().len();
        let distance_inside = (t2 - t1) * length_per_unit;
        let hit_distance = self.neg_inv_density * random::<f32>().ln();

        if hit_distance > distance_inside {
            return None;
        }

        let hit_point_unit = t1 + hit_distance / length_per_unit;

        Some(HitRecord {
            p: ray.at(hit_point_unit),
            normal: Vec3::new(1.0, 0.0, 0.0), // useless,
            t: hit_point_unit,
            front_face: true, // useless
            material: &self.phase_function,
            uv: [0.0, 0.0], // useless
        })
    }
}
