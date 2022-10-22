use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f64,
    pub point: Point3,
    pub front_face: bool,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
