mod helpers;

use std::io::{self};
use helpers::{get_image_from_args, calculate_new_dimensions, process_image, calculate_step_size, convert_to_ascii};

fn main() -> io::Result<()> {
    // Collect the command line arguments and open the image
    let img = get_image_from_args()?;

    let (new_width, new_height) = calculate_new_dimensions(&img);

    let pixels = process_image(&img, new_width, new_height);

    let step_size = calculate_step_size();

    let ascii_art = convert_to_ascii(&pixels, new_width, new_height, step_size);

    println!("{}", ascii_art);

    Ok(())
}
