use crate::{point::Point3, ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    #[allow(unused)]
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        up: Vec3,
        v_fov: f32, // vertical field-of-view in degrees
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta: f32 = v_fov.to_radians();
        let h: f32 = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w: Vec3 = (look_from - look_at).unit_vector();
        let u: Vec3 = (up.cross(w)).unit_vector();
        let v: Vec3 = w.cross(u);

        let origin: Point3 = look_from;
        let horizontal: Vec3 = focus_dist * viewport_width * u;
        let vertical: Vec3 = focus_dist * viewport_height * v;
        let lower_left_corner: Point3 = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius: f32 = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * Vec3::random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
