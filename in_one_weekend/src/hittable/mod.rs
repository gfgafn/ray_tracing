mod sphere;

use crate::{point::Point3, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
