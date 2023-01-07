mod camera;
mod color;
mod hittable;
mod image;
mod point;
mod ray;
mod vec3;

use rand::random;

use std::{sync::Arc, time};

use crate::{
    camera::Camera,
    color::ColorRGB,
    hittable::{hittable_list::HittableList, sphere::Sphere, Hittable},
    image::{PPMImg, PPMImgMagicNum},
    point::Point3,
    ray::Ray,
    vec3::Vec3,
};

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() -> std::io::Result<()> {
    // Image
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u16 = 100;
    const OUTPUT_IMAGE_PATH: &str = "./target/image.ppm";
    let mut image =
        PPMImg::<{ IMAGE_WIDTH as usize }, { IMAGE_HEIGHT as usize }>::new(PPMImgMagicNum::P3);

    // World
    let mut world: HittableList = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera
    let camera = Camera::default();

    let time_render_start: time::Instant = time::Instant::now();

    // Render
    const MAX_DEPTH_RAY_RECURSION: u16 = 50;

    (0..IMAGE_HEIGHT).for_each(|row| {
        print!("\rScanlines remaining: {row}");
        (0..IMAGE_WIDTH).for_each(|column| {
            let pixel_color: ColorRGB = pixel_color::<
                { IMAGE_HEIGHT as usize },
                { IMAGE_WIDTH as usize },
                SAMPLES_PER_PIXEL,
                MAX_DEPTH_RAY_RECURSION,
            >(row, column, &world, &camera);
            image.set_pixel_color(row as usize, column as usize, pixel_color);
        });
    });

    println!(
        "\nThe render took {} seconds",
        time_render_start.elapsed().as_secs_f32()
    );

    image.write_to_file(OUTPUT_IMAGE_PATH)
}

fn ray_color(ray: Ray, world: &dyn Hittable, depth: u16) -> ColorRGB {
    if depth == 0 {
        return ColorRGB::new(0, 0, 0);
    }

    if let Some(hit_record) = world.hit(&ray, 0.001, f32::INFINITY) {
        let target: Point3 =
            hit_record.position() + hit_record.normal() + Point3::random_in_unit_sphere();
        // Half the energy on each bounce were absorbed, 50% were reflected
        return 0.5
            * ray_color(
                Ray::new(hit_record.position(), target - hit_record.position()),
                world,
                depth - 1,
            );
    }

    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * ColorRGB::from_binary(1.0, 1.0, 1.0) + t * ColorRGB::from_binary(0.5, 0.7, 1.0)
}

fn pixel_color<const HEIGHT: usize, const WIDTH: usize, const SAMPLES: u16, const DEPTH: u16>(
    row: u32,
    column: u32,
    world: &dyn Hittable,
    camera: &Camera,
) -> ColorRGB {
    let mut pixel_color = ColorRGB::new(0, 0, 0);
    let (mut red, mut green, mut blue) = (
        pixel_color.r() as f32,
        pixel_color.g() as f32,
        pixel_color.b() as f32,
    );
    (0..SAMPLES).for_each(|_| {
        let u = (column as f32 + random::<f32>()) / (WIDTH - 1) as f32;
        let v = ((HEIGHT - 1 - row as usize) as f32 + random::<f32>()) / (HEIGHT - 1) as f32;
        let ray: Ray = camera.get_ray(u, v);
        let ray_color: ColorRGB = ray_color(ray, world, DEPTH);
        red += ray_color.r() as f32;
        green += ray_color.g() as f32;
        blue += ray_color.b() as f32;
    });
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    [red, green, blue] =
        [red, green, blue].map(|v| (v / SAMPLES as f32).sqrt() * (u8::MAX as f32).sqrt());
    pixel_color = ColorRGB::new(red as u8, green as u8, blue as u8);

    pixel_color
}
