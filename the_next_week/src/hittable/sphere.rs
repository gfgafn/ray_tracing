use in_one_weekend::{point::Point3, vec3::Vec3};

use crate::{material::Material, ray::Ray};

use super::{HitRecord, Hittable};

pub struct Sphere<M: AsRef<dyn Material>> {
    center: Point3,
    radius: f32,
    material: M,
}

impl<M: AsRef<dyn Material>> Sphere<M> {
    pub fn new(center: Point3, radius: f32, material: M) -> Self
    where
        M: AsRef<dyn Material>,
    {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: AsRef<dyn Material> + Send + Sync> Hittable for Sphere<M> {
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
        // half_b < 0 && a > 0
        // assert!((-half_b - dis_sqrt) / a <= (-half_b + dis_sqrt) / a);
        let t: f32 = (-half_b - dis_sqrt) / a;
        if !(t_min..=t_max).contains(&t) {
            return None;
        }

        let p: Point3 = ray.at(t);
        let outward_normal: Vec3 = (p - self.center) / self.radius;
        let mut hit_record = HitRecord {
            t,
            p,
            normal: outward_normal,
            front_face: true,
            material: self.material.as_ref(),
        };
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}

pub struct MovingSphere<M: AsRef<dyn Material>> {
    center_0: Point3,
    center_1: Point3,
    time_0: f32,
    time_1: f32,
    radius: f32,
    material: M,
}

impl<M: AsRef<dyn Material>> MovingSphere<M> {
    pub fn new(
        center_0: Point3,
        center_1: Point3,
        time_0: f32,
        time_1: f32,
        radius: f32,
        material: M,
    ) -> Self
    where
        M: AsRef<dyn Material>,
    {
        debug_assert!(time_0 <= time_1);

        Self {
            center_0,
            center_1,
            time_0,
            time_1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Point3 {
        debug_assert!((self.time_0..=self.time_1).contains(&time));

        self.center_0
            + ((time - self.time_0) / (self.time_1 - self.time_0) * (self.center_1 - self.center_0))
    }
}

impl<M: AsRef<dyn Material> + Send + Sync> Hittable for MovingSphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center(ray.time());
        let a: f32 = ray.direction().len_squared();
        let half_b: f32 = ray.direction().dot(oc);
        let c: f32 = oc.len_squared() - self.radius.powf(2.);
        let discriminant: f32 = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let dis_sqrt: f32 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        // half_b < 0 && a > 0
        // assert!((-half_b - dis_sqrt) / a <= (-half_b + dis_sqrt) / a);
        let t: f32 = (-half_b - dis_sqrt) / a;
        if !(t_min..=t_max).contains(&t) {
            return None;
        }

        let p: Point3 = ray.at(t);
        let outward_normal: Vec3 = (p - self.center(ray.time())) / self.radius;
        let mut hit_record = HitRecord {
            t,
            p,
            normal: outward_normal,
            front_face: true,
            material: self.material.as_ref(),
        };
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
