use crate::{point::Point3, ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, up: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let theta: f32 = vfov.to_radians();
        let h: f32 = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w: Vec3 = (look_from - look_at).unit_vector();
        let u: Vec3 = (up.cross(w)).unit_vector();
        let v: Vec3 = w.cross(u);

        let origin: Point3 = look_from;
        let horizontal: Vec3 = viewport_width * u;
        let vertical: Vec3 = viewport_height * v;
        let lower_left_corner: Point3 = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        use crate::ASPECT_RATIO;
        const VIEWPORT_HEIGHT: f32 = 2.0;
        const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.0;

        const ORIGIN: Point3 = Vec3(0.0, 0.0, 0.0);
        const HORIZONTAL: Vec3 = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
        const VERTICAL: Vec3 = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);

        Self {
            origin: ORIGIN,
            lower_left_corner: ORIGIN
                - HORIZONTAL / 2.0
                - VERTICAL / 2.0
                - Vec3(0.0, 0.0, FOCAL_LENGTH),
            horizontal: HORIZONTAL,
            vertical: VERTICAL,
        }
    }
}
