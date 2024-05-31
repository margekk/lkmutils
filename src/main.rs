/*  TODO:
 *      - Add functions to calculate invariants
 *      - Add function to procedurally generate knots in a reasonable way
 *      - Find way to rotate output images
 *      - 
 */

use image::{GenericImageView, ImageBuffer, RgbaImage, Rgba};
use std::path::Path;
use std::collections::HashMap;
use std::io;
use dialoguer::Confirm;
use dialoguer::Input;

fn toVec() {

}

fn main() {
    // Define the matrix to hold the input

    println!("Please enter the rows of the knot mosaic, one row per line.");
    println!("Each row should contain tile numbers between 0 and 10, excluding 9, separate by spaces.");
    println!("Enter an empty line to finish input.");
    
    let mut matrix: Vec<Vec<u8>> = Vec::new();

    loop {
        //Stores current row
        let mut input = String::new();

        // Read the input from stdin
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim any whitespace from the input
        let trimmed = input.trim();

        // Break the loop if the input is empty (i.e., the user pressed Enter without typing anything)
        if trimmed.is_empty() {
            break;
        }

        let row: Result<Vec<u8>, _> = trimmed.split_whitespace()
            .map(|s| match s.parse::<u8>() {
                Ok(num) if num <= 10 && num != 9 => Ok(num),
                _ => Err("Invalid number")
            })
            .collect();

// Handle possible parsing errors
        match row {
            Ok(vec) => { 
                if !matrix.is_empty() && matrix[0].len() != vec.len() {
                    println!("All rows must have the same number of elements.");
                    continue;
                }
                matrix.push(vec);
            }
            Err(_) => {
                println!("Invalid input. Please enter a row of integers.");
                continue;
            }
        }
    
//todo:check for connectedness, invariants

    }

    
    // Print the resulting matrix 
    println!("parsed mosaic: {:?}", matrix);

    let save_confirmation = Confirm::new()
        .with_prompt("Save to file as .png?")
        .interact()
        .unwrap();

    if save_confirmation { 
        let save_name: String = Input::new()
            .with_prompt("Enter file name: ")
            .interact_text()
            .unwrap();
        create_knot_mosaic(&matrix,&save_name);
    }
}

fn create_knot_mosaic(matrix: &Vec<Vec<u8>>, output_filename: &str) {
    let tile_size = 115; 
    let mut tile_images = HashMap::new();
    let border_size = 4; 
    let border_color = Rgba([196, 196, 196, 255]); 

    // Load the tile images
    for num in 0..11 {
        if num != 9 {
            let file_name = format!("tiles/{}.png", num); // Assuming tiles are named "0.png", "1.png", etc.
            let img = image::open(&Path::new(&file_name)).expect(&format!("Failed to load image {}", file_name));
            tile_images.insert(num, img.to_rgba8());
        }
    }

    // Determine the size of the final mosaic image
    let rows = matrix.len();
    let mosaic_width = rows * tile_size + 2 * border_size;


    // Create a new image buffer for the mosaic
    let mut mosaic: RgbaImage = ImageBuffer::new(mosaic_width as u32, mosaic_width as u32);

    //Introduces border color
    for y in 0..mosaic_width {
        for x in 0..mosaic_width {
            mosaic.put_pixel(x as u32, y as u32, border_color);
        }
    }
    // Place the tiles in the mosaic image
    for (i, row) in matrix.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            if let Some(tile) = tile_images.get(&num) {
                for y in 0..tile_size {
                    for x in 0..tile_size {
                        let pixel = tile.get_pixel(x as u32, y as u32);
                        mosaic.put_pixel((j * tile_size + x + border_size) as u32, (i * tile_size + y + border_size) as u32, *pixel);
                    }
                }
            }
        }
    }
    // Save the mosaic image to disk
    mosaic.save(output_filename).expect("Failed to save mosaic image");
}
