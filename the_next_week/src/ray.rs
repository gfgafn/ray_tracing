use in_one_weekend::{point::Point3, vec3::Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    #[inline]
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    #[inline]
    pub fn time(&self) -> f32 {
        self.time
    }

    /// 光线的终点所在位置坐标
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
