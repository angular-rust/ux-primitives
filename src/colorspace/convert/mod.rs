use super::*;

mod from_rgb;
pub use from_rgb::*;
mod from_rgba;
pub use from_rgba::*;
mod from_hsl;
pub use from_hsl::*;
mod from_hsv;
pub use from_hsv::*;
mod from_cmyk;
pub use from_cmyk::*;
mod from_cmy;
pub use from_cmy::*;

#[cfg(feature = "experimental")]
mod from_xyz;
#[cfg(feature = "experimental")]
pub use from_xyz::*;
#[cfg(feature = "experimental")]
mod from_lab;
#[cfg(feature = "experimental")]
pub use from_lab::*;

mod to_hex_string;
pub use to_hex_string::*;

mod from_color;
pub use from_color::*;

pub use to_color::*;
mod to_color {
    use super::*;
    use crate::ColorSpace;

    impl ColorSpace for RgbColor {}
    impl ColorSpace for RgbaColor {}
    impl ColorSpace for HslColor {}
    impl ColorSpace for HsvColor {}
    impl ColorSpace for CmykColor {}
    impl ColorSpace for CmyColor {}

    #[cfg(feature = "experimental")]
    impl ColorSpace for LabColor {}
    #[cfg(feature = "experimental")]
    impl ColorSpace for XyzColor {}

    impl From<RgbColor> for Color {
        fn from(c: RgbColor) -> Color {
            Color::RGB(c.red, c.green, c.blue)
        }
    }
    impl From<RgbaColor> for Color {
        fn from(c: RgbaColor) -> Color {
            Color::RGBA(c.red, c.green, c.blue, c.alpha)
        }
    }
    impl From<HslColor> for Color {
        fn from(c: HslColor) -> Color {
            Color::HSL(c.hue, c.saturation, c.lightness)
        }
    }
    impl From<HsvColor> for Color {
        fn from(c: HsvColor) -> Color {
            Color::HSV(c.hue, c.saturation, c.value)
        }
    }
    impl From<CmykColor> for Color {
        fn from(c: CmykColor) -> Color {
            Color::CMYK(c.cyan, c.magenta, c.yellow, c.key)
        }
    }
    impl From<CmyColor> for Color {
        fn from(c: CmyColor) -> Color {
            Color::CMY(c.cyan, c.magenta, c.yellow)
        }
    }

    #[cfg(feature = "experimental")]
    impl From<LabColor> for Color {
        fn from(c: LabColor) -> Color {
            Color::LAB(c.l, c.a, c.b)
        }
    }
    #[cfg(feature = "experimental")]
    impl From<XyzColor> for Color {
        fn from(c: XyzColor) -> Color {
            Color::XYZ(c.x, c.y, c.z)
        }
    }
}

#[cfg(test)]
mod test_utils;
