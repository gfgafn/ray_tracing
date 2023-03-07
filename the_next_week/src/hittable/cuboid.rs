use in_one_weekend::point::Point3;

use crate::{material::Material, ray::Ray};

use super::{Hittable, HittableList, XYRect, XZRect, YZRect, HitRecord};

pub struct Cuboid<M: AsRef<dyn Material>> {
    min: Point3,
    max: Point3,
    material: M,
    sides: HittableList<Box<dyn Hittable>>,
}

impl<M: AsRef<dyn Material> + Clone + Send + Sync + 'static> Cuboid<M> {
    pub fn new(min: Point3, max: Point3, material: M) -> Self {
        let mut sides: HittableList<Box<dyn Hittable>> = HittableList::default();
        sides.add(Box::new(XYRect::new(
            min.x()..=max.x(),
            min.y()..=max.y(),
            max.z(),
            material.clone(),
        )));
        sides.add(Box::new(XYRect::new(
            min.x()..=max.x(),
            min.y()..=max.y(),
            min.z(),
            material.clone(),
        )));

        sides.add(Box::new(XZRect::new(
            min.x()..=max.x(),
            min.z()..=max.z(),
            max.y(),
            material.clone(),
        )));
        sides.add(Box::new(XZRect::new(
            min.x()..=max.x(),
            min.z()..=max.z(),
            min.y(),
            material.clone(),
        )));

        sides.add(Box::new(YZRect::new(
            min.y()..=max.y(),
            min.z()..=max.z(),
            max.x(),
            material.clone(),
        )));
        sides.add(Box::new(YZRect::new(
            min.y()..=max.y(),
            min.z()..=max.z(),
            min.x(),
            material.clone(),
        )));

        Self {
            min,
            max,
            material,
            sides,
        }
    }
}

impl<M: AsRef<dyn Material> + Send + Sync> Hittable for Cuboid<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
}
