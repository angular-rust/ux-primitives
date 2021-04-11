use std::fmt;
use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CmyColor {
    pub c: f64, // cyan
    pub m: f64, // magenta
    pub y: f64, // yellow
}

impl fmt::Display for CmyColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cmy({}%, {}%, {}%)", self.c, self.m, self.y)
    }
}

impl ToHexString for CmyColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
    }
}
