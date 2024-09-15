use image::{DynamicImage, ImageError, GenericImageView, imageops::{self, FilterType}};
use std::env;

// Define an ASCII gradient
// and STEP_SIZE: how many numbers (of the 0-255 that represents the intensity
// of a pixel in the grayscale image) does each ascii character own
pub const ASCII_GRADIENT: &str = "B@$%&8#*+=-;:^'. ";
pub const GRADIENT_LENGTH: usize = ASCII_GRADIENT.len();
pub const STEP_SIZE: u8 = 255 / GRADIENT_LENGTH as u8;

pub fn get_args() -> (Result<DynamicImage, ImageError>, Result<u32, std::num::ParseIntError>) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <desired_width>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let desired_width = args[2].parse::<u32>();
    let img = image::open(file_path);
    (img, desired_width)
}

pub fn calculate_new_dimensions(img: &DynamicImage, desired_width: u32) -> (u32, u32) {
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio = orig_height as f32 / orig_width as f32;
    let new_width = desired_width;
    let new_height = (desired_width as f32 * aspect_ratio / 2.0).round() as u32;
    (new_width, new_height)
}

pub fn process_image(img: &DynamicImage, new_width: u32, new_height: u32) -> Vec<u8> {
    let resized_img = img.resize_exact(new_width, new_height, FilterType::Nearest);
    let gray_image = imageops::grayscale(&resized_img);
    gray_image.into_raw()
}

pub fn convert_to_ascii(pixels: &[u8], new_width: u32, new_height: u32, step_size: u8) -> String {
    let mut ascii_art = String::new();

    for y in 0..new_height {
        for x in 0..new_width {
            let pixel_index = (y * new_width + x) as usize;
            let pixel = pixels[pixel_index];
            let ascii_char = ASCII_GRADIENT.chars().nth((pixel / step_size) as usize).unwrap_or(' ');
            ascii_art.push(ascii_char);
        }
        ascii_art.push('\n');
    }

    ascii_art
}