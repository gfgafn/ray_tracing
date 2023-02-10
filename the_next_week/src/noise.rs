use in_one_weekend::point::Point3;
use rand::{random, Rng};

pub struct Perlin {
    ranfloat: Box<[f32]>,
    perm_x: Box<[usize]>,
    perm_y: Box<[usize]>,
    perm_z: Box<[usize]>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn noise(&self, p: &Point3) -> f32 {
        let [i, j, k]: [usize; 3] = [p.x(), p.y(), p.z()].map(|v| (4.0 * v) as usize & 255);

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm() -> Box<[usize; Self::POINT_COUNT]> {
        let mut p = Box::new([0; Self::POINT_COUNT]);
        p.iter_mut()
            .enumerate()
            .for_each(|(index, value)| *value = index);

        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut [usize; Self::POINT_COUNT]) {
        (1..p.len()).rev().for_each(|index| {
            let target = rand::thread_rng().gen_range(0..index);
            p.swap(index, target);
        })
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranfloat = Box::new([0f32; Self::POINT_COUNT]);
        ranfloat.iter_mut().for_each(|v: &mut f32| *v = random());

        Self {
            ranfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
}
