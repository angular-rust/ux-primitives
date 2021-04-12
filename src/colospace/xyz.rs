#![cfg(feature = "experimental")]

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
