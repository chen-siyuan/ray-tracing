use crate::vec3::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        from: Point3,
        at: Point3,
        vup: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let viewport_height: f64 = 2. * f64::tan(theta / 2.);
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (from - at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        let origin = from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - focus_dist * w - vertical / 2. - horizontal / 2.;
        let lens_radius = aperture / 2.;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = rd.0 * self.u + rd.1 * self.v;

        Ray {
            origin: self.origin + offset,
            direction: -self.origin - offset
                + self.lower_left_corner
                + s * self.horizontal
                + t * self.vertical,
        }
    }
}
