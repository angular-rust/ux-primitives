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
        write!(f, "rgba({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha)
    }
}

impl RgbaColor {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { red, green, blue, alpha }
    }
}

// RGBAu8 -> RGB
impl From<RgbaColor> for Rgb {
    fn from(rgba: RgbaColor) -> Self {
        Rgb {
            red: rgba.red as f64 / 255.0,
            green: rgba.green as f64 / 255.0,
            blue: rgba.blue  as f64 / 255.0
        }
    }
}

impl From<Rgb> for RgbaColor {
    fn from(rgb: Rgb) -> Self {
        RgbaColor {
            red: (rgb.red * 255.0).round() as u8,
            green: (rgb.green * 255.0).round() as u8,
            blue: (rgb.blue * 255.0).round() as u8,
            alpha: 255,
        }
    }
}
