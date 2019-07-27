use std::path::Path;
use std::env;
use mandelbrot::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Not enough arguments - program requires width, height, and iterations arguments")
    }

    let mut config = Config::new(
        args[1].parse().unwrap(),
        args[2].parse().unwrap(),
        args[3].parse().unwrap(),
    );
    run(&mut config);
    

    image::save_buffer(&Path::new("image.png"), &config.buffer, config.width, config.height, image::RGB(8)).unwrap();
}