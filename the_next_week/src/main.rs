mod camera;
mod hittable;
mod material;
mod noise;
mod ray;
mod textures;

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
use rand::{random, Rng};

use std::{
    sync::{Arc, Mutex},
    time,
};

use crate::{
    camera::Camera,
    hittable::{
        ConstantMedium, Cuboid, Hittable, HittableList, Instance, MovingSphere, Sphere, XZRect,
    },
    material::{Attenuation, Dielectric, DiffuseLight, Lambertian, Material, Metal},
    noise::Perlin,
    ray::Ray,
    textures::{ImageTexture, NoiseTexture, SolidColor},
};

const ASPECT_RATIO: f32 = 1.0;

fn main() -> std::io::Result<()> {
    let num_cpus = num_cpus::get();
    println!("num_cpus::get() : {num_cpus}");

    let thread_pool: ThreadPool = ThreadPool::new(num_cpus);

    // Image
    const IMAGE_WIDTH: usize = 800;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 10240;
    const OUTPUT_IMAGE_PATH: &str = "./target/image.ppm";
    let image: Arc<Mutex<_>> = Arc::new(Mutex::new(PPMImg::<IMAGE_WIDTH, IMAGE_HEIGHT>::new(
        PPMImgMagicNum::P3,
    )));

    // World
    let world: Arc<HittableList<Box<dyn Hittable>>> = Arc::new(self::final_scene());

    // Camera
    let look_from: Point3 = Point3::new(478.0, 278.0, -600.0);
    let look_at: Point3 = Point3::new(278.0, 278.0, 0.0);
    let disk_to_focus: f32 = 10.0;
    const APERTURE: f32 = 0.1;

    let camera: Arc<Camera> = Arc::new(
        Camera::builder()
            .look_from(look_from)
            .look_at(look_at)
            .up(Vec3::new(0.0, 1.0, 0.0))
            .fov(40.0)
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

    {
        let num_pixels_has_rendered = Arc::clone(&num_pixels_has_rendered);
        thread_pool.execute(move || {
            while let Ok(num_mutex_guard) = num_pixels_has_rendered.lock() {
                if *num_mutex_guard == IMAGE_HEIGHT * IMAGE_WIDTH {
                    break;
                }
            }

            eprintln!(
                "The render took {}",
                format_duration(time_render_start.elapsed())
            );
        });
    }

    thread_pool.execute(move || {
        while let Ok(num_mutex_guard) = num_pixels_has_rendered.lock() {
            if *num_mutex_guard == IMAGE_HEIGHT * IMAGE_WIDTH {
                break;
            }
        }

        image
            .lock()
            .map(|img_mutex_guard| {
                eprintln!("Writing image to file···");
                img_mutex_guard.write_to_file(OUTPUT_IMAGE_PATH)
            })
            .unwrap_or_else(|p_err| p_err.into_inner().write_to_file(OUTPUT_IMAGE_PATH))
            .map(|_| eprintln!("Writing image to file done!"))
            .unwrap_or_else(|err| eprintln!("Writing image to file failed! {}", err))
    });

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

fn final_scene() -> HittableList<Box<dyn Hittable>> {
    let mut objects: HittableList<Box<dyn Hittable>> = HittableList::default();

    const SIZE: usize = 20;
    let mut boxes1: HittableList<Box<dyn Hittable>> = HittableList::with_capacity(SIZE * SIZE);
    let ground: Arc<Lambertian<SolidColor>> = Arc::new(Lambertian::new(SolidColor::from(
        ColorRGBMapTo0_1::new(0.48, 0.83, 0.53),
    )));
    let mut rng = rand::thread_rng();
    (0..SIZE).for_each(|i| {
        (0..SIZE).for_each(|j| {
            let w = 100.0;
            let [x0, y0, z0]: [f32; 3] = [-1000.0 + i as f32 * w, 0.0, -1000.0 + j as f32 * w];
            let [x1, y1, z1]: [f32; 3] = [x0 + w, rng.gen_range(1.0..101.0), z0 + w];

            boxes1.add(Box::new(Cuboid::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground) as Arc<dyn Material>,
            )))
        })
    });
    objects.add(Box::new(boxes1));

    let light: Arc<DiffuseLight<SolidColor>> = Arc::new(DiffuseLight::new(
        SolidColor::from(ColorRGBMapTo0_1::new(1.0, 1.0, 1.0)),
        15.0,
    ));
    objects.add(Box::new(XZRect::new(
        123.0..=423.0,
        147.0..=412.0,
        554.0,
        light as Arc<dyn Material>,
    )));

    let center_1: Point3 = Point3::new(400.0, 400.0, 200.0);
    let center_2: Point3 = Point3::new(400.0, 400.0, 200.0) + Point3::new(30.0, 0.0, 0.0);
    objects.add(Box::new(MovingSphere::new(
        center_1,
        center_2,
        0.0,
        1.0,
        50.0,
        Arc::new(Lambertian::new(SolidColor::from(ColorRGBMapTo0_1::new(
            0.7, 0.3, 0.1,
        )))) as Arc<dyn Material>,
    )));

    let dielectric: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));
    objects.add(Box::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::clone(&dielectric) as Arc<dyn Material>,
    )));

    let metal: Arc<Metal> = Arc::new(Metal::new(Attenuation::new(Vec3::new(0.8, 0.8, 0.9)), 1.0));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        metal as Arc<dyn Material>,
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::clone(&dielectric) as Arc<dyn Material>,
    ));
    objects.add(Box::new(Arc::clone(&boundary)));
    objects.add(Box::new(ConstantMedium::new(
        boundary,
        SolidColor::from(ColorRGBMapTo0_1::new(0.2, 0.4, 0.9)),
        0.2,
    )));
    objects.add(Box::new(ConstantMedium::new(
        Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            5000.0,
            Arc::clone(&dielectric) as Arc<dyn Material>,
        ),
        SolidColor::from(ColorRGBMapTo0_1::new(1.0, 1.0, 1.0)),
        0.0001,
    )));

    let emat: Arc<Lambertian<ImageTexture>> = Arc::new(Lambertian::new(
        ImageTexture::new("the_next_week/assets/images/earthmap.jpg")
            .expect("can not find image under the path 'the_next_week/assets/images/earthmap.jpg'"),
    ));
    objects.add(Box::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat as Arc<dyn Material>,
    )));

    let pertext: NoiseTexture<Perlin> = NoiseTexture::new(Perlin::default()).set_scale(0.1);
    objects.add(Box::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext)) as Arc<dyn Material>,
    )));

    const CAPACITY: usize = 1000;
    let mut boxes2: HittableList<Box<dyn Hittable>> = HittableList::with_capacity(CAPACITY);
    let white: Arc<Lambertian<SolidColor>> = Arc::new(Lambertian::new(SolidColor::from(
        ColorRGBMapTo0_1::new(0.73, 0.73, 0.73),
    )));
    (0..CAPACITY).for_each(|_| {
        boxes2.add(Box::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            Arc::clone(&white) as Arc<dyn Material>,
        )))
    });

    objects.add(Box::new(Instance::Translate {
        prototype: Box::new(Instance::RotateY {
            prototype: Box::new(boxes2) as Box<dyn Hittable>,
            radians: 15_f32.to_radians(),
        }) as Box<dyn Hittable>,
        displacement: Vec3::new(-100.0, 270.0, 395.0),
    }));

    objects
}
