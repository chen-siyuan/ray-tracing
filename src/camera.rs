use crate::vec3::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vertical_fov: f64, aspect_ratio: f64) -> Self {
        let theta = vertical_fov.to_radians();
        let viewport_height: f64 = 2. * f64::tan(theta / 2.);
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.;

        let origin = Point3(0., 0., 0.);
        let horizontal = Vec3(viewport_width, 0., 0.);
        let vertical = Vec3(0., viewport_height, 0.);
        let lower_left_corner =
            origin + Vec3(0., 0., -focal_length) - vertical / 2. - horizontal / 2.;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: -self.origin
                + self.lower_left_corner
                + u * self.horizontal
                + v * self.vertical,
        }
    }
}
