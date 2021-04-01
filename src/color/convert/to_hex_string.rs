use super::*;

pub trait ToHexString {
    fn to_hex_string(&self) -> String;
}

impl ToHexString for Color {
    fn to_hex_string(&self) -> String {
        match self {
            Color::RGB(_, _, _) => RgbColor::from(*self).to_hex_string(),
            Color::RGBA(_, _, _, _) => RgbaColor::from(*self).to_hex_string(),
            Color::HSV(_, _, _) => RgbColor::from(*self).to_hex_string(),
            Color::HSL(_, _, _) => RgbColor::from(*self).to_hex_string(),
            Color::CMY(_, _, _) => RgbColor::from(*self).to_hex_string(),
            Color::CMYK(_, _, _, _) => RgbColor::from(*self).to_hex_string(),
            #[cfg(feature = "experimental")]
            Color::LAB(_, _, _) => RgbColor::from(*self).to_hex_string(),
            #[cfg(feature = "experimental")]
            Color::XYZ(_, _, _) => RgbColor::from(*self).to_hex_string(),
        }
    }
}

impl ToHexString for RgbColor {
    fn to_hex_string(&self) -> String {
        format!("#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
    }
}
impl ToHexString for RgbaColor {
    fn to_hex_string(&self) -> String {
        format!("#{:0>2x}{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b, self.a)
    }
}

impl ToHexString for HslColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
    }
}

impl ToHexString for CmykColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
    }
}

impl ToHexString for CmyColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
    }
}

#[cfg(feature = "experimental")]
impl ToHexString for LabColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
    }
}
#[cfg(feature = "experimental")]
impl ToHexString for XyzColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
    }
}
