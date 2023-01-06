// Add vector to scope
mod vector;
mod ray;

use std::io::{stderr, Write};
use vector::{Color, Vec3, Point3};
use ray::Ray;


fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    
    // Setup Const
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;

    // Setup Camera
    let view_height = 2.0;
    let view_width = ASPECT_RATIO * view_height;
    let focal_legnth = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(view_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, view_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_legnth);
    
    
    // Render Image

    println!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}\n", IMAGE_HEIGHT);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&ray);

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("Done.");
}
