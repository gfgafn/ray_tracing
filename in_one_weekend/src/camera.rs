use crate::{point::Point3, ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        origin: Point3,
        lower_left_corner: Point3,
        horizontal: Vec3,
        vertical: Vec3,
    ) -> Self {
        assert_eq!(0.0, horizontal.dot(vertical));
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
