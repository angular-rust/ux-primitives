use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub cyan: f64,
    pub magenta: f64,
    pub yellow: f64,
    pub key: f64,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cmyk({}%, {}%, {}%, {}%)", self.cyan, self.magenta, self.yellow, self.key)
    }
}