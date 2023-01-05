pub mod hittable_list;
pub mod sphere;

use crate::{point::Point3, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    p: Point3,
    pub normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if true == self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
