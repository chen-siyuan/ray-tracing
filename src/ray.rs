use crate::point3::Point3;
use crate::vec3::Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        return self.orig + t * self.dir;
    }
}
