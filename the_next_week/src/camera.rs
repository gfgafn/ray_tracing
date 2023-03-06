use in_one_weekend::{point::Point3, vec3::Vec3};
use rand::Rng;

use crate::ray::Ray;

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
    time_0: f32,
    time_1: f32,
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * Vec3::random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            rand::thread_rng().gen_range(self.time_0..self.time_1),
        )
    }
}

pub struct CameraBuilder {
    look_from: Point3,
    look_at: Point3,
    up: Vec3,
    v_fov: f32, // vertical field-of-view in degrees
    aspect_ratio: f32,
    aperture: f32,
    focus_dist: f32,
    time_0: f32, // shutter open times
    time_1: f32, // shutter close times
}

impl CameraBuilder {
    pub fn look_from(mut self, point: Point3) -> Self {
        self.look_from = point;
        self
    }

    pub fn look_at(mut self, point: Point3) -> Self {
        self.look_at = point;
        self
    }

    pub fn up(mut self, direction: Vec3) -> Self {
        self.up = direction;
        self
    }

    pub fn fov(mut self, degree: f32) -> Self {
        self.v_fov = degree;
        self
    }

    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = ratio;
        self
    }

    pub fn aperture(mut self, diameter: f32) -> Self {
        self.aperture = diameter;
        self
    }

    pub fn focus_dist(mut self, distance: f32) -> Self {
        self.focus_dist = distance;
        self
    }

    pub fn time_0(mut self, time: f32) -> Self {
        self.time_0 = time;
        self
    }

    pub fn time_1(mut self, time: f32) -> Self {
        self.time_1 = time;
        self
    }

    pub fn build(&self) -> Camera {
        debug_assert!(self.time_0 <= self.time_1);

        let theta: f32 = self.v_fov.to_radians();
        let h: f32 = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = self.aspect_ratio * viewport_height;

        let w: Vec3 = (self.look_from - self.look_at).unit_vector();
        let u: Vec3 = (self.up.cross(w)).unit_vector();
        let v: Vec3 = w.cross(u);

        let origin: Point3 = self.look_from;
        let horizontal: Vec3 = self.focus_dist * viewport_width * u;
        let vertical: Vec3 = self.focus_dist * viewport_height * v;
        let lower_left_corner: Point3 =
            origin - horizontal / 2.0 - vertical / 2.0 - self.focus_dist * w;

        let lens_radius: f32 = self.aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time_0: self.time_0,
            time_1: self.time_1,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            v_fov: 90.0,
            aspect_ratio: 1.0,
            aperture: 0.0,
            focus_dist: 0.0,
            time_0: f32::NEG_INFINITY,
            time_1: f32::INFINITY,
        }
    }
}
