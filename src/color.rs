use crate::vec3::Vec3;

pub type Color = Vec3;

#[inline]
fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

pub fn write_color(color: Color, samples_per_pixel: i32) {
    // Average the color by the number of samples
    let color = color / samples_per_pixel as f64;
    // Apply the linear to gamma correction
    let color = Color::new(
        linear_to_gamma(color.x),
        linear_to_gamma(color.y),
        linear_to_gamma(color.z),
    );

    let ir = (255.0 * color.x.clamp(0.0, 1.0)) as i32;
    let ig = (255.0 * color.y.clamp(0.0, 1.0)) as i32;
    let ib = (255.0 * color.z.clamp(0.0, 1.0)) as i32;

    println!("{ir} {ig} {ib}")
}
