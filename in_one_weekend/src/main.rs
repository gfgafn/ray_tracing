mod camera;
mod color;
mod hittable;
mod image;
mod material;
mod point;
mod ray;
mod vec3;

use rand::{random, rngs::ThreadRng, Rng};

use std::{f32, sync::Arc, time};

use crate::{
    camera::Camera,
    color::{ColorRGB, ColorRGBMapTo0_1},
    hittable::{hittable_list::HittableList, sphere::Sphere, Hittable},
    image::{PPMImg, PPMImgMagicNum},
    material::{Attenuation, Dielectric, Lambertian, Material, Metal},
    point::Point3,
    ray::Ray,
    vec3::Vec3,
};

const ASPECT_RATIO: f32 = 3.0 / 2.0;

fn main() -> std::io::Result<()> {
    // Image
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u16 = 500;
    const OUTPUT_IMAGE_PATH: &str = "./target/image.ppm";
    let mut image = Box::new(
        PPMImg::<{ IMAGE_WIDTH as usize }, { IMAGE_HEIGHT as usize }>::new(PPMImgMagicNum::P3),
    );

    // World
    let world: HittableList<Arc<dyn Hittable>> = self::random_scene();

    // Camera
    let look_from: Point3 = Point3::new(13.0, 2.0, 3.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let disk_to_focus: f32 = 10.0;
    const APERTURE: f32 = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        APERTURE,
        disk_to_focus,
    );

    let time_render_start: time::Instant = time::Instant::now();

    // Render
    const MAX_DEPTH_RAY_RECURSION: u16 = 50;

    (0..IMAGE_HEIGHT).for_each(|row| {
        print!("\rScanlines remaining: {row}");
        (0..IMAGE_WIDTH).for_each(|column| {
            let pixel_color: ColorRGB = self::pixel_color::<
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

fn pixel_color<const HEIGHT: usize, const WIDTH: usize, const SAMPLES: u16, const DEPTH: u16>(
    row: u32,
    column: u32,
    world: &dyn Hittable,
    camera: &Camera,
) -> ColorRGB {
    let [red, green, blue] = (0..SAMPLES)
        .fold([0.0, 0.0, 0.0], |[r, g, b], _| {
            let u = (column as f32 + random::<f32>()) / (WIDTH - 1) as f32;
            let v = ((HEIGHT - 1 - row as usize) as f32 + random::<f32>()) / (HEIGHT - 1) as f32;
            let ray: Ray = camera.get_ray(u, v);
            let ray_color: ColorRGBMapTo0_1 = ray_color(&ray, world, DEPTH);

            [r + ray_color.r(), g + ray_color.g(), b + ray_color.b()]
        })
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        .map(|v: f32| (v / SAMPLES as f32).sqrt());

    ColorRGBMapTo0_1::new(red, green, blue).into()
}

fn random_scene() -> HittableList<Arc<dyn Hittable>> {
    let mut world: HittableList<Arc<dyn Hittable>> = HittableList::new();

    let ground_material: Arc<Lambertian> =
        Arc::new(Lambertian::new(Attenuation::new(Vec3::new(0.5, 0.5, 0.5))));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material as Arc<dyn Material>,
    )));

    let mut rng: ThreadRng = rand::thread_rng();

    (-11..11).for_each(|a| {
        (-11..11).for_each(|b| {
            let choose_mat: f32 = random::<f32>();
            let center: Point3 = Point3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Attenuation::random() * Attenuation::random();
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(albedo)) as Arc<dyn Material>,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Attenuation::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(albedo, fuzz)) as Arc<dyn Material>,
                    )));
                } else {
                    // glass
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)) as Arc<dyn Material>,
                    )));
                }
            }
        });
    });

    let material1: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1 as Arc<dyn Material>,
    )));

    let material2: Arc<Lambertian> =
        Arc::new(Lambertian::new(Attenuation::new(Vec3::new(0.4, 0.2, 0.1))));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2 as Arc<dyn Material>,
    )));

    let material3: Arc<Metal> =
        Arc::new(Metal::new(Attenuation::new(Vec3::new(0.7, 0.6, 0.5)), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3 as Arc<dyn Material>,
    )));

    world
}
