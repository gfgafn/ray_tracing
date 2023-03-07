use in_one_weekend::{point::Point3, vec3::Vec3};
use rand::Rng;

pub struct Perlin {
    ranvec: Box<[Vec3]>,
    perm_x: Box<[usize]>,
    perm_y: Box<[usize]>,
    perm_z: Box<[usize]>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn noise(&self, p: &Point3) -> f32 {
        let [u, v, w]: [f32; 3] = [p.x(), p.y(), p.z()].map(|v| v - v.floor());
        let [i, j, k]: [usize; 3] = [p.x(), p.y(), p.z()].map(|v| v.floor() as usize);
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        (0..c.len()).for_each(|di| {
            (0..2).for_each(|dj| {
                (0..2).for_each(|dk| {
                    c[di][dj][dk] = self.ranvec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]]
                })
            })
        });
        Self::trilinear_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: usize) -> f32 {
        (0..depth)
            .fold(
                (0.0, *p, 1.0),
                |(accum, p, weight): (f32, Point3, f32), _| {
                    (accum + weight * self.noise(&p), (p * 2.0), weight * 0.5)
                },
            )
            .0
            .abs()
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

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let [u, v, w]: [f32; 3] = [u, v, w].map(|v| v * v * (3.0 - 2.0 * v));

        (0..2)
            .flat_map(|i| {
                (0..2).flat_map(move |j| {
                    (0..2).map(move |k| {
                        (i as f32).mul_add(u, (1 - i) as f32 * (1.0 - u))
                            * (j as f32).mul_add(v, (1 - j) as f32 * (1.0 - v))
                            * (k as f32).mul_add(w, (1 - k) as f32 * (1.0 - w))
                            * c[i][j][k].dot(Vec3::new(u - i as f32, v - i as f32, w - k as f32))
                    })
                })
            })
            .sum()
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranvec = Box::new([Vec3::default(); Self::POINT_COUNT]);
        ranvec
            .iter_mut()
            .for_each(|vec| *vec = Vec3::random_range(-1.0, 1.0).unit_vector());
        Self {
            ranvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
}
