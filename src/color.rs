pub use crate::vec3::Vec3 as Color;

const MAX_COLOR: f64 = 255.999;

pub fn write_color(color: &Color) -> () {
    let Color(r, g, b) = color;

    let ir = (MAX_COLOR * r).floor() as i32;
    let ig = (MAX_COLOR * g).floor() as i32;
    let ib = (MAX_COLOR * b).floor() as i32;

    println!("{} {} {}", ir, ig, ib);
}

impl Color {
    pub fn interpolate(&self, other: &Color, k: f64) -> Color {
        (1. - k) * *self + k * *other
    }

    pub fn clamp(&self) -> Color {
        Color(
            f64::clamp(self.0, 0., 1.),
            f64::clamp(self.1, 0., 1.),
            f64::clamp(self.2, 0., 1.),
        )
    }
}
