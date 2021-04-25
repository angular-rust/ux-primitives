use super::{utils, Color, ColorError};
use std::fmt;

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
            lightness: if s > 100.0 { 100.0 } else { l },
        }
    }
}

impl fmt::Display for HslColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hsl({}°, {}%, {}%)",
            self.hue, self.saturation, self.lightness
        )
    }
}

// HSL -> RGB
impl From<HslColor> for Color {
    fn from(hsl: HslColor) -> Self {
        let HslColor {
            hue,
            saturation,
            lightness,
        } = hsl;
        let c = (1. - ((2. * (lightness as f64 / 100.)) - 1.).abs()) * (saturation as f64 / 100.);
        let x = c * (1. - ((((hue as f64) / 60.) % 2.) - 1.).abs());
        let m = (lightness as f64 / 100.) - (c / 2.);

        let (r_prime, g_prime, b_prime) = {
            if (0.0..60.0).contains(&hue) {
                (c, x, 0.)
            } else if (60.0..120.0).contains(&hue) {
                (x, c, 0.)
            } else if (120.0..180.0).contains(&hue) {
                (0., c, x)
            } else if (180.0..240.0).contains(&hue) {
                (0., x, c)
            } else if (240.0..300.0).contains(&hue) {
                (x, 0., c)
            } else if (300.0..360.0).contains(&hue) {
                (c, 0., x)
            } else {
                unreachable!("{}", ColorError::DegreeOverflow)
            }
        };
        Color {
            red: r_prime + m,
            green: g_prime + m,
            blue: b_prime + m,
            alpha: 1.,
        }
    }
}

// RGB -> HSL
impl From<Color> for HslColor {
    fn from(rgb: Color) -> Self {
        let Color {
            red,
            green,
            blue,
            alpha: _,
        } = rgb;

        let (c_min, c_max) = utils::min_max_tuple([red, green, blue].iter());
        let delta = c_max - c_min;

        let hue = if (delta - 0.).abs() < f64::EPSILON {
            0.
        } else {
            match c_max {
                x if (x - red).abs() < f64::MIN_POSITIVE => 60. * (((green - blue) / delta) % 6.),
                x if (x - green).abs() < f64::MIN_POSITIVE => 60. * (((blue - red) / delta) + 2.),
                x if (x - blue).abs() < f64::MIN_POSITIVE => 60. * (((red - green) / delta) + 4.),
                _ => unreachable!("Invalid hue calculation!"),
            }
        };

        let lightness = (c_max + c_min) / 2.;

        let saturation = if (delta - 0.).abs() < f64::EPSILON {
            0.
        } else {
            delta / (1. - ((2. * lightness) - 1.).abs()) * 100.
        };

        HslColor {
            hue,
            saturation,
            lightness: lightness * 100.,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::test_utils;
    use super::super::*;

    #[test]
    fn to_rgb() {
        test_utils::test_to_rgb_conversion(test_utils::RGB_HSL.iter())
    }

    #[test]
    fn rgb_to_hsv() {
        test_utils::test_conversion(test_utils::RGB_HSL.iter(), |actual_color, expected_hsl| {
            let actual_rgb: RgbColor = (*actual_color).into();
            let actual_hsl: HslColor = actual_rgb.into();
            let HslColor {
                hue: actual_h,
                saturation: actual_s,
                lightness: actual_l,
            } = actual_hsl;
            let (actual_h, actual_s, actual_l) =
                (actual_h.round(), actual_s.round(), actual_l.round());
            let HslColor {
                hue: expected_h,
                saturation: expected_s,
                lightness: expected_l,
            } = *expected_hsl;
            assert!(
                test_utils::diff_less_than_f64(actual_h, expected_h, 1.),
                "wrong hue: {} -> {} != {}",
                actual_rgb,
                actual_hsl,
                expected_hsl
            );
            assert!(
                test_utils::diff_less_than_f64(actual_s, expected_s, 1.),
                "wrong saturation: {} -> {} != {}",
                actual_rgb,
                actual_hsl,
                expected_hsl
            );
            assert!(
                test_utils::diff_less_than_f64(actual_l, expected_l, 1.),
                "wrong lightness: {} -> {} != {}",
                actual_rgb,
                actual_hsl,
                expected_hsl
            );
        })
    }
}
