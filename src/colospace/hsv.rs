use std::fmt;
use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HsvColor {
    pub h: f64, // hue
    pub s: f64, // saturation
    pub v: f64, // value
}

impl fmt::Display for HsvColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsv({}, {}, {})", self.h, self.s, self.v)
    }
}

impl ToHexString for HsvColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
    }
}
