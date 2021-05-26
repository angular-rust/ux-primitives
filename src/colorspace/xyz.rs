#![cfg(feature = "experimental")]

use super::{Color, ColorError, Float};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct XyzColor {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl XyzColor {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
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
