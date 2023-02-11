use image::{self, DynamicImage, GenericImageView};
use in_one_weekend::{
    color::{ColorRGB, ColorRGBMapTo0_1},
    point::Point3,
};

use std::path::Path;

use crate::{material::Attenuation, noise::Perlin};

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> ColorRGBMapTo0_1;
}

pub struct SolidColor {
    color: ColorRGBMapTo0_1,
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> ColorRGBMapTo0_1 {
        self.color
    }
}

impl From<Attenuation> for SolidColor {
    fn from(value: Attenuation) -> Self {
        Self {
            color: <Attenuation as Into<ColorRGBMapTo0_1>>::into(value),
        }
    }
}

impl From<ColorRGBMapTo0_1> for SolidColor {
    fn from(value: ColorRGBMapTo0_1) -> Self {
        Self { color: value }
    }
}
pub struct CheckerTexture<T: Texture, U: Texture> {
    even: T,
    odd: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(even: T, odd: U) -> Self {
        Self { even, odd }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f32, v: f32, p: &Point3) -> ColorRGBMapTo0_1 {
        let sines: f32 = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if 0.0 > sines {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture<T> {
    noise: T,
    scale: f32,
}

impl<T> NoiseTexture<T> {
    pub fn new(noise: T) -> Self {
        Self { noise, scale: 1f32 }
    }

    pub fn set_scale(mut self, scale: f32) -> Self {
        self.scale = scale;

        self
    }
}

impl Texture for NoiseTexture<Perlin> {
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> ColorRGBMapTo0_1 {
        ColorRGBMapTo0_1::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    img: DynamicImage,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let img = image::open(path).map_err(|err| err.to_string())?;

        Ok(Self { img })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Point3) -> ColorRGBMapTo0_1 {
        let [width, height]: [f32; 2] = [self.img.width() as f32, self.img.height() as f32];
        let [x, y]: [u32; 2] = [
            ((u.clamp(0.0, 1.0) * width) as u32).clamp(0, width as u32 - 1),
            (((1.0 - v.clamp(0.0, 1.0)) * height) as u32).clamp(0, height as u32 - 1),
        ];
        let pixel_color_rgba: [u8; 4] = self.img.get_pixel(x, y).0;

        ColorRGB::new(
            pixel_color_rgba[0],
            pixel_color_rgba[1],
            pixel_color_rgba[2],
        )
        .into()
    }
}
