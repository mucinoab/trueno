use crate::vec3::Vec3;

use std::io::Write;

pub type Color = Vec3;

impl Color {
    pub fn write(&self, w: &mut impl Write, samples_per_pixel: usize) {
        // Divide the color by the number of samples.
        let rgb = *self * (1. / samples_per_pixel as f32);

        // Translate to [0,255] value of each color component.
        let rgb = rgb.clamp(0., 0.999) * 256.0;
        let r = rgb.x() as u8;
        let g = rgb.y() as u8;
        let b = rgb.z() as u8;

        writeln!(w, "{r} {g} {b}").unwrap();
    }
}
