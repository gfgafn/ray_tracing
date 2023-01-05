mod color;
mod point;
mod ray;
mod vec3;

use std::{fs::File, io::Write};

use crate::{color::ColorRGB, point::Point3, ray::Ray, vec3::Vec3};

fn main() -> std::io::Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    // Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    const ORIGIN: Point3 = Vec3(0.0, 0.0, 0.0);
    const HORIZONTAL: Vec3 = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    const VERTICAL: Vec3 = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Point3 =
        ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3(0.0, 0.0, FOCAL_LENGTH);

    // Render
    const OUTPUT_IMAGE_PATH: &'static str = "./target/image.ppm";
    let mut image = File::create(OUTPUT_IMAGE_PATH)?;
    image.write(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())?;
    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let ray = Ray::new(
                ORIGIN,
                lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            );
            let pixel_color: ColorRGB = ray_color(ray);
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

fn ray_color(ray: Ray) -> ColorRGB {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return ColorRGB::from_binary(1.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * ColorRGB::from_binary(1.0, 1.0, 1.0) + t * ColorRGB::from_binary(0.5, 0.7, 1.0)
}

fn hit_sphere(sphere_center: &Point3, radius: f32, ray: &Ray) -> bool {
    let oc: Vec3 = ray.origin() - sphere_center;
    let a: f32 = ray.direction().dot(ray.direction());
    let b: f32 = 2.0 * ray.direction().dot(oc);
    let c: f32 = oc.dot(oc) - radius * radius;
    let discriminant: f32 = b * b - 4.0 * a * c;
    discriminant >= 0.0
}
