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
        let [u, v, w]: [f32; 3] = [p.x(), p.y(), p.z()].map(|v| v - v.floor());
        let [i, j, k]: [usize; 3] = [p.x(), p.y(), p.z()].map(|v| v.floor() as usize);
        let mut c: [[[f32; 2]; 2]; 2] = [[[0f32; 2]; 2]; 2];
        (0..c.len()).for_each(|di| {
            (0..2).for_each(|dj| {
                (0..2).for_each(|dk| {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]]
                })
            })
        });
        Self::trilinear_interp(&c, u, v, w)
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

    fn trilinear_interp(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        (0..2)
            .flat_map(|i| {
                (0..2).flat_map(move |j| {
                    (0..2).map(move |k| {
                        (i as f32).mul_add(u, (1 - i) as f32 * (1.0 - u))
                            * (j as f32).mul_add(v, (1 - j) as f32 * (1.0 - v))
                            * (k as f32).mul_add(w, (1 - k) as f32 * (1.0 - w))
                            * c[i][j][k]
                    })
                })
            })
            .sum()
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
