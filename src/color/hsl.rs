use std::fmt;
use super::{Rgb, ColorError, utils};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HslColor {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}

impl HslColor {
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self {
            hue: h % 360.0,
            saturation: if s > 100.0 { 100.0 } else { s },
            lightness: if s > 100.0 { 100.0 } else { l }
        }
    }
}

impl fmt::Display for HslColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsl({}°, {}%, {}%)", self.hue, self.saturation, self.lightness)
    }
}

// HSL -> RGB
impl From<HslColor> for Rgb {
    fn from(hsl: HslColor) -> Self {
        let HslColor { hue, saturation, lightness} = hsl;
        let c = ( 1.   -   ( (2. * (lightness as f64 / 100.)) - 1. ).abs() )
            * (saturation as f64 / 100.);
        let x = c * (1. - ( (((hue as f64) / 60.) % 2.) - 1. ).abs());
        let m = (lightness as f64 / 100.) - (c / 2.);

        let (r_prime, g_prime, b_prime) = {
            if hue >= 0. && hue < 60. {
                (c, x, 0.)
            } else if hue >= 60. && hue < 120. {
                (x, c, 0.)
            } else if hue >= 120. && hue < 180. {
                (0., c, x)
            } else if hue >= 180. && hue < 240. {
                (0., x, c)
            } else if hue >= 240. && hue < 300. {
                (x, 0., c)
            } else if hue >= 300. && hue < 360. {
                (c, 0., x)
            } else {
                unreachable!("{}", ColorError::DegreeOverflow)
            }
        };
        Rgb {
            red: r_prime + m,
            green: g_prime + m,
            blue: b_prime + m
        }
    }
}

// RGB -> HSL
impl From<Rgb> for HslColor {
    fn from(rgb: Rgb) -> Self {
        let Rgb { red, green, blue } = rgb;

        let (c_min, c_max) = utils::min_max_tuple([red, green, blue].iter());
        let delta = c_max - c_min;

        let hue = if (delta - 0.) < f64::EPSILON {
            0.
        } else {
            match c_max {
                x if x == red => 60. * (((green - blue) / delta) % 6.),
                x if x == green => 60. * (((blue - red) / delta) + 2.),
                x if x == blue => 60. * (((red - green) / delta) + 4.),
                _ => unreachable!("Invalid hue calculation!"),
            }.round()
        };

        let lightness = (c_max + c_min) / 2.;

        let saturation = if (delta - 0.) < f64::EPSILON {
            0.
        } else {
            (delta / (1. - ((2. * lightness) - 1.)) * 100.).round()
        };

        HslColor { hue, saturation, lightness: (lightness * 100.).round() }
    }
}

//noinspection DuplicatedCode
#[cfg(test)]
mod test {
    //use super::*;
    use super::super::test_utils;

    #[test]
    fn to_rgb() {
        test_utils::test_to_rgb_conversion(test_utils::RGB_HSL.iter())
    }

    // TODO: This test fails. Need to fix algorithm
    // #[test]
    // fn from_rgb() {
    //     for (color, _, _, expected_hsl) in TEST_DATA.iter() {
    //         let HslColor {
    //             h: actual_hue,
    //             s: actual_saturation,
    //             l: actual_lightness,
    //         } = HslColor::from(*color);
    //         let HslColor {
    //             h: expected_hue,
    //             s: expected_saturation,
    //             l: expected_lightness,
    //         } = *expected_hsl;
    //
    //         assert_eq!(
    //             stochastic(actual_hue, 0), expected_hue,
    //             "wrong hue in hsl conversion from {}", color.to_hex_string()
    //         );
    //         assert_eq!(
    //             stochastic(actual_saturation, 0), expected_saturation,
    //             "wrong saturation in hsl conversion from {}", color.to_hex_string()
    //         );
    //         assert_eq!(
    //             stochastic(actual_lightness, 0), expected_lightness,
    //             "wrong lightness in hsl conversion from {}", color.to_hex_string()
    //         );
    //     }
    // }
}