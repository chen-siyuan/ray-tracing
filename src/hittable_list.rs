use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList<'a> {
    pub objects: Vec<&'a dyn Hittable>,
}

impl<'a> Hittable for HittableList<'a> {
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
