use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Point3,
    upper_left_corner: Point3,
    vertical: Vec3,
    horizontal: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = Point3(0., 0., 0.);
        let vertical = Vec3(viewport_height, 0., 0.);
        let horizontal = Vec3(0., viewport_width, 0.);
        let upper_left_corner =
            origin + Vec3(0., 0., -focal_length) - vertical / 2. - horizontal / 2.;
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
