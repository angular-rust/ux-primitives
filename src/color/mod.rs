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
    blue: T
}

impl<T: Float> Rgb<T> {
    fn new(red: T, green: T, blue: T) -> Self {
        Rgb { red, green, blue }
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

