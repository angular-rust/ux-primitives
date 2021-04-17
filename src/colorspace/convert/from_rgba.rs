use super::*;

// RGBA -> RGB
impl From<RgbaColor> for RgbColor {
    fn from(rgba: RgbaColor) -> Self {
        RgbColor {
            red: rgba.red,
            green: rgba.green,
            blue: rgba.blue,
        }
    }
}
impl From<RgbaColor> for Result<RgbColor, ColorError> {
    fn from(rgb: RgbaColor) -> Self {
        Ok(RgbColor {
            red: rgb.red,
            green: rgb.green,
            blue: rgb.blue,
        })
    }
}

// RGBA -> HSL
impl From<RgbaColor> for HslColor {
    fn from(color: RgbaColor) -> HslColor {
        HslColor::from(RgbColor::from(color))
    }
}
