use crate::vec3::Vec3;

pub type Point3 = Vec3;

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.2
    }
}

impl std::fmt::Debug for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point-{}", self)
    }
}
