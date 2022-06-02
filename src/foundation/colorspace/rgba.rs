use super::*;
use std::fmt;

/// Rgba color representation with u8 components
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RgbaColor {
    /// Red component
    pub red: u8,
    /// Green component
    pub green: u8,
    /// Blue component
    pub blue: u8,
    /// Alpha component
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
    /// Create new Rgba color with parameters
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

// RGBAu8 -> RGB
impl From<RgbaColor> for Color {
    fn from(rgba: RgbaColor) -> Self {
        Color {
            red: rgba.red as Float / 255.0,
            green: rgba.green as Float / 255.0,
            blue: rgba.blue as Float / 255.0,
            alpha: rgba.alpha as Float / 255.0,
        }
    }
}

impl From<Color> for RgbaColor {
    fn from(color: Color) -> Self {
        RgbaColor {
            red: (color.red * 255.0).round() as u8,
            green: (color.green * 255.0).round() as u8,
            blue: (color.blue * 255.0).round() as u8,
            alpha: (color.alpha * 255.0).round() as u8,
        }
    }
}
