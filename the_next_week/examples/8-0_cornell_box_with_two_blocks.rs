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
    hittable::{Cuboid, Hittable, HittableList, XYRect, XZRect, YZRect},
    material::{DiffuseLight, Lambertian, Material},
    ray::Ray,
    textures::SolidColor,
};

const ASPECT_RATIO: f32 = 1.0;

fn main() -> std::io::Result<()> {
    let num_cpus = num_cpus::get();
    println!("num_cpus::get() : {num_cpus}");

    let thread_pool: ThreadPool = ThreadPool::new(num_cpus);

    // Image
    const IMAGE_WIDTH: usize = 600;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 200;
    const OUTPUT_IMAGE_PATH: &str = "./target/image.ppm";
    let image: Arc<Mutex<_>> = Arc::new(Mutex::new(PPMImg::<IMAGE_WIDTH, IMAGE_HEIGHT>::new(
        PPMImgMagicNum::P3,
    )));

    // World
    let world: Arc<HittableList<Box<dyn Hittable>>> = Arc::new(self::cornell_box());

    // Camera
    let look_from: Point3 = Point3::new(278.0, 278.0, -800.0);
    let look_at: Point3 = Point3::new(278.0, 278.0, 0.0);
    let disk_to_focus: f32 = 10.0;
    const APERTURE: f32 = 0.1;

    let camera: Arc<Camera> = Arc::new(Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        ASPECT_RATIO,
        APERTURE,
        disk_to_focus,
        0.0,
        1.0,
    ));

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

    let background_color = ColorRGBMapTo0_1::new(0.0, 0.0, 0.0);

    world
        .hit(ray, 0.001, f32::INFINITY)
        .map_or(background_color, |hit_record| {
            match (
                hit_record.material().emitted(
                    hit_record.u(),
                    hit_record.v(),
                    &hit_record.position(),
                ),
                hit_record.material().scatter(ray, &hit_record),
            ) {
                (None, None) => background_color,
                (Some(emit_record), None) => emit_record.color() * emit_record.luminance(),
                (None, Some(scatter_rec)) => {
                    scatter_rec.albedo() * ray_color(scatter_rec.ray_scattered(), world, depth - 1)
                }
                (Some(emit_record), Some(scatter_rec)) => {
                    emit_record.color() * emit_record.luminance()
                        + scatter_rec.albedo()
                            * ray_color(scatter_rec.ray_scattered(), world, depth - 1)
                }
            }
        })
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

fn cornell_box() -> HittableList<Box<dyn Hittable>> {
    let mut objects: HittableList<Box<dyn Hittable>> = HittableList::default();

    let red: Arc<Lambertian<SolidColor>> = Arc::new(Lambertian::new(SolidColor::from(
        ColorRGBMapTo0_1::new(0.65, 0.05, 0.05),
    )));
    let white: Arc<Lambertian<SolidColor>> = Arc::new(Lambertian::new(SolidColor::from(
        ColorRGBMapTo0_1::new(0.73, 0.73, 0.73),
    )));
    let green: Arc<Lambertian<SolidColor>> = Arc::new(Lambertian::new(SolidColor::from(
        ColorRGBMapTo0_1::new(0.12, 0.45, 0.15),
    )));
    let light: Arc<DiffuseLight<SolidColor>> = Arc::new(DiffuseLight::new(
        SolidColor::from(ColorRGBMapTo0_1::new(1.0, 1.0, 1.0)),
        15.0,
    ));

    objects.add(Box::new(YZRect::new(
        0.0..=555.0,
        0.0..=555.0,
        555.0,
        Arc::clone(&green) as Arc<dyn Material>,
    )));
    objects.add(Box::new(YZRect::new(
        0.0..=555.0,
        0.0..=555.0,
        0.0,
        Arc::clone(&red) as Arc<dyn Material>,
    )));
    objects.add(Box::new(XZRect::new(
        213.0..=343.0,
        227.0..=332.0,
        554.0,
        light as Arc<dyn Material>,
    )));
    objects.add(Box::new(XZRect::new(
        0.0..=555.0,
        1.0..=555.0,
        0.0,
        Arc::clone(&white) as Arc<dyn Material>,
    )));
    objects.add(Box::new(XZRect::new(
        0.0..=555.0,
        0.0..=555.0,
        555.0,
        Arc::clone(&white) as Arc<dyn Material>,
    )));
    objects.add(Box::new(XYRect::new(
        0.0..=555.0,
        0.0..=555.0,
        555.0,
        Arc::clone(&white) as Arc<dyn Material>,
    )));
    objects.add(Box::new(XYRect::new(
        0.0..=555.0,
        0.0..=555.0,
        555.0,
        Arc::clone(&white) as Arc<dyn Material>,
    )));

    objects.add(Box::new(Cuboid::new(
        Point3::new(130.0, 0.0, 65.0),
        Point3::new(295.0, 165.0, 230.0),
        Arc::clone(&white) as Arc<dyn Material>,
    )));
    objects.add(Box::new(Cuboid::new(
        Point3::new(265.0, 0.0, 265.0),
        Point3::new(430.0, 330.0, 460.0),
        Arc::clone(&white) as Arc<dyn Material>,
    )));

    objects
}
