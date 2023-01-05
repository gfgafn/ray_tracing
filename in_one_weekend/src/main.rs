mod color;
mod point;
mod vec3;

use std::{fs::File, io::Write};

use color::ColorRGB;

fn main() -> std::io::Result<()> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    const OUTPUT_IMAGE_PATH: &'static str = "./target/image.ppm";
    let mut image = File::create(OUTPUT_IMAGE_PATH)?;
    image.write(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())?;
    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let pixel_color = ColorRGB::new(
                (255.0 * i as f32 / (IMAGE_WIDTH - 1) as f32) as u8,
                (255.0 * j as f32 / (IMAGE_HEIGHT - 1) as f32) as u8,
                (0.25 * 255.0) as u8,
            );
            write_color(&mut image, pixel_color)?;
        }
    }

    Ok(())
}

/// write a single pixel's color out to the file
fn write_color(file: &mut std::fs::File, color: ColorRGB) -> std::io::Result<()> {
    file.write(format!("{} {} {}\n", color.r(), color.g(), color.b()).as_bytes())?;
    Ok(())
}
