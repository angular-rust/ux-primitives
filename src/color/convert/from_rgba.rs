use super::*;

// RGBA -> RGB
impl From<RgbaColor> for RgbColor {
    fn from(rgb: RgbaColor) -> RgbColor {
        RgbColor {r: rgb.r, g: rgb.g, b: rgb.b }
    }
}

// RGBA -> HSL
impl From<RgbaColor> for HslColor {
    fn from(color: RgbaColor) -> HslColor {
        HslColor::from(RgbColor::from(color))
    }
}
