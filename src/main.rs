use std::env;
use std::io::{self};
use image::{GenericImageView, imageops::{self, FilterType}};

// Define the desired width for the output
static DESIRED_WIDTH: u32 = 300;

// Define an ASCII gradient
static ASCII_GRADIENT: &str = "B@$%&8#*+=-;:^'. ";

fn main() -> io::Result<()> {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();
    // Ensure a file path is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    // Get the file path from the arguments
    let file_path = &args[1];
    // Open the image
    let img = image::open(file_path).expect("Failed to open image");

    // Get the dimensions and aspect ratio
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio = orig_height as f32 / orig_width as f32;
    // Calculate the new dimensions
    // height should have half the amount of characters as width (if aspect ratio was 1),
    // given that ascii characters are higher than wider in general
    let new_width = DESIRED_WIDTH;
    let new_height = (DESIRED_WIDTH as f32 * aspect_ratio / 2.0).round() as u32;

    // Resize the image to fit within the maximum dimensions
    let resized_img = img.resize(new_width, new_height, FilterType::Nearest);
    // Convert the resized image to grayscale
    let gray_image = imageops::grayscale(&resized_img);
    // Get the raw pixels of the grayscale image
    let pixels = gray_image.into_raw();

    // Get the step size (how many of the 255 bits an ascii character represents)
    let gradient_length = ASCII_GRADIENT.chars().count();
    let step_size = (255.0 / (gradient_length as f32)).round() as u8;

    // Loop through the bitmap representation
    let half_width = new_width / 2;
    let half_height = new_height / 2;
    for y in 0..half_height {
        for x in 0..half_width {
            let pixel_index = (y * new_width + x) as usize;
            let pixel = pixels[pixel_index];
            // Map pixel value (0-255) to an ASCII character
            let ascii_char = ASCII_GRADIENT.chars().nth((pixel / step_size) as usize).unwrap_or(' ');
            print!("{}", ascii_char);
        }
        println!(); // New line at the end of each row
    }

    Ok(())
}