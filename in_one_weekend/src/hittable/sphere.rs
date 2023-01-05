use crate::{point::Point3, ray::Ray, vec3::Vec3};

use super::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let a: f32 = ray.direction().len_squared();
        let half_b: f32 = ray.direction().dot(oc);
        let c: f32 = oc.len_squared() - self.radius.powf(2.);
        let discriminant: f32 = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let dis_sqrt: f32 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut t: f32 = (-half_b - dis_sqrt) / a;
        if t < t_min || t_max < t {
            t = (-half_b + dis_sqrt) / a;
            if t < t_min || t_max < t {
                return None;
            }
        }

        let p: Point3 = ray.at(t);

        Some(HitRecord {
            t,
            p,
            normal: (p - self.center) / self.radius,
        })
    }
}
