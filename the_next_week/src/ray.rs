use in_one_weekend::{point::Point3, vec3::Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[inline]
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// 光线的终点所在位置坐标
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
