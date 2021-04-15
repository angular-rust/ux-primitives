use super::*;

// RGBA -> RGB
impl From<RgbaColor> for RgbColor {
    fn from(rgba: RgbaColor) -> Self {
        RgbColor {
            r: rgba.red,
            g: rgba.green,
            b: rgba.blue,
        }
    }
}
impl From<RgbaColor> for Result<RgbColor, ColorError> {
    fn from(rgb: RgbaColor) -> Self {
        Ok(RgbColor {
            r: rgb.red,
            g: rgb.green,
            b: rgb.blue,
        })
    }
}

// RGBA -> HSL
impl From<RgbaColor> for HslColor {
    fn from(color: RgbaColor) -> HslColor {
        HslColor::from(RgbColor::from(color))
    }
}
