mod color;
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
            write_color(&p);
        }
    }
    eprintln!("\nDone.");
}
