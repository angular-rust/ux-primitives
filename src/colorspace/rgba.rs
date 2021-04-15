use super::*;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RgbaColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl fmt::Display for RgbaColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

impl RgbaColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        }
    }
}

impl From<RgbColor> for RgbaColor {
    fn from(rgb: RgbColor) -> RgbaColor {
        match Result::<RgbaColor, ColorError>::from(rgb) {
            Ok(rgba) => rgba,
            Err(err) => panic!("Converting RgbColor to RgbaColor failed: {}", err),
        }
    }
}
impl From<RgbColor> for Result<RgbaColor, ColorError> {
    fn from(rgb: RgbColor) -> Self {
        Ok(RgbaColor {
            red: rgb.r,
            green: rgb.g,
            blue: rgb.b,
            alpha: 0xFF,
        })
    }
}
