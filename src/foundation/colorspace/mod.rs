//! Color spaces with conversion from/to any color space
//!
//! Supports the following color spaces:
//!
//! * `CMYK` 4-channel [CMYK](http://en.wikipedia.org/wiki/CMYK_color_model) color space.
//! * `HSV` (AKA HSB) 3-channel [HSB](http://en.wikipedia.org/wiki/HSL_and_HSV) color space.
//! * `HSL` 3-channel [HSL](http://en.wikipedia.org/wiki/HSL_and_HSV) color space.
//! * `Lab` 3-channel [Lab](http://en.wikipedia.org/wiki/Lab_color_space) color space. (experimantal)
//! * `RGB` Normal 3-channel [RGB](http://en.wikipedia.org/wiki/RGB_color_space) color space.
//! * `RGBA` 4-channel RGBA color space. It is a sub-struct of `RGB` with an additional `alpha` value.
//! * `XYZ` 3-channel [XYZ](http://en.wikipedia.org/wiki/CIE_1931_color_space) color space. (experimantal)
//! * `Cmy`
//!

// ## Todo
//
// * `Gray` A single channel gray-scale color. Any color given will be converted to gray-scale.
// * `Hex` A simple wrapper of color value in hex(eg. `0xFFFFCCCC`).
// * `YUV` 3-channel [YUV](http://en.wikipedia.org/wiki/YUV) color space.
// * `LCh` (AKA HCL)
// * `Luv` (or LUV)
// * `CubeHelix`
// * `HunterLab`
// * `Rgbx(a)` (an high resolution version of RGB)
// * `Temperature`
// * `Yxy`
//

mod base;
mod cmy;
mod cmyk;
mod hsl;
mod hsv;
#[cfg(feature = "experimental")]
mod lab;
mod rgb;
mod rgba;
#[cfg(feature = "experimental")]
mod xyz;

mod adjust;
mod alpha;
mod marker;
mod mix;
mod convert;
mod round;
mod to_hex_string;

pub(crate) type Float = f32;

#[cfg(any(feature = "color_from_css", test))]
pub mod css;

mod utils;
pub(crate) use utils::*;

#[cfg(test)]
mod test_utils;

pub use base::{Color, ColorError};
pub use cmy::CmyColor;
pub use cmyk::CmykColor;
pub use hsl::HslColor;
pub use hsv::HsvColor;
#[cfg(feature = "experimental")]
pub use lab::LabColor;
pub use rgb::RgbColor;
pub use rgba::RgbaColor;
#[cfg(feature = "experimental")]
pub use xyz::XyzColor;

/// Module with most usable functionality
pub mod prelude {
    pub use super::alpha::*;
    pub use super::adjust::*;
    pub use super::marker::*;
    pub use super::mix::*;
    pub use super::convert::*;
    pub use super::round::*;
    pub use super::to_hex_string::*;
}