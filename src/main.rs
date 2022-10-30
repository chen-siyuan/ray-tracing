extern crate rand;

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    const EPSILON: f64 = 0.001;
    const INFINITY: f64 = f64::MAX;

    if depth == 0 {
        return Color(0., 0., 0.);
    }
    match world.hit(ray, EPSILON, INFINITY) {
        Some(HitRecord {
            t: _,
            point,
            front_face: _,
            normal,
        }) => {
            let target = point + Vec3::random_in_hemisphere(&normal);
            0.5 * ray_color(
                &Ray {
                    origin: point,
                    direction: target - point,
                },
                world,
                depth - 1,
            )
        }
        None => {
            let unit_direction = ray.direction.normalize();
            Color(1.0, 1.0, 1.0).interpolate(&Color(0.5, 0.7, 1.0), (-unit_direction.0 + 1.) / 2.)
        }
    }
}

pub fn write_color(color: &Color) -> () {
    const MAX_COLOR: f64 = 255.999;

    let Color(r, g, b) = color;

    let r = f64::sqrt(r.clamp(0., 1.));
    let g = f64::sqrt(g.clamp(0., 1.));
    let b = f64::sqrt(b.clamp(0., 1.));

    let ir = (MAX_COLOR * r).floor() as i32;
    let ig = (MAX_COLOR * g).floor() as i32;
    let ib = (MAX_COLOR * b).floor() as i32;

    println!("{} {} {}", ir, ig, ib);
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_HEIGHT: i32 = 400;
    const IMAGE_WIDTH: i32 = (ASPECT_RATIO * IMAGE_HEIGHT as f64) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList { objects: vec![] };
    world.objects.push(Box::new(Sphere {
        center: Point3(0., 0., -1.),
        radius: 0.5,
    }));
    world.objects.push(Box::new(Sphere {
        center: Point3(100.5, 0., -1.),
        radius: 100.,
    }));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for i in 0..IMAGE_HEIGHT {
        eprint!("\rLines remaining: {} ", IMAGE_HEIGHT - i);
        for j in 0..IMAGE_WIDTH {
            let mut color = Color(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / IMAGE_HEIGHT as f64;
                let v = (j as f64 + rand::random::<f64>()) / IMAGE_WIDTH as f64;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, MAX_DEPTH);
            }
            color /= SAMPLES_PER_PIXEL as f64;
            write_color(&color);
        }
    }
    eprintln!("\rLines remaining: {} ", 0);

    eprintln!("Done.");
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::vec3::{Point3, Vec3};

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
