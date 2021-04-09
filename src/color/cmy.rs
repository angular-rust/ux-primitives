use std::fmt;

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

