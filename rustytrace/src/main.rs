// Add vector to scope
mod vector;
use std::io::{stderr, Write};
use vector::{Color, Vec3};

// Construct image Specs
const IMAGE_WIDTH: i64 = 256;
const IMAGE_HEIGHT: i64 = 256;

fn main() {
    // Render Image

    println!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (-1..IMAGE_HEIGHT - 1).rev().rev() {
        eprint!("\rScanlines remaining: {:3}\n", IMAGE_HEIGHT);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.25,
            );

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("Done.");
}
