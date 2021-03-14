use std::fmt;
use crate::{RgbColor, ColorError};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CmykColor {
    pub c: f64, // cyan
    pub m: f64, // magenta
    pub y: f64, // yellow
    pub k: f64, // key color
}

impl fmt::Display for CmykColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cmyk({}%, {}%, {}%, {}%)", self.c, self.m, self.y, self.k)
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
