use palette::{Hsv, rgb};

pub struct MandelbrotConfig {
    pub width: u32,
    pub height: u32,
    pub max_iterations: u32,
    pub buffer: Vec<u8>,
}

impl MandelbrotConfig {
    pub fn new(args: &[String]) -> Result<MandelbrotConfig, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments - program requires width, height, and iterations arguments");
        }

        let width: u32 = args[1].parse()
            .expect("Unable to parse the width");
        let height: u32 = args[2].parse()
            .expect("Unable to parse height");
        let max_iterations: u32 = args[3].parse()
            .expect("Unable to parse iterations");
        let mut buffer = Vec::new();
        buffer.reserve((width * height * 3u32) as usize);
        Ok(MandelbrotConfig {
            width,
            height,
            max_iterations,
            buffer,
        })
    }
}

pub fn generate_mandelbrot(config: &mut MandelbrotConfig) {
    let MandelbrotConfig{ width, height, max_iterations, buffer } = config;
    for row in 0..*height {
        for col in 0..*width {
            let real_c = (col as f64 - *width as f64 / 2.0) * 4.0 / *width as f64;
            let imaginary_c = (row as f64 - *height as f64 / 2.0) * 4.0 / *width as f64;

            let mut x: f64 = 0.0;
            let mut y: f64 = 0.0;
            let mut iterations = 0;
            while x.powi(2) + y.powi(2) < 4.0 && iterations < *max_iterations {
                let x_new = x.powi(2) - y.powi(2) + real_c;
                y = 2.0 * x * y + imaginary_c;
                x = x_new;
                iterations += 1;
            }
            if iterations < *max_iterations {
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
}