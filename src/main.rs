use std::path::Path;
use palette::{Hsv, rgb};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Not enough arguments - program requires width, height, and iterations arguments")
    }

    // crappy argument parsing
    let width: u32 = args[1].parse().unwrap();
    let height: u32 = args[1].parse().unwrap();
    let max = args[1].parse().unwrap();

    let mut colors: [(u8, u8, u8); 10000] = [(0, 0, 0); 10000];
    for i in 0..max {
        let i = i as f32;
        let l = i / (i + 8.0);
        colors[i as usize] = ((l * 255.0) as u8, (i / max as f32 * 255.0) as u8, 0u8);
    }

    let mut buffer: Vec<u8> = Vec::new();
    buffer.reserve((height * width * 3u32) as usize);
    for row in 0..height {
        //println!("row {} of {}. {}% complete", row, height, (row as f32 / height as f32) * 100.0);
        for col in 0..width {
            let real_c = (col as f64 - width as f64 / 2.0) * 4.0 / width as f64;
            let imaginary_c = (row as f64 - height as f64 / 2.0) * 4.0 / width as f64;

            let mut x: f64 = 0.0;
            let mut y: f64 = 0.0;
            let mut iterations = 0;
            while x.powi(2) + y.powi(2) < 4.0 && iterations < max {
                let x_new = x.powi(2) - y.powi(2) + real_c;
                y = 2.0 * x * y + imaginary_c;
                x = x_new;
                iterations += 1;
            }
            if iterations < max {
                let zn = (x.powi(2) + y.powi(2)).sqrt();
                let nsmooth = iterations as f64 + 1.0f64 - zn.abs().log10().log10() / 2.0f64.log10();
                let color: rgb::Rgb = rgb::Rgb::from(Hsv::new(0.95 + 10.0 * nsmooth as f32, 0.6, 1.0));
                let (r, g, b) = color.into_components();
                buffer.push((r * 255.0) as u8);
                buffer.push((g * 255.0) as u8);
                buffer.push((b * 255.0) as u8);

            } else {
                for _ in 0..3 {
                    buffer.push(0);
                }
            }
        }
    }

    image::save_buffer(&Path::new("image.png"), &buffer, width, height, image::RGB(8)).unwrap();
}