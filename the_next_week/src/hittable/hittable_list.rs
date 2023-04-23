use crate::ray::Ray;

use super::{HitRecord, Hittable};

pub struct HittableList<H: AsRef<dyn Hittable>> {
    objects: Vec<H>,
}

impl<H: AsRef<dyn Hittable>> HittableList<H> {
    pub fn add(&mut self, object: H)
    where
        H: AsRef<dyn Hittable>,
    {
        self.objects.push(object)
    }

    // pub fn clear(&mut self) {
    //     self.objects.clear()
    // }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            objects: Vec::with_capacity(capacity),
        }
    }
}

impl<H: AsRef<dyn Hittable>> Default for HittableList<H> {
    fn default() -> Self {
        Self { objects: vec![] }
    }
}

impl<H: AsRef<dyn Hittable> + Send + Sync> Hittable for HittableList<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| object.as_ref().hit(ray, t_min, t_max))
            .reduce(|closest_hit_record, current_hit_record| {
                if current_hit_record.t < closest_hit_record.t {
                    current_hit_record
                } else {
                    closest_hit_record
                }
            })
    }
}
