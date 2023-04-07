use crate::{vec3::Vec3, SAMPLES_PER_PIXEL};

pub type Color = Vec3;

impl Color {
    pub fn _write(&self) {
        // Divide the color by the number of samples.
        let rgb = *self * (1. / SAMPLES_PER_PIXEL as f32);

        // Translate to [0,255] value of each color component.
        let rgb = rgb.clamp(0., 0.999) * 256.0;
        let _g = rgb.y();
        let _r = rgb.x();
        let _b = rgb.z();

        //writeln!(w, "{r:.0} {g:.0} {b:.0}").unwrap();
    }
}
