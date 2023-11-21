use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color, samples_per_pixel: i32) {
    let color = color / samples_per_pixel as f64;

    let ir = (255.0 * color.x.clamp(0.0, 1.0)) as i32;
    let ig = (255.0 * color.y.clamp(0.0, 1.0)) as i32;
    let ib = (255.0 * color.z.clamp(0.0, 1.0)) as i32;

    println!("{ir} {ig} {ib}")
}
