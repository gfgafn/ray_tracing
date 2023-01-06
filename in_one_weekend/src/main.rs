mod camera;
mod color;
mod hittable;
mod point;
mod ray;
mod vec3;

use rand;

use std::{fs::File, io::Write, sync::Arc};

use crate::{
    camera::Camera,
    color::ColorRGB,
    hittable::{hittable_list::HittableList, sphere::Sphere, Hittable},
    point::Point3,
    ray::Ray,
    vec3::Vec3,
};

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() -> std::io::Result<()> {
    // Image
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // World
    let mut world: HittableList = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera
    let camera = Camera::default();

    // Render
    const OUTPUT_IMAGE_PATH: &'static str = "./target/image.ppm";
    let mut image = File::create(OUTPUT_IMAGE_PATH)?;
    image.write(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())?;
    for j in 0..IMAGE_HEIGHT {
        print!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let pixel_color = pixel_color(
                j,
                i,
                IMAGE_HEIGHT,
                IMAGE_WIDTH,
                &world,
                &camera,
                SAMPLES_PER_PIXEL,
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

fn pixel_color(
    row: u32,
    column: u32,
    img_height: u32,
    img_width: u32,
    world: &HittableList,
    camera: &Camera,
    samples_per_pixel: u32,
) -> ColorRGB {
    let mut pixel_color = ColorRGB::new(0, 0, 0);
    let (mut red, mut green, mut blue) = (
        pixel_color.r() as f32,
        pixel_color.g() as f32,
        pixel_color.b() as f32,
    );
    (0..samples_per_pixel).for_each(|_| {
        let u = (column as f32 + rand::random::<f32>()) / (img_width - 1) as f32;
        let v = ((img_height - 1 - row) as f32 + rand::random::<f32>()) / (img_height - 1) as f32;
        let ray: Ray = camera.get_ray(u, v);
        let ray_color: ColorRGB = ray_color(ray, world);
        red += ray_color.r() as f32;
        green += ray_color.g() as f32;
        blue += ray_color.b() as f32;
    });
    [red, green, blue] = [red, green, blue].map(|v| v / samples_per_pixel as f32);
    pixel_color = ColorRGB::new(red as u8, green as u8, blue as u8);

    pixel_color
}
