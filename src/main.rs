fn main() {
    let height = 256;
    let width = 256;
    let max_color = 255.999;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for i in (0..height).rev() {
        for j in 0..width {
            let r = j as f64 / (width - 1) as f64;
            let g = i as f64 / (height - 1) as f64;
            let b = 0.25;

            let ir = (max_color * r).floor() as i32;
            let ig = (max_color * g).floor() as i32;
            let ib = (max_color * b).floor() as i32;

            println!("{} {} {}", ir, ig, ib)
        }
    }
}
