use std::{mem, ops};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct ColorRGB(u8, u8, u8);

impl ColorRGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(red, green, blue)
    }

    #[inline]
    pub fn r(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.1
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.2
    }

    pub fn as_bytes(&self) -> &[u8; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl ops::Add for ColorRGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Mul<f32> for ColorRGB {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(
            (self.0 as f32 * rhs) as u8,
            (self.1 as f32 * rhs) as u8,
            (self.2 as f32 * rhs) as u8,
        )
    }
}

impl ops::Mul<ColorRGB> for f32 {
    type Output = ColorRGB;

    fn mul(self, rhs: ColorRGB) -> Self::Output {
        ColorRGB(
            (self * rhs.0 as f32) as u8,
            (self * rhs.1 as f32) as u8,
            (self * rhs.2 as f32) as u8,
        )
    }
}

impl From<ColorRGBMapTo0_1> for ColorRGB {
    fn from(color: ColorRGBMapTo0_1) -> Self {
        Self(
            (color.0 * u8::MAX as f32) as u8,
            (color.1 * u8::MAX as f32) as u8,
            (color.2 * u8::MAX as f32) as u8,
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ColorRGBMapTo<const MIN: usize, const MAX: usize, T: Copy>(T, T, T);

pub type ColorRGBMapTo0_1 = ColorRGBMapTo<0, 1, f32>;

impl<const MIN: usize, const MAX: usize, T: Copy> ColorRGBMapTo<MIN, MAX, T> {
    pub fn new(red: T, green: T, blue: T) -> Self {
        Self(red, green, blue)
    }

    #[inline]
    pub fn r(&self) -> T {
        self.0
    }

    #[inline]
    pub fn g(&self) -> T {
        self.1
    }

    #[inline]
    pub fn b(&self) -> T {
        self.2
    }
}
impl ops::Add for ColorRGBMapTo0_1 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Mul<f32> for ColorRGBMapTo0_1 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<ColorRGBMapTo0_1> for f32 {
    type Output = ColorRGBMapTo0_1;

    fn mul(self, rhs: ColorRGBMapTo0_1) -> Self::Output {
        ColorRGBMapTo0_1::new(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl From<ColorRGB> for ColorRGBMapTo0_1 {
    fn from(color: ColorRGB) -> Self {
        Self(
            color.r() as f32 / u8::MAX as f32,
            color.g() as f32 / u8::MAX as f32,
            color.b() as f32 / u8::MAX as f32,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn color_map_convert_should_work() {
        let color_a: ColorRGBMapTo0_1 = ColorRGBMapTo0_1::new(0.0, 1.0, 0.5);
        let color_b: ColorRGB = ColorRGB::new(0, 255, 127);

        let color_b_2: ColorRGBMapTo0_1 = ColorRGBMapTo0_1::from(color_b);
        assert_eq!(color_a.r(), color_b_2.r());
        assert_eq!(color_a.g(), color_b_2.g());
        assert!((0.49..=0.51).contains(&color_b_2.b()));

        let color_a_2: ColorRGB = ColorRGB::from(color_a);
        assert_eq!(color_b.r(), color_a_2.r());
        assert_eq!(color_b.g(), color_a_2.g());
        assert!((127..=128).contains(&color_a_2.b()))
    }
}
