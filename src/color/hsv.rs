use std::fmt;
use crate::{normalize_hue, percentage_to_fraction};
use super::{Rgb, ColorError, utils};


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HsvColor {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}

impl HsvColor {
    pub fn new(hue: f64, saturation: f64, value: f64) -> Self {
        Self {
            hue: normalize_hue(hue),
            saturation: if saturation > 100. { 100. } else if saturation < 0. { 0. } else { saturation },
            value: if saturation > 100. { 100. } else if saturation < 0. { 0. } else { value }
        }
    }
}

impl fmt::Display for HsvColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsv({}, {}, {})", self.hue, self.saturation, self.value)
    }
}

// HSV -> RGB
impl From<HsvColor> for Rgb {
    fn from(hsv: HsvColor) -> Self {
        //Err(ColorError::Unimplemented)
        let hue = normalize_hue(hsv.hue);
        let value = percentage_to_fraction(hsv.value);
        let saturation = percentage_to_fraction(hsv.saturation);

        // https://ru.wikipedia.org/wiki/HSV_(%D1%86%D0%B2%D0%B5%D1%82%D0%BE%D0%B2%D0%B0%D1%8F_%D0%BC%D0%BE%D0%B4%D0%B5%D0%BB%D1%8C)#HSV_%E2%86%92_RGB
        let min = (1. - saturation) * value;
        let a = (value - min) * ((hue % 60.) / 60.);

        if hue >= 0. && hue < 60. {
            Rgb::new(value, min + a, min)
        } else if hue >= 60. && hue < 120. {
            Rgb::new(value - a, value, min)
        } else if hue >= 120. && hue < 180. {
            Rgb::new(min, value, min + a)
        } else if hue >= 180. && hue < 240. {
            Rgb::new(min, value - a, value)
        } else if hue >= 240. && hue < 300. {
            Rgb::new(min + a, min, value)
        } else if hue >= 300. && hue < 360. {
            Rgb::new(value, min, value - a)
        } else {
            unreachable!("HSV -> RGB: {}", ColorError::DegreeOverflow);
        }
    }
}

// RGB -> HSV
impl From<Rgb> for HsvColor {
    fn from(rgb: Rgb) -> Self {
        // https://ru.wikipedia.org/wiki/HSV_(%D1%86%D0%B2%D0%B5%D1%82%D0%BE%D0%B2%D0%B0%D1%8F_%D0%BC%D0%BE%D0%B4%D0%B5%D0%BB%D1%8C)#RGB_%E2%86%92_HSV
        let Rgb { red, green, blue } = rgb;
        let (min, max) = utils::min_max_tuple([red, green, blue].iter());
        let hue = if max == red {
            //normalize_hue(60. * (green - blue) / delta - 30.)
            if green >= blue {
                60. * (green as f64 - blue as f64) / (max - min) as f64
            } else {
                360. - (green as f64 - blue as f64) / (max - min) as f64 * 60.
            }
        } else if max == green {
            60. * (blue as f64 - red as f64) / (max - min) as f64 + 120.
        } else if max == blue {
            60. * (red as f64 - green as f64) / (max - min) as f64 + 240.
        } else {
            0.
        };
        let saturation = 1. - (if max < f64::EPSILON { 0f64 } else { min as f64 / max as f64 });
        let brightness = max as f64 / 255.;
        HsvColor::new(hue, saturation * 100., brightness * 100.)
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    use super::super::test_utils;

    #[test]
    fn hsv_to_rgb() {
        test_utils::test_to_rgb_conversion(test_utils::RGB_HSV.iter())
    }

    #[test]
    fn rgb_to_hsv() {
        test_utils::test_conversion(
            test_utils::RGB_HSV.iter(),
            |actual_color, expected_hsv| {
                let actual_rgb: RgbColor = (*actual_color).into();
                let actual_hsv: HsvColor = actual_rgb.into();
                let HsvColor { hue: actual_h, saturation: actual_s, value: actual_v } = actual_hsv;
                let (actual_h, actual_s, actual_v) = (actual_h.round() ,actual_s.round(), actual_v.round());
                let HsvColor { hue: expected_h, saturation: expected_s, value: expected_v } = *expected_hsv;
                assert!(test_utils::diff_less_than_f64(actual_h, expected_h, 2.), "{} -> {} != {}", actual_rgb, actual_hsv, expected_hsv);
                assert!(test_utils::diff_less_than_f64(actual_s, expected_s, 2.), "{} -> {} != {}", actual_rgb, actual_hsv, expected_hsv);
                assert!(test_utils::diff_less_than_f64(actual_v, expected_v, 2.), "{} -> {} != {}", actual_rgb, actual_hsv, expected_hsv);
            }
        )
    }
}

