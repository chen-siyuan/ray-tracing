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

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        Some((
            Color(1., 1., 1.),
            Ray {
                origin: record.point,
                direction: ray.direction.normalize().refract(
                    record.normal,
                    if record.front_face {
                        1. / self.ir
                    } else {
                        self.ir
                    },
                ),
            },
        ))
    }
}
