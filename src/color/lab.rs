#![cfg(feature = "experimental")]

use std::fmt;
use super::{Rgb, ColorError};

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

// L*a*b -> RGB
impl From<LabColor> for Rgb {
    fn from(_: LabColor) -> Self {
        // TODO: implement L*a*b -> RGB
        unimplemented!("{}: L*a*b -> RGB", ColorError::Unimplemented)
    }
}

// RGB -> L*a*b
impl From<Rgb> for LabColor {
    fn from(_: Rgb) -> Self {
        unimplemented!("{}: RGB -> L*a*b", ColorError::Unimplemented)
    }
}
