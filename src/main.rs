mod helpers;

use std::io::{self};
use helpers::{STEP_SIZE, get_args, calculate_new_dimensions, process_image, convert_to_ascii};

fn main() -> io::Result<()> {
    let (img, desired_width) = get_args();

    match (img, desired_width) {
        (Ok(ref img), Ok(desired_width)) => {
            let (new_width, new_height) = calculate_new_dimensions(&img, desired_width);

            let pixels = process_image(&img, new_width, new_height);

            let ascii_art = convert_to_ascii(&pixels, new_width, new_height, STEP_SIZE);

            println!("{}", ascii_art);
        }
        (Err(e), _) => eprintln!("Failed to open image: {}", e),
        (_, Err(e)) => eprintln!("Failed to parse width: {}", e),
    }

    Ok(())
}
