use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

use crate::color::ColorRGB;

#[derive(Clone, Copy)]
pub enum PPMImgMagicNum {
    P3,
    P6,
}

impl From<PPMImgMagicNum> for String {
    fn from(value: PPMImgMagicNum) -> Self {
        match value {
            PPMImgMagicNum::P3 => "P3".to_string(),
            PPMImgMagicNum::P6 => "P6".to_string(),
        }
    }
}

pub struct PPMImg<const WIDTH: usize, const HEIGHT: usize> {
    magic_number: PPMImgMagicNum,
    max_color_component: u8,
    data_buffer: [[ColorRGB; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> PPMImg<WIDTH, HEIGHT> {
    pub fn new(magic_number: PPMImgMagicNum) -> Self {
        Self {
            magic_number,
            max_color_component: u8::default(),
            data_buffer: [[ColorRGB::default(); WIDTH]; HEIGHT],
        }
    }

    pub fn set_pixel_color(&mut self, row: usize, column: usize, color: ColorRGB) {
        self.max_color_component = [color.r(), color.g(), color.b(), self.max_color_component]
            .into_iter()
            .max()
            .unwrap_or_default();
        self.max_color_component = if 0 == self.max_color_component {
            0
        } else if u8::MAX == self.max_color_component {
            u8::MAX
        } else {
            self.max_color_component + 1
        };

        self.data_buffer[row][column] = color;
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut image: BufWriter<File> = BufWriter::new(File::create(path)?);
        image.write_all(
            format!(
                "{}\n{WIDTH} {HEIGHT}\n{}\n",
                String::from(self.magic_number),
                self.max_color_component
            )
            .as_bytes(),
        )?;
        for row in &self.data_buffer {
            image.write_all(
                row.map(|c: ColorRGB| format!("{} {} {}\n", c.r(), c.g(), c.b()))
                    .concat()
                    .as_bytes(),
            )?;
        }

        image.flush()
    }
}
