use std::fmt;

mod cmy;
pub use cmy::CmyColor;
mod cmyk;
pub use cmyk::CmykColor;
mod hsl;
pub use hsl::HslColor;
mod hsv;
pub use hsv::HsvColor;
mod lab;
pub use lab::LabColor;
mod rgb;
pub use rgb::RgbColor;
mod rgba;
pub use rgba::RgbaColor;

pub mod palette;

#[cfg(feature = "color_from_css")]
pub mod css;

mod utils;
pub use utils::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    /// A representation of the RGB (Red, Green, Blue) color space.
    RGB(u8, u8, u8),
    /// A representation of the RGBA (Red, Green, Blue, Alpha) color space.
    RGBA(u8, u8, u8, u8),
    /// A representation of the HSL (Hue, Saturation, Value) color space.
    HSV(f64, f64, f64),
    /// A representation of the HSL (Hue, Saturation, Lightness) color space.
    HSL(f64, f64, f64),
    /// A representation of the CMYK (Cyan, Magenta, Yellow) color space.
    CMY(f64, f64, f64),
    /// A representation of the CMYK (Cyan, Magenta, Yellow, Key) color space.
    CMYK(f64, f64, f64, f64),
    /// A representation of the L*a*b color space
    LAB(f64, f64, f64),
}

impl Default for Color {
    fn default() -> Self {
        palette::BLACK
    }
}

#[derive(Debug)]
pub enum ColorError {
    PercentageOverflow,
    DegreeOverflow,
    Unimplemented,
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PercentageOverflow => write!(f, "Overflow of Color percentage value (can't be greater than 100%)"),
            Self::DegreeOverflow => write!(f, "Overflow of Hue in hsl(v) color space (can't be greater than 360 deg"),
            Self::Unimplemented => write!(f, "Unimplemented color conversion"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::RGB(red, green, blue) => write!(f, "rgb({}, {}, {})", red, green, blue),
            Color::RGBA(red, green, blue, alpha) => {
                write!(f, "rgba({}, {}, {}, {})", red, green, blue, alpha)
            },
            Color::HSV(hue, saturation, value) => {
                write!(f, "hsv({}°, {}%, {}%)", hue, saturation, value)
            },
            Color::HSL(hue, saturation, lightness) => {
                write!(f, "hsl({}°, {}%, {}%)", hue, saturation, lightness)
            },
            Color::CMY(cyan, magenta, yellow) => {
                write!(f, "cmy({}%, {}%, {}%)", cyan, magenta, yellow)
            },
            Color::CMYK(cyan, magenta, yellow, key) => {
                write!(f, "cmyk({}%, {}%, {}%, {}%)", cyan, magenta, yellow, key)
            },
            Color::LAB(l, a, b) => write!(f, "lab({}, {}, {})", l, a, b)
        }
    }
}

impl From<RgbColor> for Color {
    fn from(c: RgbColor) -> Color {
        Color::RGB(c.r, c.g, c.b)
    }
}
impl From<RgbaColor> for Color {
    fn from(c: RgbaColor) -> Color {
        Color::RGBA(c.r, c.g, c.b, c.a)
    }
}
impl From<HslColor> for Color {
    fn from(c: HslColor) -> Color {
        Color::HSL(c.h, c.s, c.l)
    }
}
impl From<HsvColor> for Color {
    fn from(c: HsvColor) -> Color {
        Color::HSV(c.h, c.s, c.v)
    }
}
impl From<CmykColor> for Color {
    fn from(c: CmykColor) -> Color {
        Color::CMYK(c.c, c.m, c.y, c.k)
    }
}
impl From<CmyColor> for Color {
    fn from(c: CmyColor) -> Color {
        Color::CMY(c.c, c.m, c.y)
    }
}
impl From<LabColor> for Color {
    fn from(c: LabColor) -> Color {
        Color::LAB(c.l, c.a, c.b)
    }
}


impl Color {

    pub fn to_hex_string(&self) -> String {
        match self {
            Color::RGB(_, _, _) => RgbColor::from(*self).to_hex_string(),
            Color::RGBA(_, _, _, _) => RgbaColor::from(RgbColor::from(*self)).to_hex_string(),
            Color::HSV(_, _, _) => Color::from(RgbColor::from(*self)).to_hex_string(),
            Color::HSL(_, _, _) => RgbColor::from(*self).to_hex_string(),
            Color::CMY(_, _, _) => RgbColor::from(*self).to_hex_string(),
            Color::CMYK(_, _, _, _) => RgbColor::from(*self).to_hex_string(),
            Color::LAB(_, _, _) => RgbColor::from(*self).to_hex_string(),
        }
    }

