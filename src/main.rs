use std::path::Path;

fn main() {
    let width: u32 = 1000;
    let height: u32 = 1000;
    let max = 10000;

    let mut colors: [(u8, u8, u8); 10000] = [(0, 0, 0); 10000];
    for i in 0..max {
        let i = i as f32;
        let l = i / (i + 8.0);
        colors[i as usize] = (((l * 255.0) as u8, (i / max as f32 * 255.0) as u8, 0u8))
    }

    let mut buffer: Vec<u8> = Vec::new();
    buffer.reserve((height * width * 3u32) as usize);
    for row in 0..height {
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
                let (r, g, b) = colors[iterations];
                buffer.push(r);
                buffer.push(g);
                buffer.push(b);

            } else {
                for _ in 0..3 {
                    buffer.push(0);
                }
            }
        }
    }

    image::save_buffer(&Path::new("image.png"), &buffer, width, height, image::RGB(8)).unwrap();
}