use super::{Color, Float};
use std::fmt;

/// Cmyk color representation
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CmykColor {
    /// Cyan component
    pub cyan: Float,
    /// Magenta component
    pub magenta: Float,
    /// Yellow component
    pub yellow: Float,
    /// Key component
    pub key: Float,
}

impl CmykColor {
    /// Create new Cmyk color with parameters
    pub fn new(cyan: Float, magenta: Float, yellow: Float, key: Float) -> Self {
        Self {
            cyan,
            magenta,
            yellow,
            key,
        }
    }
}

impl fmt::Display for CmykColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cmyk({}%, {}%, {}%, {}%)",
            self.cyan, self.magenta, self.yellow, self.key
        )
    }
}

// CMYK -> RGB
impl From<CmykColor> for Color {
    fn from(cmyk: CmykColor) -> Self {
        let apply = |v| (1.0 as Float - v / 100.0) * (1.0 - cmyk.key / 100.0);
        Color {
            red: apply(cmyk.cyan),
            green: apply(cmyk.magenta),
            blue: apply(cmyk.yellow),
            alpha: 1.,
        }
    }
}

// RGB -> CMYK
impl From<Color> for CmykColor {
    fn from(rgb: Color) -> Self {
        let key = 1.
            - [rgb.red, rgb.green, rgb.blue]
                .iter()
                .cloned()
                .fold(Float::NAN, Float::max);

        let apply = |v: Float| (((1. - v - key) / (1. - key)) * 100.).round();
        CmykColor {
            cyan: apply(rgb.red),
            magenta: apply(rgb.green),
            yellow: apply(rgb.blue),
            key: key * 100.,
        }
    }
}

#[cfg(test)]
mod test {
    // use super::super::*;
    // use math::round::stochastic;
    //
    // lazy_static! {
    //     static ref TEST_DATA: Vec<(Color, &'static str, CmykColor)> = {
    //         vec!(
    //             (Color::RGB(56, 217, 169), "#38d9a9", CmykColor::new(74.0, 0.0, 22.0, 15.0)),
    //             (Color::RGB(178, 242, 187), "#b2f2bb", CmykColor::new(26.0, 0.0, 23.0, 5.0)),
    //             (Color::RGB(230, 252, 245), "#e6fcf5", CmykColor::new(9.0, 0.0, 3.0, 1.0)),
    //             (Color::RGB(18, 184, 134), "#12b886", CmykColor::new(90.0, 0.0, 27.0, 28.0)),
    //             //(Color::RGB(___), "#______", CmykColor::new(___), HslColor::new(___)),
    //         )
    //     };
    // }
    //
    // #[test]
    // fn from_rgb() {
    //     for (color, _, expected_cmyk) in TEST_DATA.iter() {
    //         let CmykColor {
    //             cyan: actual_cyan,
    //             magenta: actual_magenta,
    //             yellow: actual_yellow,
    //             key: actual_key,
    //         } = CmykColor::from(*color);
    //
    //         let CmykColor {
    //             cyan: expected_cyan,
    //             magenta: expected_magenta,
    //             yellow: expected_yellow,
    //             key: expected_key,
    //         } = *expected_cmyk;
    //
    //         assert_eq!(
    //             stochastic(actual_cyan, 0), expected_cyan,
    //             "wrong cyan in cmyk conversion from {}", color.to_hex_string()
    //         );
    //         assert_eq!(
    //             stochastic(actual_magenta, 0), expected_magenta,
    //             "wrong magenta in cmyk conversion from {}", color.to_hex_string()
    //         );
    //         assert_eq!(
    //             stochastic(actual_yellow, 0), expected_yellow,
    //             "wrong yellow in cmyk conversion from {}", color.to_hex_string()
    //         );
    //         assert_eq!(
    //             stochastic(actual_key, 0), expected_key,
    //             "wrong key in cmyk conversion from {}", color.to_hex_string()
    //         );
    //     }
    // }
}
