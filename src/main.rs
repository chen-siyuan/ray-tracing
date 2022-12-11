extern crate rand;

use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList, Sphere};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::vec3::{Color, Ray, Vec3};

mod camera;
mod hittable;
mod material;
mod vec3;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    const EPSILON: f64 = 0.001;
    const INFINITY: f64 = f64::MAX;

    if depth == 0 {
        return Color(0., 0., 0.);
    }
    match world.hit(ray, EPSILON, INFINITY) {
        Some(record) => match record.material.scatter(ray, &record) {
            Some((attenuation, scattered)) => {
                attenuation.mul(ray_color(&scattered, world, depth - 1))
            }
            None => Color(0., 0., 0.),
        },
        None => {
            let unit_direction = ray.direction.normalize();
            Color(1., 1., 1.).interpolate(Color(0.5, 0.7, 1.), (-unit_direction.0 + 1.) / 2.)
        }
    }
}

fn write_color(color: &Color) -> () {
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

fn random_world() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    world.objects.push(Box::new(Sphere {
        center: Vec3(0., -1000., 0.),
        radius: 1000.,
        material: Box::new(Lambertian {
            albedo: Color(0.5, 0.5, 0.5),
        }),
    }));

    for i in -11..11 {
        for j in -11..11 {
            let rand_material = rand::random::<f64>();
            let center = Vec3(
                i as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                j as f64 + 0.9 * rand::random::<f64>(),
            );
            if (center - Vec3(4., 0.2, 0.)).magnitude() > 0.9 {
                let material: Box<dyn Material> = if rand_material < 0.8 {
                    Box::new(Lambertian {
                        albedo: Color::random(0., 1.).mul(Color::random(0., 1.)),
                    })
                } else if rand_material < 0.95 {
                    Box::new(Metal {
                        albedo: Color::random(0.5, 1.),
                        fuzz: rand::random::<f64>() / 2.,
                    })
                } else {
                    Box::new(Dielectric { ir: 1.5 })
                };
                world.objects.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material,
                }));
            }
        }
    }

    world.objects.push(Box::new(Sphere {
        center: Vec3(0., 1., 0.),
        radius: 1.,
        material: Box::new(Dielectric { ir: 1.5 }),
    }));
    world.objects.push(Box::new(Sphere {
        center: Vec3(-4., 1., 0.),
        radius: 1.,
        material: Box::new(Lambertian {
            albedo: Color(0.4, 0.2, 0.1),
        }),
    }));
    world.objects.push(Box::new(Sphere {
        center: Vec3(4., 1., 0.),
        radius: 1.,
        material: Box::new(Metal {
            albedo: Color(0.7, 0.6, 0.5),
            fuzz: 0.,
        }),
    }));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3. / 2.;
    const IMAGE_HEIGHT: i32 = 1200;
    const IMAGE_WIDTH: i32 = (ASPECT_RATIO * IMAGE_HEIGHT as f64) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_world();

    // Camera
    let from = Vec3(13., 2., 3.);
    let at = Vec3(0., 0., 0.);
    let vup = Vec3(0., 1., 0.);
    let aperture = 0.1;
    let dist_to_focus = 10.;
    let camera = Camera::new(from, at, vup, 20., ASPECT_RATIO, aperture, dist_to_focus);

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rLines remaining: {} ", j + 1);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / IMAGE_WIDTH as f64;
                let v = (j as f64 + rand::random::<f64>()) / IMAGE_HEIGHT as f64;
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
    use crate::vec3::{Ray, Vec3};

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
        let p = Vec3(1., 2., 3.);
        let v = Vec3(-4., -5., -6.);
        let r = Ray {
            origin: p,
            direction: v,
        };
        let t = 1.;

        let expected = Vec3(-3., -3., -3.);
        let actual = r.at(t);

        assert_eq!(expected, actual);
    }

    #[test]
    fn refract() -> () {
        assert!(Vec3(1., -f64::sqrt(3.), 0.)
            .normalize()
            .refract(Vec3(0., 1., 0.), f64::sqrt(2.))
            .unwrap()
            .normalize()
            .near(Vec3(1., -1., 0.).normalize()));
        assert!(Vec3(1., -1., 0.)
            .normalize()
            .refract(Vec3(0., 1., 0.), f64::sqrt(1. / 2.))
            .unwrap()
            .normalize()
            .near(Vec3(1., -f64::sqrt(3.), 0.).normalize()));
        assert!(Vec3(f64::sqrt(3.), -1., 0.)
            .normalize()
            .refract(Vec3(0., 1., 0.), f64::sqrt(1. / 3.))
            .unwrap()
            .normalize()
            .near(Vec3(1., -f64::sqrt(3.), 0.).normalize()));
    }
}
