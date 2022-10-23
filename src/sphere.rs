use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let co = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let half_b = co * ray.direction;
        let c = co * co - self.radius * self.radius;
        let determinant = half_b * half_b - a * c;
        if determinant < 0. {
            return None;
        }
        let sqrt_determinant = f64::sqrt(determinant);
        let t = (-half_b - sqrt_determinant) / a;
        if !(t_min <= t && t <= t_max) {
            let t = (-half_b + sqrt_determinant) / a;
            if !(t_min <= t && t <= t_max) {
                return None;
            }
        }
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(ray, t, &point, &outward_normal))
    }
}
