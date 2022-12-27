 // Construct image Specs

const IMAGE_WIDTH: i64 = 256;
const IMAGE_HEIGHT: i64 = 256;

fn main() {
    

    // Render Image

    println!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (-1..IMAGE_HEIGHT-1).rev().rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64 ;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{} {} {}\n", ir, ig, ib);
        }

    }

}
