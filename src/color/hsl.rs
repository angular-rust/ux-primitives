use std::fmt;
use super::ColorError;
use crate::{RgbaColor, RgbColor};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HslColor {
    pub h: f64, // hue
    pub s: f64, // saturation
    pub l: f64, // lightness
}

impl fmt::Display for HslColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsl({}°, {}%, {}%)", self.h, self.s, self.l)
    }
}

impl From<RgbaColor> for HslColor {
    fn from(color: RgbaColor) -> HslColor {
        HslColor::from(RgbColor::from(color))
    }
}

impl From<RgbColor> for HslColor {
    fn from(rgb: RgbColor) -> HslColor {
        match Result::<HslColor, ColorError>::from(rgb) {
            Ok(cmyk) => cmyk,
            Err(err) => panic!("Converting RgbColor to HslColor failed: {}", err)
        }
    }
}
impl From<RgbColor> for Result<HslColor, ColorError> {
    fn from(rgb: RgbColor) -> Self {
        let RgbColor { r: red, g: green, b: blue} = rgb;
        let r_prime = red as f64 / 255.;
        let g_prime = green as f64 / 255.;
        let b_prime = blue as f64 / 255.;

        let c_max = [red, green, blue].iter().max().cloned().unwrap() as f64 / 255.;
        let c_min = [red, green, blue].iter().min().cloned().unwrap() as f64 / 255.;

        let delta = c_max - c_min;

        let hue = if (delta - 0.) < f64::EPSILON {
            0.
        } else {
            match c_max {
                x if x == r_prime => 60. * (((g_prime - b_prime) / delta) % 6.),
                x if x == g_prime => 60. * (((b_prime - r_prime) / delta) + 2.),
                x if x == b_prime => 60. * (((r_prime - g_prime) / delta) + 4.),
                _ => unreachable!("Invalid hue calculation!"),
            }.round()
        };

        let lightness = (c_max + c_min) / 2.;

        let saturation = if (delta - 0.) < f64::EPSILON {
            0.
        } else {
            (delta / (1. - ((2. * lightness) - 1.)) * 100.).round()
        };

        Ok(HslColor { h: hue, s: saturation, l: (lightness * 100.).round() })
    }
}
