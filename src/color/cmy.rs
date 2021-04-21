use std::fmt;
use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CmyColor {
    pub cyan: f64,
    pub magenta: f64,
    pub yellow: f64, // yellow
}

impl fmt::Display for CmyColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cmy({}%, {}%, {}%)", self.cyan, self.magenta, self.yellow)
    }
}

// CMY -> RGB
impl From<CmyColor> for Rgb {
    fn from(_: CmyColor) -> Self {
        // TODO: implement CMY -> RGB
        unimplemented!("{}: CMY -> RGB", ColorError::Unimplemented);
    }
}

// RGB -> CMY
impl From<Rgb> for CmyColor {
    fn from(_: Rgb) -> Self {
        // TODO: implement RGB -> CMY
        unimplemented!("{}: RGB -> CMY", ColorError::Unimplemented);
    }
}