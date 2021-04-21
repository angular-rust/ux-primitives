use std::fmt;
use num_traits::Float;

mod rgb;
pub use rgb::RgbColor;
mod rgba;
pub use rgba::RgbaColor;
mod cmy;
pub use cmy::CmyColor;
mod cmyk;
pub use cmyk::CmykColor;
mod hsl;
pub use hsl::HslColor;
mod hsv;
pub use hsv::HsvColor;

#[cfg(feature = "experimental")]
mod lab;
#[cfg(feature = "experimental")]
pub use lab::LabColor;
#[cfg(feature = "experimental")]
mod xyz;
#[cfg(feature = "experimental")]
pub use xyz::XyzColor;

pub mod alpha;
pub use alpha::*;

pub mod adjust;
pub use adjust::*;

pub mod mix;
pub use mix::*;

pub mod unicolor;
pub use unicolor::*;

pub mod convert;
pub use convert::*;

pub mod to_hex_string;
pub use to_hex_string::*;

pub mod palette;

#[cfg(feature = "color_from_css")]
pub mod css;

pub mod round;
pub use round::*;

mod utils;
pub(crate) use utils::*;

//#[cfg(test)]
mod test_utils;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rgb<T: Float = f64> {
    red: T,
    green: T,
    blue: T,
    alpha: T
}
// #[derive(Clone, Copy, PartialEq, Debug)]
// pub struct Rgb {
//     red: f64,
//     green: f64,
//     blue: f64,
//     alpha: f64
// }

impl Rgb {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Rgb { red, green, blue, alpha }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::new(
            red as f64 / 255.,
            green as f64 / 255.,
            blue as f64 / 255.,
            1.,
        )
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::new(
            red as f64 / 255.,
            green as f64 / 255.,
            blue as f64 / 255.,
            alpha as f64 / 255.,
        )
    }

    pub fn hsl(hue: f64, saturation: f64, lightness: f64) -> Self {
        Self::from_color(HslColor::new(hue, saturation, lightness))
    }

    pub fn hsla(hue: f64, saturation: f64, lightness: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(HslColor::new(hue, saturation, lightness));
        color.alpha = alpha;
        color
    }

    pub fn hsv(hue: f64, saturation: f64, value: f64) -> Self {
        Self::from_color(HslColor::new(hue, saturation, value))
    }

    pub fn hsva(hue: f64, saturation: f64, value: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(HslColor::new(hue, saturation, value));
        color.alpha = alpha;
        color
    }

    pub fn cmyk(cyan: f64, magenta: f64, yellow: f64, key: f64) -> Self {
        Self::from_color(CmykColor::new(cyan, magenta, yellow, key))
    }
    //noinspection SpellCheckingInspection
    pub fn cmyka(cyan: f64, magenta: f64, yellow: f64, key: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(CmykColor::new(cyan, magenta, yellow, key));
        color.alpha = alpha;
        color
    }

    pub fn cmy(cyan: f64, magenta: f64, yellow: f64) -> Self {
        Self::from_color(CmyColor::new(cyan, magenta, yellow))
    }
    //noinspection SpellCheckingInspection
    pub fn cmya(cyan: f64, magenta: f64, yellow: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(CmyColor::new(cyan, magenta, yellow));
        color.alpha = alpha;
        color
    }

    #[cfg(feature = "experimental")]
    pub fn xyz(x: f64, y: f64, z: f64) -> Self {
        Self::from_color(XyzColor::new(x, y, z))
    }
    //noinspection SpellCheckingInspection
    #[cfg(feature = "experimental")]
    pub fn xyza(x: f64, y: f64, z: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(XyzColor::new(x, y, z));
        color.alpha = alpha;
        color
    }

    #[cfg(feature = "experimental")]
    pub fn lab(l: f64, a: f64, b: f64) -> Self {
        Self::from_color(XyzColor::new(l, a, b))
    }
    //noinspection SpellCheckingInspection
    #[cfg(feature = "experimental")]
    pub fn laba(l: f64, a: f64, b: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(XyzColor::new(l, a, b));
        color.alpha = alpha;
        color
    }

    // EMULATE creation of unicolor::Color enum

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn RGB(red: u8, green: u8, blue: u8) -> Self {
        Self::rgb(red, green, blue)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn RGBA(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::rgba(red, green, blue, alpha)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn HSL(hue: f64, saturation: f64, lightness: f64) -> Self {
        Self::hsl(hue, saturation, lightness)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn HSV(hue: f64, saturation: f64, value: f64) -> Self {
        Self::hsv(hue, saturation, value)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn CMYK(cyan: f64, magenta: f64, yellow: f64, key: f64) -> Self {
        Self::cmyk(cyan, magenta, yellow, key)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn CMY(cyan: f64, magenta: f64, yellow: f64) -> Self {
        Self::cmy(cyan, magenta, yellow)
    }

    #[cfg(feature = "experimental")]
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn XYZ(x: f64, y: f64, z: f64) -> Self {
        Self::xyz(x, y, z)
    }

    #[cfg(feature = "experimental")]
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn LAB(l: f64, a: f64, b: f64) -> Self {
        Self::lab(l, a, b)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.red, self.green, self.blue)
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

