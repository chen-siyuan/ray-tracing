mod color;
mod hittable;
mod hittable_list;
mod point3;
mod ray;
mod sphere;
mod vec3;

use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    match world.hit(ray, 0., f64::MAX) {
        Some(HitRecord {
            t: _,
            point: _,
            front_face: _,
            normal,
        }) => (normal + Vec3(1., 1., 1.)) / 2.,
        None => {
            let unit_direction = ray.direction.normalize();
            Color(1.0, 1.0, 1.0).interpolate(Color(0.5, 0.7, 1.0), (-unit_direction.0 + 1.) / 2.)
        }
    }
}

fn main() {
    // Image

    let aspect_ratio = 16. / 9.;
    let image_height = 400;
    let image_width = (aspect_ratio * image_height as f64) as i32;

    // World

    let mut world = HittableList { objects: vec![] };
    world.objects.push(&Sphere {
        center: Point3(0., 0., -1.),
        radius: 0.5,
    });
    world.objects.push(&Sphere {
        center: Point3(100.5, 0., -1.),
        radius: 100.,
    });

    // Camera

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Point3(0., 0., 0.);
    let vertical = Vec3(viewport_height, 0., 0.);
    let horizontal = Vec3(0., viewport_width, 0.);
    let upper_left_corner = origin + Vec3(0., 0., -focal_length) - vertical / 2. - horizontal / 2.;

    // Render

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
            let color = ray_color(&ray, &world);
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
