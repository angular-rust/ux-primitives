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

impl CmyColor {
    pub fn new(cyan: f64, magenta: f64, yellow: f64) -> Self {
        Self { cyan, magenta, yellow }
    }
}

// CMY -> RGB
impl From<CmyColor> for Color {
    fn from(_: CmyColor) -> Self {
        // TODO: implement CMY -> RGB
        unimplemented!("{}: CMY -> RGB", ColorError::Unimplemented);
    }
}

// RGB -> CMY
impl From<Color> for CmyColor {
    fn from(_: Color) -> Self {
        // TODO: implement RGB -> CMY
        unimplemented!("{}: RGB -> CMY", ColorError::Unimplemented);
    }
}
