use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CmykColor {
    pub cyan: f64,    // cyan
    pub magenta: f64, // magenta
    pub yellow: f64,  // yellow
    pub key: f64,     // key color
}

impl CmykColor {
    pub fn new(c: f64, m: f64, y: f64, k: f64) -> Self {
        Self {
            cyan: c,
            magenta: m,
            yellow: y,
            key: k,
        }
    }
}

impl fmt::Display for CmykColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cmyk({}%, {}%, {}%, {}%)",
            self.cyan, self.magenta, self.yellow, self.key
        )
    }
}
