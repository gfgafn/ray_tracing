use std::ops;

#[derive(Debug, PartialEq, Eq)]
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

    /// 将RGB中各分量值的范围从 `0-1` 映射到 `0-255`
    pub fn from_binary(red: f32, green: f32, blue: f32) -> Self {
        assert!(0.0 <= red && red <= 1.0);
        assert!(0.0 <= green && green <= 1.0);
        assert!(0.0 <= blue && blue <= 1.0);
        Self(
            (red * 255 as f32) as u8,
            (green * 255 as f32) as u8,
            (blue * 255 as f32) as u8,
        )
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            ColorRGB::new(255, 255, 255),
            ColorRGB::from_binary(1.0, 1.0, 1.0)
        );
        assert_eq!(ColorRGB::new(0, 0, 0), ColorRGB::from_binary(0.0, 0.0, 0.0));
        assert_eq!(
            ColorRGB::new(127, 178, 255),
            ColorRGB::from_binary(0.5, 0.7, 1.0)
        )
    }
}
