use crate::vec3::Vec3;

use std::io::Write;

pub type Color = Vec3;

impl Color {
    pub fn write(&self, w: &mut impl Write) {
        let r = (255.999 * self.x) as usize;
        let g = (255.999 * self.y) as usize;
        let b = (255.999 * self.z) as usize;

        writeln!(w, "{r} {g} {b}").unwrap();
    }
}
