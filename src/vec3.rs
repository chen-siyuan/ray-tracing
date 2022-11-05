#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);
use crate::material::Dielectric;
pub use Vec3 as Point3;
pub use Vec3 as Color;

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        1. / rhs * self
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
    }
}

impl Vec3 {
    pub fn near_zero(self) -> bool {
        const EPSILON: f64 = 1e-8;
        self.0.abs() < EPSILON && self.1.abs() < EPSILON && self.2.abs() < EPSILON
    }

    pub fn near(self, other: Vec3) -> bool {
        (self - other).near_zero()
    }

    pub fn magnitude(self) -> f64 {
        f64::sqrt(self * self)
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn interpolate(self, other: Vec3, k: f64) -> Vec3 {
        (1. - k) * self + k * other
    }

    pub fn mul(self, other: Vec3) -> Vec3 {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2. * (self * normal) * normal
    }

    pub fn refract(self, normal: Vec3, ratio: f64) -> Option<Vec3> {
        let cos_theta = -self * normal;
        let sin_theta = f64::sqrt(1. - cos_theta * cos_theta);
        match ratio * sin_theta > 1. || Dielectric::reflectance(cos_theta, ratio) > rand::random() {
            true => None,
            false => {
                let perpendicular = ratio * (self + cos_theta * normal);
                let parallel = -f64::sqrt(1. - perpendicular * perpendicular) * normal;
                Some(perpendicular + parallel)
            }
        }
    }

    pub fn random(min: f64, max: f64) -> Self {
        Vec3(
            min + rand::random::<f64>() * (max - min),
            min + rand::random::<f64>() * (max - min),
            min + rand::random::<f64>() * (max - min),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random(-1., 1.);
            if v * v <= 1. {
                break v;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let v = Self::random_in_unit_sphere();
            if v * v != 0. {
                break v.normalize();
            }
        }
    }

    pub fn random_in_hemisphere(normal: Self) -> Self {
        let v = Self::random_in_unit_sphere();
        if v * normal > 0. {
            v
        } else {
            -v
        }
    }
}

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.direction;
    }
}
