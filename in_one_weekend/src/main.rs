mod color;
mod hittable;
mod point;
mod ray;
mod vec3;

use std::{fs::File, io::Write, sync::Arc};

use crate::{
    color::ColorRGB,
    hittable::{hittable_list::HittableList, sphere::Sphere, Hittable},
    point::Point3,
    ray::Ray,
    vec3::Vec3,
};

fn main() -> std::io::Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    // World
    let mut world: HittableList = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

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
            let pixel_color: ColorRGB = ray_color(ray, &world);
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

fn ray_color(ray: Ray, world: &dyn Hittable) -> ColorRGB {
    if let Some(hit_record) = world.hit(&ray, 0., f32::INFINITY) {
        let normal: Vec3 = hit_record.normal;
        return ColorRGB::from_binary(
            0.5 * normal.x() + 0.5,
            0.5 * normal.y() + 0.5,
            0.5 * normal.z() + 0.5,
        );
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * ColorRGB::from_binary(1.0, 1.0, 1.0) + t * ColorRGB::from_binary(0.5, 0.7, 1.0)
}