    #[deprecated]
    pub fn to_rgb(&self) -> Result<Color, ColorError> {
        match self {
            Color::RGB(_, _, _) => Ok(self.clone()),
            Color::RGBA(_, _, _, _) => Ok(Color::from(RgbColor::from(*self))),
            Color::HSV(_, _, _) => Err(ColorError::Unimplemented),
            Color::HSL(_, _, _) => Ok(Color::from(RgbColor::from(*self))),
            Color::CMY(_, _, _) => Err(ColorError::Unimplemented),
            Color::CMYK(_, _, _, _) => Ok(Color::from(RgbColor::from(*self))),
            Color::LAB(_, _, _) => Err(ColorError::Unimplemented),
        }
    }

    #[deprecated]
    pub fn to_cmyk(&self) -> Result<Color, ColorError> {
        match self {
            Color::RGB(_, _, _) => Ok(Color::from(CmykColor::from(RgbColor::from(*self)))),
            Color::RGBA(_, _, _, _) => Ok(Color::from(CmykColor::from(RgbColor::from(*self)))),
            Color::HSV(_, _, _) => Ok(Color::from(CmykColor::from(RgbColor::from(*self)))),
            Color::HSL(_, _, _) => Ok(Color::from(CmykColor::from(RgbColor::from(*self)))),
            Color::CMY(_, _, _) => Ok(Color::from(CmykColor::from(RgbColor::from(*self)))),
            Color::CMYK(_, _, _, _) => Ok(self.clone()),
            Color::LAB(_, _, _) => Err(ColorError::Unimplemented),
        }
    }

    #[deprecated]
    pub fn to_hsl(&self) -> Result<Color, ColorError> {
        match self {
            Color::RGB(_, _, _) => Ok(Color::from(HslColor::from(RgbColor::from(*self)))),
            Color::RGBA(_, _, _, _) => Ok(Color::from(HslColor::from(RgbColor::from(*self)))),
            Color::HSV(_, _, _) => Ok(Color::from(HslColor::from(RgbColor::from(*self)))),
            Color::HSL(_, _, _) => Ok(self.clone()),
            Color::CMY(_, _, _) => Ok(Color::from(HslColor::from(RgbColor::from(*self)))),
            Color::CMYK(_, _, _, _) => Ok(Color::from(HslColor::from(RgbColor::from(*self)))),
            Color::LAB(_, _, _) => Ok(Color::from(HslColor::from(RgbColor::from(*self)))),
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn rgb_to_string() {
        // let rgb = Rgb::new(30, 50, 60);
        // assert_eq!(rgb.to_string(), String::from("rgb(30, 50, 60)"));
    }

    #[test]
    fn rgb_to_hex_string() {
        // let hex = Rgb::new(30, 50, 60).to_hex_string();
        // assert_eq!(hex, String::from("#1e323c"));
    }

    #[test]
    fn rgb_to_cmyk() {
        // let rgb = Rgb::new(30, 50, 60).to_cmyk();
        // assert_eq!(rgb, Cmyk::new_unchecked(50, 17, 0, 76));
    }

    #[test]
    fn rgb_to_hsl() {
        // let hsl = Rgb::new(204, 153, 102).to_hsl();
        // assert_eq!(hsl, Hsl::new_unchecked(30, 50, 60));
    }

    #[test]
    fn hsl_to_string() {
        // let hsl = Hsl::new_unchecked(100, 100, 100);
        // assert_eq!(hsl.to_string(), String::from("hsl(100°, 100%, 100%)"));
    }

    #[test]
    fn hsl_to_hex_string() {
        // let hex = Hsl::new_unchecked(30, 50, 60).to_hex_string();
        // assert_eq!(hex, String::from("#cc9966"));
    }

    #[test]
    fn hsl_to_rgb() {
        // let rgb = Hsl::new_unchecked(30, 50, 60).to_rgb();
        // assert_eq!(rgb, Rgb::new(204, 153, 102));
    }

    // #[should_panic]
    // #[test]
    // fn test_hsl_checked_hsl() {
    //     // Hsl::new(361, 101, 101).unwrap();
    // }

    #[test]
    fn cmyk_to_string() {
        // let cmyk = Cmyk::new(30, 50, 60, 40).unwrap();
        // assert_eq!(cmyk.to_string(), String::from("cmyk(30%, 50%, 60%, 40%)"));
    }

    #[test]
    fn cmyk_to_hex_string() {
        // let hex = Cmyk::new(30, 50, 60, 40).unwrap().to_hex_string();
        // assert_eq!(hex, String::from("#6b4d3d"));
    }

    #[test]
    fn cmyk_to_rgb() {
        // let hex = Cmyk::new(30, 50, 60, 40).unwrap().to_rgb();
        // assert_eq!(hex, Rgb::new(107, 77, 61));
    }

    // #[should_panic]
    // #[test]
    // fn cmyk_checked_cmyk() {
    //     // Cmyk::new(255, 255, 255, 255).unwrap();
    // }
}
