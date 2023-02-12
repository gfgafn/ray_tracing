use in_one_weekend::{point::Point3, vec3::Vec3};

use std::ops::RangeInclusive;

use crate::{material::Material, ray::Ray};

use super::{HitRecord, Hittable};

pub struct XYRect<M: AsRef<dyn Material>> {
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
    k: f32,
    material: M,
}

impl<M: AsRef<dyn Material>> XYRect<M> {
    pub fn new(
        x_range: RangeInclusive<f32>,
        y_range: RangeInclusive<f32>,
        k: f32,
        material: M,
    ) -> Self {
        Self {
            x_range,
            y_range,
            k,
            material,
        }
    }
}

impl<M: AsRef<dyn Material> + Send + Sync> Hittable for XYRect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z()) / ray.direction().z();
        if !(t_min..=t_max).contains(&t) {
            return None;
        }

        let ray_at_t: Point3 = ray.at(t);
        if !self.x_range.contains(&ray_at_t.x()) || !self.y_range.contains(&ray_at_t.y()) {
            return None;
        }

        let uv: [f32; 2] = [
            (ray_at_t.x() - self.x_range.start()) / (self.x_range.end() - self.x_range.start()),
            (ray_at_t.y() - self.y_range.start()) / (self.y_range.end() - self.y_range.start()),
        ];

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let mut hit_record = HitRecord {
            p: ray_at_t,
            normal: outward_normal,
            t,
            front_face: true,
            material: self.material.as_ref(),
            uv,
        };
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}

pub struct XZRect<M: AsRef<dyn Material>> {
    x_range: RangeInclusive<f32>,
    z_range: RangeInclusive<f32>,
    k: f32,
    material: M,
}

impl<M: AsRef<dyn Material>> XZRect<M> {
    pub fn new(
        x_range: RangeInclusive<f32>,
        z_range: RangeInclusive<f32>,
        k: f32,
        material: M,
    ) -> Self {
        Self {
            x_range,
            z_range,
            k,
            material,
        }
    }
}

impl<M: AsRef<dyn Material> + Send + Sync> Hittable for XZRect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().y()) / ray.direction().y();
        if !(t_min..=t_max).contains(&t) {
            return None;
        }

        let ray_at_t: Point3 = ray.at(t);
        if !self.x_range.contains(&ray_at_t.x()) || !self.z_range.contains(&ray_at_t.z()) {
            return None;
        }

        let uv: [f32; 2] = [
            (ray_at_t.x() - self.x_range.start()) / (self.x_range.end() - self.x_range.start()),
            (ray_at_t.z() - self.z_range.start()) / (self.z_range.end() - self.z_range.start()),
        ];

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let mut hit_record = HitRecord {
            p: ray_at_t,
            normal: outward_normal,
            t,
            front_face: true,
            material: self.material.as_ref(),
            uv,
        };
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}

pub struct YZRect<M: AsRef<dyn Material>> {
    y_range: RangeInclusive<f32>,
    z_range: RangeInclusive<f32>,
    k: f32,
    material: M,
}

impl<M: AsRef<dyn Material>> YZRect<M> {
    pub fn new(
        y_range: RangeInclusive<f32>,
        z_range: RangeInclusive<f32>,
        k: f32,
        material: M,
    ) -> Self {
        Self {
            y_range,
            z_range,
            k,
            material,
        }
    }
}

impl<M: AsRef<dyn Material> + Send + Sync> Hittable for YZRect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().x()) / ray.direction().x();
        if !(t_min..=t_max).contains(&t) {
            return None;
        }

        let ray_at_t: Point3 = ray.at(t);
        if !self.y_range.contains(&ray_at_t.y()) || !self.z_range.contains(&ray_at_t.z()) {
            return None;
        }

        let uv: [f32; 2] = [
            (ray_at_t.x() - self.y_range.start()) / (self.y_range.end() - self.y_range.start()),
            (ray_at_t.z() - self.z_range.start()) / (self.z_range.end() - self.z_range.start()),
        ];

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let mut hit_record = HitRecord {
            p: ray_at_t,
            normal: outward_normal,
            t,
            front_face: true,
            material: self.material.as_ref(),
            uv,
        };
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
