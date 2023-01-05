use std::sync::Arc;

use crate::ray::Ray;

use super::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest_so_far: f32 = t_max;

        self.objects.iter().for_each(|obj: &Arc<dyn Hittable>| {
            if let Some(hit_record) = obj.hit(ray, t_min, closest_so_far) {
                assert!(t_min <= hit_record.t && hit_record.t <= closest_so_far);
                closest_so_far = hit_record.t;
                hit = Some(hit_record);
            }
        });

        hit
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self { objects: vec![] }
    }
}
