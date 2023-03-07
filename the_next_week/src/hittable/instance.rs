use in_one_weekend::{point::Point3, vec3::Vec3};

use crate::{hittable::Hittable, ray::Ray};

use super::HitRecord;

pub enum Instance<H: AsRef<dyn Hittable>> {
    Translate { prototype: H, displacement: Vec3 },
    RotateY { prototype: H, radians: f32 },
}

impl<H: AsRef<dyn Hittable> + Send + Sync> Hittable for Instance<H> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Instance::Translate {
                prototype,
                displacement,
            } => {
                let ray_moved = Ray::new(ray.origin() - displacement, ray.direction(), ray.time());
                prototype
                    .as_ref()
                    .hit(&ray_moved, t_min, t_max)
                    .map(|mut hit_record| {
                        hit_record.p += displacement;
                        hit_record.set_face_normal(&ray_moved, hit_record.normal);
                        hit_record
                    })
            }
            Instance::RotateY { prototype, radians } => {
                let (origin, direction): (Point3, Vec3) = (ray.origin(), ray.direction());
                let (mut origin_new, mut direction_new) = (origin, direction);
                let (cos_theta, sin_theta): (f32, f32) = (radians.cos(), radians.sin());

                *origin_new.x_mut() = cos_theta * origin.x() - sin_theta * origin.z();
                *origin_new.z_mut() = sin_theta * origin.x() + cos_theta * origin.z();

                *direction_new.x_mut() = cos_theta * direction.x() - sin_theta * direction.z();
                *direction_new.z_mut() = sin_theta * direction.x() + cos_theta * direction.z();

                prototype
                    .as_ref()
                    .hit(
                        &Ray::new(origin_new, direction_new, ray.time()),
                        t_min,
                        t_max,
                    )
                    .map(|mut hit_record| {
                        let HitRecord {
                            mut p, mut normal, ..
                        } = hit_record;

                        *p.x_mut() = cos_theta * hit_record.p.x() + sin_theta * hit_record.p.z();
                        *p.z_mut() = -sin_theta * hit_record.p.x() + cos_theta * hit_record.p.z();

                        *normal.x_mut() =
                            cos_theta * hit_record.normal.x() + sin_theta * hit_record.normal.z();
                        *normal.z_mut() =
                            -sin_theta * hit_record.normal.x() + cos_theta * hit_record.normal.z();

                        hit_record.p = p;
                        hit_record.normal = normal;
                        hit_record.set_face_normal(ray, normal);

                        hit_record
                    })
            }
        }
    }
}
