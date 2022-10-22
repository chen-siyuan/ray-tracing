mod color;
mod point3;
mod ray;
mod vec3;

use color::write_color;
use color::Color;

fn main() {
    let height = 256;
    let width = 256;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for i in (0..height).rev() {
        eprint!("\rScanlines remaining: {} ", i);

        for j in 0..width {
            let p = Color(
                j as f64 / (width - 1) as f64,
                i as f64 / (height - 1) as f64,
                0.25,
            );
            write_color(p);
        }
    }
    eprintln!("\nDone.");
}

#[cfg(test)]
mod tests {
    use crate::point3::Point3;
    use crate::ray::Ray;
    use crate::vec3::Vec3;

    #[test]
    fn foo() -> () {
        let p = Point3(1., 2., 3.);
        let v = Vec3(-4., -5., -6.);
        let r = Ray { orig: p, dir: v };
        let t = 1.;

        let expected = Point3(-3., -3., -3.);
        let actual = r.at(t);

        assert_eq!(expected, actual);
    }
}
