use super::*;

// RGBA -> RGB
impl From<RgbaColor> for RgbColor {
    fn from(rgba: RgbaColor) -> Self {
        RgbColor {r: rgba.r, g: rgba.g, b: rgba.b }
    }
}
impl From<RgbaColor> for Result<RgbColor, ColorError> {
    fn from(rgb: RgbaColor) -> Self {
        Ok(RgbColor {r: rgb.r, g: rgb.g, b: rgb.b })
    }
}

// RGBA -> HSL
impl From<RgbaColor> for HslColor {
    fn from(color: RgbaColor) -> HslColor {
        HslColor::from(RgbColor::from(color))
    }
}
