use crate::hittable::HitRecord;
use crate::vec3::{Color, Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut direction = record.normal + Vec3::random_unit_vector();
        if direction.near_zero() {
            direction = record.normal;
        }
        Some((
            self.albedo,
            Ray {
                origin: record.point,
                direction,
            },
        ))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let direction =
            ray.direction.reflect(record.normal) + self.fuzz * Vec3::random_in_unit_sphere();
        match direction * record.normal > 0. {
            true => Some((
                self.albedo,
                Ray {
                    origin: record.point,
                    direction,
                },
            )),
            false => None,
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn reflectance(cos_theta: f64, ratio: f64) -> f64 {
        let r0 = f64::powi((1. - ratio) / (1. + ratio), 2);
        r0 + (1. - r0) * f64::powi(1. - cos_theta, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let unit_direction = ray.direction.normalize();
        let ratio = if record.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let direction = match unit_direction.refract(record.normal, ratio) {
            Some(direction) => direction,
            None => unit_direction.reflect(record.normal),
        };
        Some((
            Color(1., 1., 1.),
            Ray {
                origin: record.point,
                direction,
            },
        ))
    }
}
