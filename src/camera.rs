use crate::vec3::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(from: Point3, at: Point3, vup: Vec3, vertical_fov: f64, aspect_ratio: f64) -> Self {
        let theta = vertical_fov.to_radians();
        let viewport_height: f64 = 2. * f64::tan(theta / 2.);
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (from - at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - w - vertical / 2. - horizontal / 2.;
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
