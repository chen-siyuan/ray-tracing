use crate::vec3::{Point3, Vec3, Ray};

pub struct HitRecord {
    pub t: f64,
    pub point: Point3,
    pub front_face: bool,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(ray: &Ray, t: f64, point: &Point3, outward_normal: &Vec3) -> Self {
        if ray.direction * *outward_normal < 0. {
            HitRecord {
                t,
                point: *point,
                front_face: true,
                normal: *outward_normal,
            }
        } else {
            HitRecord {
                t,
                point: *point,
                front_face: false,
                normal: -*outward_normal,
            }
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_curr = t_max;
        let mut res = None;
        for object in self.objects.iter() {
            if let Some(tmp) = object.hit(ray, t_min, t_curr) {
                t_curr = tmp.t;
                res = Some(tmp);
            }
        }
        res
    }
}

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
