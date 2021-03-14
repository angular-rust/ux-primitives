use std::fmt;
use crate::{RgbColor, ColorError};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl fmt::Display for RgbaColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl RgbaColor {
    pub fn to_hex_string(&self) -> String {
        format!("#{:0>2x}{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b, self.a)
    }
}

impl From<RgbColor> for RgbaColor {
    fn from(rgb: RgbColor) -> RgbaColor {
        match Result::<RgbaColor, ColorError>::from(rgb) {
            Ok(rgba) => rgba,
            Err(err) => panic!("Converting RgbColor to RgbaColor failed: {}", err)
        }
    }
}
impl From<RgbColor> for Result<RgbaColor, ColorError> {
    fn from(rgb: RgbColor) -> Self {
        Ok(RgbaColor {r: rgb.r, g: rgb.g, b: rgb.b, a: 0xFF })
    }
}