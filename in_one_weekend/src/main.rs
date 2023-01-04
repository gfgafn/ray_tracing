use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut image = File::create("./target/image.ppm")?;
    image.write(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())?;
    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let r = 256 * i / (IMAGE_WIDTH - 1);
            let g = 256 * j / (IMAGE_HEIGHT - 1);
            let b = (256.0 * 0.25) as u32;
            image.write(format!("{r} {g} {b}\n").as_bytes())?;
        }
    }

    Ok(())
}
