use super::Color;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RgbaColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl fmt::Display for RgbaColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

impl From<Color> for RgbaColor {
    fn from(item: Color) -> RgbaColor {
        match item {
            Color::RGB(red, green, blue) => Self {
                red,
                green,
                blue,
                alpha: 255,
            },
            Color::RGBA(red, green, blue, alpha) => Self {
                red,
                green,
                blue,
                alpha,
            },
            _ => {
                if let Color::RGB(red, green, blue) = item.to_rgb().unwrap() {
                    Self {
                        red,
                        green,
                        blue,
                        alpha: 255,
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }
}
