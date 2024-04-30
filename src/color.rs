use crate::vec3::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write}; 
use crate::interval::Interval;

fn linear_to_gamma(linear_component: f64) -> f64 {
    return linear_component.sqrt();
}

pub fn write_color(file: &mut BufWriter<File>, pixel_color: Vec3, samples_per_pixel: i32) -> std::io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale; 
    g *= scale;
    b *= scale;

    // apply gamma correction
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // write the translated [0, 255] value of each color component
    let intensity = Interval::interval(0.000, 0.999);
    file.write_all(format!("{} {} {}\n",
                           (256.0 * intensity.clamp(r)) as i32,
                           (256.0 * intensity.clamp(g)) as i32,
                           (256.0 * intensity.clamp(b)) as i32).as_bytes())?;

    Ok(())
}
