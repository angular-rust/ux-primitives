use std::fmt;
use super::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl From<Color> for RgbColor {
    fn from(item: Color) -> RgbColor {
        if let Color::RGB(red, green, blue) = item.to_rgb().unwrap() {
            Self { red, green, blue }
        } else {
            unreachable!()
        }
    }
}
