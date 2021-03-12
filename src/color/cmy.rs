use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CmyColor {
    pub cyan: f64,
    pub magenta: f64,
    pub yellow: f64,
}

impl fmt::Display for CmyColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cmy({}%, {}%, {}%)", self.cyan, self.magenta, self.yellow)
    }
}
