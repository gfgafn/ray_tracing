extern crate num_cpus;
use humantime::format_duration;
use in_one_weekend::{
    color::{ColorRGB, ColorRGBMapTo0_1},
    image::{PPMImg, PPMImgMagicNum},
    point::Point3,
    thread_pool::ThreadPool,
    utils,
    vec3::Vec3,
};
use rand::random;

use std::{
    mem,
    sync::{Arc, Mutex},
    time,
};

use the_next_week::{
    camera::Camera,
    hittable::{Hittable, HittableList, Sphere},
    material::{Lambertian, Material},
    noise::Perlin,
    ray::Ray,
    textures::NoiseTexture,
};

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() -> std::io::Result<()> {
    let num_cpus = num_cpus::get();
    println!("num_cpus::get() : {num_cpus}");

    let thread_pool: ThreadPool = ThreadPool::new(num_cpus);

    // Image
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const OUTPUT_IMAGE_PATH: &str = "./target/image.ppm";
    let image: Arc<Mutex<_>> = Arc::new(Mutex::new(PPMImg::<IMAGE_WIDTH, IMAGE_HEIGHT>::new(
        PPMImgMagicNum::P3,
    )));

    // World
    let world: Arc<HittableList<Box<dyn Hittable>>> = Arc::new(self::two_perlin_spheres());

    // Camera
    let look_from: Point3 = Point3::new(13.0, 2.0, 3.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let disk_to_focus: f32 = 10.0;
    const APERTURE: f32 = 0.1;

    let camera: Arc<Camera> = Arc::new(
        Camera::builder()
            .look_from(look_from)
            .look_at(look_at)
            .up(Vec3::new(0.0, 1.0, 0.0))
            .fov(20.0)
            .aspect_ratio(ASPECT_RATIO)
            .aperture(APERTURE)
            .focus_dist(disk_to_focus)
            .time_0(0.0)
            .time_1(1.0)
            .build(),
    );

    let time_render_start: time::Instant = time::Instant::now();

    // Render
    const MAX_DEPTH_RAY_RECURSION: u16 = 50;
    let num_pixels_has_rendered: Arc<Mutex<usize>> = Arc::new(Mutex::new(0usize));

    (0..IMAGE_HEIGHT).for_each(|row| {
        (0..IMAGE_WIDTH).for_each(|column| {
            let image = Arc::clone(&image);
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);
            let num_pixels_has_rendered = Arc::clone(&num_pixels_has_rendered);

            thread_pool.execute(move || {
                let pixel_color: ColorRGB = self::pixel_color::<
                    IMAGE_HEIGHT,
                    IMAGE_WIDTH,
                    SAMPLES_PER_PIXEL,
                    MAX_DEPTH_RAY_RECURSION,
                >(row, column, world.as_ref(), &camera);

                image
                    .lock()
                    .unwrap()
                    .set_pixel_color(row, column, pixel_color);

                if let Ok(mut num) = num_pixels_has_rendered.lock() {
                    *num += 1;
                    if *num % (IMAGE_HEIGHT * IMAGE_WIDTH / 1000) == 0 {
                        utils::log_progress(*num as f64 / (IMAGE_HEIGHT * IMAGE_WIDTH) as f64)
                            .unwrap();
                    }
                };
            })
        });
    });

    mem::drop(thread_pool);

    println!(
        "The render took {}",
        format_duration(time_render_start.elapsed())
    );

    image.lock().unwrap().write_to_file(OUTPUT_IMAGE_PATH)?;

    Ok(())
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u16) -> ColorRGBMapTo0_1 {
    if depth == 0 {
        return ColorRGBMapTo0_1::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f32::INFINITY) {
        return match hit_record.material().scatter(ray, &hit_record) {
            None => ColorRGBMapTo0_1::new(0.0, 0.0, 0.0),
            Some(scatter_rec) => {
                scatter_rec.albedo() * ray_color(scatter_rec.ray_scattered(), world, depth - 1)
            }
        };
    }

    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * ColorRGBMapTo0_1::new(1.0, 1.0, 1.0) + t * ColorRGBMapTo0_1::new(0.5, 0.7, 1.0)
}

fn pixel_color<const HEIGHT: usize, const WIDTH: usize, const SAMPLES: usize, const DEPTH: u16>(
    row: usize,
    column: usize,
    world: &dyn Hittable,
    camera: &Camera,
) -> ColorRGB {
    let [red, green, blue] = (0..SAMPLES)
        .fold([0.0, 0.0, 0.0], |[r, g, b], _| {
            let u = (column as f32 + random::<f32>()) / (WIDTH - 1) as f32;
            let v = ((HEIGHT - 1 - row) as f32 + random::<f32>()) / (HEIGHT - 1) as f32;
            let ray: Ray = camera.get_ray(u, v);
            let ray_color: ColorRGBMapTo0_1 = ray_color(&ray, world, DEPTH);

            [r + ray_color.r(), g + ray_color.g(), b + ray_color.b()]
        })
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        .map(|v: f32| (v / SAMPLES as f32).sqrt());

    ColorRGBMapTo0_1::new(red, green, blue).into()
}

fn two_perlin_spheres() -> HittableList<Box<dyn Hittable>> {
    let mut objects: HittableList<Box<dyn Hittable>> = HittableList::default();
    let pertext: NoiseTexture<Perlin> = NoiseTexture::new(Perlin::default()).set_scale(4.0);
    let material: Arc<Lambertian<NoiseTexture<Perlin>>> = Arc::new(Lambertian::new(pertext));

    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material) as Arc<dyn Material>,
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&material) as Arc<dyn Material>,
    )));

    objects
}
