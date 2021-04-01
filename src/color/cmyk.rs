use std::fmt;


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CmykColor {
    pub c: f64, // cyan
    pub m: f64, // magenta
    pub y: f64, // yellow
    pub k: f64, // key color
}

impl CmykColor {
    pub fn new(c: f64, m: f64, y: f64, k: f64) -> Self {
        Self { c, m, y, k }
    }
}

impl fmt::Display for CmykColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cmyk({}%, {}%, {}%, {}%)", self.c, self.m, self.y, self.k)
    }
}
