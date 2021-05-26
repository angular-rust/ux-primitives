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

pub mod adjust;
pub mod alpha;
pub mod mix;
pub mod round;

pub mod convert;
pub mod marker;

pub mod to_hex_string;

pub(crate) type Float = f32;

#[cfg(any(feature = "color_from_css", test))]
pub mod css;

mod utils;
pub(crate) use utils::*;

#[cfg(test)]
mod test_utils;

pub use alpha::*;
pub use prelude::*;

pub mod prelude {
    use super::*;
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

    pub use adjust::*;
    pub use alpha::{Alpha as AlphaChannel, HasAlpha};
    pub use mix::*;
    pub use round::*;

    pub use convert::*;
    pub use marker::*;
    // #[deprecated]
    // pub use palette::*;
    pub use to_hex_string::*;
}
