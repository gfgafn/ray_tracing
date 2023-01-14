pub mod hittable_list;
pub mod sphere;

use crate::{material::Material, point::Point3, ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    #[inline]
    pub fn position(&self) -> Point3 {
        self.p
    }

    #[inline]
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    #[inline]
    pub fn material(&self) -> &dyn Material {
        self.material
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        assert!((0.95..1.05).contains(&outward_normal.len()));
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
