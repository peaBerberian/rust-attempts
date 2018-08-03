extern crate image;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Write;

struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        writeln!(std::io::stderr(), "You forgot to set the filename wanted")
            .unwrap();
        std::process::exit(1);
    }
    let output_filename = &args[1];
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: generate_grayscale_vec((width, height)),
        size: (width, height)
    };

    let output = File::create(output_filename).expect("error creating file");
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &image.pixels,
        image.size.0 as u32,
        image.size.1 as u32,
        ColorType::Gray(8)
        ).expect("error encoding file");
}

/// Returns a Vec<u8>
fn generate_grayscale_vec((width, height) : (usize, usize)) -> Vec<u8> {
    let mut current_value : u8 = 0;
    let mut res : Vec<u8> = Vec::with_capacity(width * height);
    for _ in 0 .. width {
        for _ in 0 .. height {
            current_value = if current_value < 255 {
                current_value + 1
            } else {
                0
            };
            res.push(current_value);
        }
    }
    res
}
