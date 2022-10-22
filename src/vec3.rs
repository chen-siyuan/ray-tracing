#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

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

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        1. / rhs * self
    }
}

impl Vec3 {
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
}
