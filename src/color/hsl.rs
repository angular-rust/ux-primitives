use std::fmt;
use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HslColor {
    pub h: f64, // hue
    pub s: f64, // saturation
    pub l: f64, // lightness
}

impl HslColor {
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self { h, s, l }
    }
}

impl fmt::Display for HslColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsl({}°, {}%, {}%)", self.h, self.s, self.l)
    }
}

// Color enum -> CmykColor
impl From<Color> for HslColor {
    fn from(c: Color) -> HslColor {
        match Result::<HslColor, ColorError>::from(c) {
            Ok(hsl) => hsl,
            Err(err) => panic!("Converting Color to CmykColor failed: {}", err)
        }
    }
}
impl From<Color> for Result<HslColor, ColorError> {
    fn from(c: Color) -> Result<HslColor, ColorError> {
        match c {
            Color::RGB(r, g, b) => RgbColor { r, g, b }.into(),
            Color::RGBA(r, g, b, _) => RgbColor { r, g, b }.into(),
            Color::HSL(h, s, l) => Ok(HslColor{h, s, l}),
            Color::HSV(h, s, v) => RgbColor::from(HsvColor{h, s, v}).into(),
            Color::CMYK(c, m, y, k) => RgbColor::from(CmykColor{c, m, y, k}).into(),
            Color::CMY(c, m, y) => RgbColor::from(CmykColor{c, m, y, k: 0.0}).into(),
            Color::LAB(l, a, b) => RgbColor::from(LabColor{l, a, b}).into(),
        }
    }
}

impl ToHexString for HslColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
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
    // FIXME: unit tests fail when calculating saturation
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
