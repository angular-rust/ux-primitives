use super::*;
use std::fmt;

/// Cmy color representation
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CmyColor {
    /// Cyan component
    pub cyan: Float,
    /// Magenta component
    pub magenta: Float,
    /// Yellow component
    pub yellow: Float,
}

impl fmt::Display for CmyColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cmy({}%, {}%, {}%)",
            self.cyan, self.magenta, self.yellow
        )
    }
}

impl CmyColor {
    /// Create new Cmy color with parameters
    pub fn new(cyan: Float, magenta: Float, yellow: Float) -> Self {
        Self {
            cyan,
            magenta,
            yellow,
        }
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
