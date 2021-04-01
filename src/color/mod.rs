use std::fmt;

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

#[cfg(feature = "experimental")]
mod alpha;
#[cfg(feature = "experimental")]
pub use alpha::{Alpha, WithAlpha};

pub mod palette;
pub mod convert;
pub use convert::*;

#[cfg(feature = "color_from_css")]
pub mod css;

mod utils;
pub use utils::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    /// A representation of the RGB (Red, Green, Blue) color space.
    RGB(u8, u8, u8),
    /// A representation of the RGBA (Red, Green, Blue, Alpha) color space.
    RGBA(u8, u8, u8, u8), //TODO: #[deprecated] via Alpha and WithAlpha
    /// A representation of the HSL (Hue, Saturation, Value) color space.
    HSV(f64, f64, f64),
    /// A representation of the HSL (Hue, Saturation, Lightness) color space.
    HSL(f64, f64, f64),
    /// A representation of the CMYK (Cyan, Magenta, Yellow) color space.
    CMY(f64, f64, f64),
    /// A representation of the CMYK (Cyan, Magenta, Yellow, Key) color space.
    CMYK(f64, f64, f64, f64),

    /// representation of the XYZ color space\
    #[cfg(feature = "experimental")]
    XYZ(f64, f64, f64),
    /// A representation of the L*a*b color space
    #[cfg(feature = "experimental")]
    LAB(f64, f64, f64),
}

impl Default for Color {
    fn default() -> Self { palette::BLACK }
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
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => write!(f, "lab({}, {}, {})", l, a, b),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => write!(f, "xyz({}, {}, {})", x, y, z),
        }
    }
}



impl Color {

    #[deprecated]
    pub fn to_rgb(&self) -> Result<Color, ColorError> {
        match self {
            Color::RGB(_, _, _) => Ok(self.clone()),
            Color::RGBA(_, _, _, _) => Ok(Color::from(RgbColor::from(*self))),
            Color::HSV(_, _, _) => Err(ColorError::Unimplemented),
            Color::HSL(_, _, _) => Ok(Color::from(RgbColor::from(*self))),
            Color::CMY(_, _, _) => Err(ColorError::Unimplemented),
            Color::CMYK(_, _, _, _) => Ok(Color::from(RgbColor::from(*self))),
            #[cfg(feature = "experimental")]
            Color::LAB(_, _, _) => Err(ColorError::Unimplemented),
            #[cfg(feature = "experimental")]
            Color::XYZ(_, _, _) => Err(ColorError::Unimplemented),
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
            #[cfg(feature = "experimental")]
            Color::LAB(_, _, _) => Err(ColorError::Unimplemented),
            #[cfg(feature = "experimental")]
            Color::XYZ(_, _, _) => Err(ColorError::Unimplemented),
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
            #[cfg(feature = "experimental")]
            Color::LAB(_, _, _) => Err(ColorError::Unimplemented),
            //Color::LAB(_, _, _) => Ok(Color::from(HslColor::from(RgbColor::from(*self)))),
            #[cfg(feature = "experimental")]
            Color::XYZ(_, _, _) => Err(ColorError::Unimplemented),
        }
    }
}
