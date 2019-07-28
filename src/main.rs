use std::path::Path;
use std::env;
use mandelbrot::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config = MandelbrotConfig::new(&args).unwrap_or_else(|err| {
        panic!("Failed to init configuration: {}", err);
    });
    generate_mandelbrot(&mut config);
    
    image::save_buffer(&Path::new("image.png"), &config.buffer, config.width, config.height, image::RGB(8))
        .expect("Unable to save image");
}