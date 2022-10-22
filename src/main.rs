mod color;
mod point3;
mod ray;
mod vec3;

use crate::color::{write_color, Color};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> Option<Point3> {
    let co = ray.origin - center;
    let a = ray.direction * ray.direction;
    let half_b = co * ray.direction;
    let c = co * co - radius * radius;
    let determinant = half_b * half_b - a * c;
    if determinant < 0. {
        None
    } else {
        let t = (-half_b - f64::sqrt(determinant)) / a;
        Some(ray.at(t))
    }
}

fn ray_color(ray: &Ray) -> Color {
    let center = Point3(0., 0., -1.);
    let radius = 0.5;

    match hit_sphere(center, radius, ray) {
        Some(intersection) => {
            let normal = intersection - center;
            let unit_normal = normal.normalize();
            (unit_normal + Vec3(1., 1., 1.)) / 2.
        }
        None => {
            let unit_direction = ray.direction.normalize();
            Color(1.0, 1.0, 1.0).interpolate(Color(0.5, 0.7, 1.0), (-unit_direction.0 + 1.) / 2.)
        }
    }
}

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_height = 400;
    let image_width = (aspect_ratio * image_height as f64) as i32;

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Point3(0., 0., 0.);
    let vertical = Vec3(viewport_height, 0., 0.);
    let horizontal = Vec3(0., viewport_width, 0.);
    let upper_left_corner = origin + Vec3(0., 0., -focal_length) - vertical / 2. - horizontal / 2.;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in 0..image_height {
        eprint!("\rLines remaining: {} ", image_height - i);
        for j in 0..image_width {
            let u = i as f64 / (image_height - 1) as f64;
            let v = j as f64 / (image_width - 1) as f64;
            let ray = Ray {
                origin,
                direction: -origin + upper_left_corner + u * vertical + v * horizontal,
            };
            let color = ray_color(&ray);
            write_color(&color);
        }
    }
    eprintln!("\rLines remaining: {} ", 0);
    eprintln!("Done.");
}

#[cfg(test)]
mod tests {
    use crate::point3::Point3;
    use crate::ray::Ray;
    use crate::vec3::Vec3;

    #[test]
    fn vec_arithmetic() -> () {
        let z = Vec3(0., 0., 0.);
        let v1 = Vec3(32., -435., -9.);
        let v2 = Vec3(-2021., 3., 325.);
        let v3 = Vec3(-34., 12., -49.);

        assert_eq!(v1 + v2, v2 + v1);
        assert_eq!(v1 + v2 + v3, v1 + (v2 + v3));
        assert_eq!(v1, z + v1);
        assert_eq!(-v1, z - v1);
        assert_eq!(v1 - v2, -v2 + v1);
        assert_eq!(-v1 + -v2 + -v3, -(v1 + v2 + v3));
        assert_eq!(v1 / 2. + v1 / 2., v1);
        assert_eq!(v1 / 6., v1 / 2. / 3.);
    }

    #[test]
    fn ray_at() -> () {
        let p = Point3(1., 2., 3.);
        let v = Vec3(-4., -5., -6.);
        let r = Ray {
            origin: p,
            direction: v,
        };
        let t = 1.;

        let expected = Point3(-3., -3., -3.);
        let actual = r.at(t);

        assert_eq!(expected, actual);
    }
}
