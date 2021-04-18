use std::fmt;
use super::{Rgb};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.red, self.green, self.blue)
    }
}

// RGBu8 -> RGB
impl From<RgbColor> for Rgb {
    fn from(c: RgbColor) -> Self {
        Rgb {
            red: c.red as f64 / 255.0,
            green: c.green as f64 / 255.0,
            blue: c.blue as f64 / 255.0,
        }
    }
}

// RGB -> RGBu8
impl From<Rgb> for RgbColor {
    fn from(c: Rgb) -> Self {
        RgbColor {
            red: (c.red * 255.0).round() as u8,
            green: (c.green * 255.0).round() as u8,
            blue: (c.blue * 255.0).round() as u8,
        }
    }
}
