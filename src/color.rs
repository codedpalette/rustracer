use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(color: &Color) {
    let ir = (255.0 * color.x) as i32;
    let ig = (255.0 * color.y) as i32;
    let ib = (255.0 * color.z) as i32;

    println!("{ir} {ig} {ib}")
}
