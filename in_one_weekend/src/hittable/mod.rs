pub mod hittable_list;
pub mod sphere;

use crate::{point::Point3, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    #[inline]
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        assert!((0.95..1.05).contains(&outward_normal.len()));
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
