use std::fmt;
use super::*;

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
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl ToHexString for RgbaColor {
    fn to_hex_string(&self) -> String {
        format!("#{:0>2x}{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b, self.a)
    }
}

// Color enum -> RgbaColor
impl From<Color> for RgbaColor {
    fn from(c: Color) -> RgbaColor {
        match Result::<RgbaColor, ColorError>::from(c) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting Color to RgbColor failed: {}", err)
        }
    }
}
impl From<Color> for Result<RgbaColor, ColorError> {
    fn from(c: Color) -> Result<RgbaColor, ColorError> {
        match c {
            Color::RGB(r, g, b) => Ok(RgbaColor { r, g, b, a: 0}),
            Color::RGBA(r, g, b, a) => Ok(RgbaColor { r, g, b, a }),
            Color::HSL(h, s, l) => RgbColor::from(HslColor{h, s, l}).into(),
            Color::HSV(h, s, v) => RgbColor::from(HsvColor{h, s, v}).into(),
            Color::CMYK(c, m, y, k) => RgbColor::from(CmykColor{c, m, y, k}).into(),
            Color::CMY(c, m, y) => RgbColor::from(CmyColor{c, m, y}).into(),
            Color::LAB(l, a, b) => RgbColor::from(LabColor{l, a, b}).into(),
        }
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