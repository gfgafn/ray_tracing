pub mod hittable_list;
pub mod sphere;

use in_one_weekend::{point::Point3, vec3::Vec3};

use crate::{material::Material, ray::Ray};

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

    #[inline]
    pub fn is_front_face(&self) -> bool {
        self.front_face
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        debug_assert!((0.99..1.01).contains(&outward_normal.len()));
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
