#![cfg(feature = "experimental")]

use super::{Color, ColorError};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct XyzColor {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl XyzColor {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

// XYZ -> RGB
impl From<XyzColor> for Color {
    fn from(_: XyzColor) -> Self {
        // TODO: implement L*a*b -> RGB
        panic!("{}: XYZ -> RGB", ColorError::Unimplemented)
    }
}

// RGB -> XYZ
impl From<Color> for XyzColor {
    fn from(_: Color) -> Self {
        // TODO: implement RGB -> XYZ
        panic!("{}: RGB -> XYZ", ColorError::Unimplemented)
    }
}
