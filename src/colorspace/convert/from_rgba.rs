use super::*;

// RGBA -> RGB
impl From<RgbaColor> for RgbColor {
    fn from(rgba: RgbaColor) -> Self {
        RgbColor {r: rgba.r, g: rgba.g, b: rgba.b }
    }
}

// RGBA -> HSL
impl From<RgbaColor> for HslColor {
    fn from(color: RgbaColor) -> HslColor {
        HslColor::from(RgbColor::from(color))
    }
}

// RGBA -> HSV
impl From<RgbaColor> for HsvColor {
    fn from(color: RgbaColor) -> HsvColor {
        HsvColor::from(RgbColor::from(color))
    }
}
