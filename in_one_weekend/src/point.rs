use rand::{random, rngs::ThreadRng, thread_rng, Rng};

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

    #[inline]
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.1
    }

    #[inline]
    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.2
    }

    pub fn random() -> Self {
        Self(random(), random(), random())
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        let mut rng: ThreadRng = thread_rng();
        Self(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p: Self = Self::random_range(-1.0, 1.0);
            if 1. >= p.len_squared() {
                break p;
            }
        }
    }
}

impl std::fmt::Debug for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point-{self}")
    }
}
