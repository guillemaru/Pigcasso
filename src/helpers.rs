use std::io::{self};
use image::{DynamicImage, GenericImageView, imageops::{self, FilterType}};
use std::env;

// Define the desired width for the output
pub static DESIRED_WIDTH: u32 = 300;

// Define an ASCII gradient
pub static ASCII_GRADIENT: &str = "B@$%&8#*+=-;:^'. ";

pub fn get_image_from_args() -> io::Result<DynamicImage> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let img = image::open(file_path).expect("Failed to open image");
    Ok(img)
}

pub fn calculate_new_dimensions(img: &DynamicImage) -> (u32, u32) {
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio = orig_height as f32 / orig_width as f32;
    let new_width = DESIRED_WIDTH;
    let new_height = (DESIRED_WIDTH as f32 * aspect_ratio / 2.0).round() as u32;
    (new_width, new_height)
}

pub fn process_image(img: &DynamicImage, new_width: u32, new_height: u32) -> Vec<u8> {
    let resized_img = img.resize(new_width, new_height, FilterType::Nearest);
    let gray_image = imageops::grayscale(&resized_img);
    gray_image.into_raw()
}

pub fn calculate_step_size() -> u8 {
    let gradient_length = ASCII_GRADIENT.chars().count();
    (255.0 / (gradient_length as f32)).round() as u8
}

pub fn convert_to_ascii(pixels: &[u8], new_width: u32, new_height: u32, step_size: u8) -> String {
    let half_width = new_width / 2;
    let half_height = new_height / 2;
    let mut ascii_art = String::new();

    for y in 0..half_height {
        for x in 0..half_width {
            let pixel_index = (y * new_width + x) as usize;
            let pixel = pixels[pixel_index];
            let ascii_char = ASCII_GRADIENT.chars().nth((pixel / step_size) as usize).unwrap_or(' ');
            ascii_art.push(ascii_char);
        }
        ascii_art.push('\n');
    }

    ascii_art
}