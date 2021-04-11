use std::fmt;
use super::*;


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CmykColor {
    pub c: f64, // cyan
    pub m: f64, // magenta
    pub y: f64, // yellow
    pub k: f64, // key color
}

impl CmykColor {
    pub fn new(c: f64, m: f64, y: f64, k: f64) -> Self {
        Self { c, m, y, k }
    }
}

impl fmt::Display for CmykColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cmyk({}%, {}%, {}%, {}%)", self.c, self.m, self.y, self.k)
    }
}

// Color enum -> CmykColor
impl From<Color> for CmykColor {
    fn from(c: Color) -> CmykColor {
        match Result::<CmykColor, ColorError>::from(c) {
            Ok(cmyk) => cmyk,
            Err(err) => panic!("Converting Color to CmykColor failed: {}", err)
        }
    }
}
impl From<Color> for Result<CmykColor, ColorError> {
    fn from(c: Color) -> Result<CmykColor, ColorError> {
        match c {
            Color::RGB(r, g, b) => RgbColor { r, g, b }.into(),
            Color::RGBA(r, g, b, _) => RgbColor { r, g, b }.into(),
            Color::HSL(h, s, l) => RgbColor::from(HslColor{h, s, l}).into(),
            Color::HSV(h, s, v) => RgbColor::from(HsvColor{h, s, v}).into(),
            Color::CMYK(c, m, y, k) => Ok(CmykColor{c, m, y, k}),
            Color::CMY(c, m, y) => Ok(CmykColor{c, m, y, k: 0.0}),
            Color::LAB(l, a, b) => RgbColor::from(LabColor{l, a, b}).into(),
        }
    }
}

impl ToHexString for CmykColor {
    fn to_hex_string(&self) -> String {
        RgbColor::from(*self).to_hex_string()
    }
}

impl From<RgbColor> for CmykColor {
    fn from(rgb: RgbColor) -> CmykColor {
        match Result::<CmykColor, ColorError>::from(rgb) {
            Ok(cmyk) => cmyk,
            Err(err) => panic!("Converting RgbColor to CmykColor failed: {}", err)
        }
    }
}
impl From<RgbColor> for Result<CmykColor, ColorError> {
    fn from(rgb: RgbColor) -> Self {
        let r_prime = rgb.r as f64 / 255.;
        let g_prime = rgb.g as f64 / 255.;
        let b_prime = rgb.b as f64 / 255.;

        let key = 1.
            - [r_prime, g_prime, b_prime]
            .iter()
            .cloned()
            .fold(f64::NAN, f64::max);

        let apply = |v: f64| (((1. - v - key) / (1. - key)) * 100.).round();

        Ok(CmykColor {
            c: apply(r_prime),
            m: apply(g_prime),
            y: apply(b_prime),
            k: key * 100.
        })
    }
}
