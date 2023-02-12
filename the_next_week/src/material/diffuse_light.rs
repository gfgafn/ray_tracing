use in_one_weekend::point::Point3;

use crate::{hittable::HitRecord, ray::Ray, textures::Texture};

use super::{Emit, EmitRecord, Scatter, ScatterRecord};

pub struct DiffuseLight<T: Texture> {
    emit: T,
    luminance: f32,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(texture: T, luminance: f32) -> Self {
        Self {
            emit: texture,
            luminance,
        }
    }
}

impl<T: Texture> Emit for DiffuseLight<T> {
    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Option<EmitRecord> {
        Some(EmitRecord {
            color: self.emit.value(u, v, p),
            luminance: self.luminance,
        })
    }
}

impl<T: Texture> Scatter for DiffuseLight<T> {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }
}
