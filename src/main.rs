use std::path::Path;
use std::env;
use mandelbrot::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Not enough arguments - program requires width, height, and iterations arguments")
    }

    // crappy argument parsing
    let width = args[1].parse().unwrap();
    let height = args[2].parse().unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    {
        let config = Config::new(
            width,
            height,
            args[3].parse().unwrap(),
            &mut buffer,
        );
        run(config);    
    }
    

    image::save_buffer(&Path::new("image.png"), &buffer, width, height, image::RGB(8)).unwrap();
}