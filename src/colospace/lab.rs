use std::fmt;
use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LabColor {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl fmt::Display for LabColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lab({}, {}, {})", self.l, self.a, self.b)
    }
}

impl ToHexString for LabColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
    }
}
