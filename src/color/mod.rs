mod base;
mod rgb;
mod rgba;
mod cmy;
mod cmyk;
mod hsl;
mod hsv;
#[cfg(feature = "experimental")]
mod lab;
#[cfg(feature = "experimental")]
mod xyz;

pub mod alpha;
pub mod adjust;
pub mod mix;
pub mod round;

pub mod convert;
pub mod marker;

pub mod to_hex_string;
pub mod palette;

#[cfg(any(feature = "color_from_css", test))]
pub mod css;

mod utils;
pub(crate) use utils::*;

#[cfg(test)]
mod test_utils;

pub use prelude::*;
pub use alpha::*;

pub mod prelude {
    use super::*;
    pub use base::{Color, ColorError};
    pub use rgb::RgbColor;
    pub use rgba::RgbaColor;
    pub use cmy::CmyColor;
    pub use cmyk::CmykColor;
    pub use hsl::HslColor;
    pub use hsv::HsvColor;
    #[cfg(feature = "experimental")]
    pub use lab::LabColor;
    #[cfg(feature = "experimental")]
    pub use xyz::XyzColor;

    pub use alpha::{
        Alpha as AlphaChannel,
        HasAlpha
    };
    pub use adjust::*;
    pub use mix::*;
    pub use round::*;

    pub use convert::*;
    pub use marker::*;
    pub use to_hex_string::*;
    #[deprecated]
    pub use palette::*;
}
