use crate::vec3::{Point3, Vec3, Ray};

pub struct Camera {
    origin: Point3,
    upper_left_corner: Point3,
    vertical: Vec3,
    horizontal: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        const ASPECT_RATIO: f64 = 16. / 9.;
        const VIEWPORT_HEIGHT: f64 = 2.;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.;

        let origin = Point3(0., 0., 0.);
        let vertical = Vec3(VIEWPORT_HEIGHT, 0., 0.);
        let horizontal = Vec3(0., VIEWPORT_WIDTH, 0.);
        let upper_left_corner =
            origin + Vec3(0., 0., -FOCAL_LENGTH) - vertical / 2. - horizontal / 2.;
        Camera {
            origin,
            upper_left_corner,
            vertical,
            horizontal,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: -self.origin
                + self.upper_left_corner
                + u * self.vertical
                + v * self.horizontal,
        }
    }
}
