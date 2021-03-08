use std::fmt;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha)
    }
}
