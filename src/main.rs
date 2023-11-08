mod color;
mod vec3;
use crate::{color::write_color, vec3::Vec3};

fn main() {
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        let remaining = image_height - j;
        eprintln!("Scanlines remaining: {remaining}");
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;
            let pixel_color = Vec3::new(r, g, b);
            write_color(&pixel_color);
        }
    }
    eprintln!("Done")
}
