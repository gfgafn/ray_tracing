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

    /// p: a given point on the sphere of radius one, centered at the origin.
    ///
    /// u: returned value [0,1] of angle around the Y axis from X=-1.
    ///
    /// v: returned value [0,1] of angle from Y=-1 to Y=+1.
    ///
    /// <1 0 0> yields <0.50 0.50>       < -1  0  0> yields <0.00 0.50>
    ///
    /// <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    ///
    /// <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    fn uv(p: &Point3) -> [f32; 2] {
        let theta: f32 = (-p.y()).acos();
        let phi: f32 = f32::atan2(-p.z(), p.x()) + core::f32::consts::PI;
        let u = phi / (2.0 * core::f32::consts::PI);
        let v = theta / core::f32::consts::PI;

        [u, v]
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
        let mut t: f32 = (-half_b - dis_sqrt) / a;
        if !(t_min..=t_max).contains(&t) {
            t = (-half_b + dis_sqrt) / a;
            if !(t_min..=t_max).contains(&t) {
                return None;
            }
        }

        let p: Point3 = ray.at(t);
        // `(p - self.center) / self.radius` will make an vector which length is infinite
        // when `self.radius` is near 0
        let outward_normal: Vec3 = (p - self.center).unit_vector();
        let mut hit_record = HitRecord {
            t,
            p,
            normal: outward_normal,
            front_face: true,
            material: self.material.as_ref(),
            uv: Self::uv(&outward_normal),
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

    /// p: a given point on the sphere of radius one, centered at the origin.
    ///
    /// u: returned value [0,1] of angle around the Y axis from X=-1.
    ///
    /// v: returned value [0,1] of angle from Y=-1 to Y=+1.
    ///
    /// <1 0 0> yields <0.50 0.50>       < -1  0  0> yields <0.00 0.50>
    ///
    /// <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    ///
    /// <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    fn uv(p: &Point3) -> [f32; 2] {
        let theta: f32 = (-p.y()).acos();
        let phi: f32 = f32::atan2(-p.z(), p.x()) + core::f32::consts::PI;
        let u = phi / (2.0 * core::f32::consts::PI);
        let v = theta / core::f32::consts::PI;

        [u, v]
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
        let mut t: f32 = (-half_b - dis_sqrt) / a;
        if !(t_min..=t_max).contains(&t) {
            t = (-half_b + dis_sqrt) / a;
            if !(t_min..=t_max).contains(&t) {
                return None;
            }
        }

        let p: Point3 = ray.at(t);
        // `(p - self.center(ray.time())) / self.radius` will make an vector which length is infinite
        // when `self.radius` is near 0
        let outward_normal: Vec3 = (p - self.center(ray.time())).unit_vector();
        let mut hit_record = HitRecord {
            t,
            p,
            normal: outward_normal,
            front_face: true,
            material: self.material.as_ref(),
            uv: Self::uv(&outward_normal),
        };
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
