use super::{utils, Color, ColorError, Float};
use std::fmt;
use utils::{hue_bound, percentage_to_fraction};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HsvColor {
    pub hue: Float,
    pub saturation: Float,
    pub value: Float,
}

impl HsvColor {
    pub fn new(hue: Float, saturation: Float, value: Float) -> Self {
        Self {
            hue: hue_bound(hue),
            saturation: if saturation > 100. {
                100.
            } else if saturation < 0. {
                0.
            } else {
                saturation
            },
            value: if saturation > 100. {
                100.
            } else if saturation < 0. {
                0.
            } else {
                value
            },
        }
    }
}

impl fmt::Display for HsvColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsv({}, {}, {})", self.hue, self.saturation, self.value)
    }
}

// HSV -> RGB
impl From<HsvColor> for Color {
    fn from(hsv: HsvColor) -> Self {
        //Err(ColorError::Unimplemented)
        let hue = hue_bound(hsv.hue);
        let value = percentage_to_fraction(hsv.value);
        let saturation = percentage_to_fraction(hsv.saturation);

        // https://ru.wikipedia.org/wiki/HSV_(%D1%86%D0%B2%D0%B5%D1%82%D0%BE%D0%B2%D0%B0%D1%8F_%D0%BC%D0%BE%D0%B4%D0%B5%D0%BB%D1%8C)#HSV_%E2%86%92_RGB
        let min = (1. - saturation) * value;
        let a = (value - min) * ((hue % 60.) / 60.);

        if (0.0..60.0).contains(&hue) {
            Color::new(value, min + a, min, 1.)
        } else if (60.0..120.0).contains(&hue) {
            Color::new(value - a, value, min, 1.)
        } else if (120.0..180.0).contains(&hue) {
            Color::new(min, value, min + a, 1.)
        } else if (180.0..240.0).contains(&hue) {
            Color::new(min, value - a, value, 1.)
        } else if (240.0..300.0).contains(&hue) {
            Color::new(min + a, min, value, 1.)
        } else if (300.0..360.0).contains(&hue) {
            Color::new(value, min, value - a, 1.)
        } else {
            unreachable!("HSV -> RGB: {}", ColorError::DegreeOverflow);
        }
    }
}

// RGB -> HSV
impl From<Color> for HsvColor {
    fn from(rgb: Color) -> Self {
        // https://ru.wikipedia.org/wiki/HSV_(%D1%86%D0%B2%D0%B5%D1%82%D0%BE%D0%B2%D0%B0%D1%8F_%D0%BC%D0%BE%D0%B4%D0%B5%D0%BB%D1%8C)#RGB_%E2%86%92_HSV
        let Color {
            red,
            green,
            blue,
            alpha: _,
        } = rgb;
        let (min, max) = utils::min_max_tuple([red, green, blue].iter());
        let hue = if (max - red).abs() < Float::MIN_POSITIVE {
            //normalize_hue(60. * (green - blue) / delta - 30.)
            if green >= blue {
                60. * (green as Float - blue as Float) / (max - min) as Float
            } else {
                360. - (green as Float - blue as Float) / (max - min) as Float * 60.
            }
        } else if (max - green).abs() < Float::MIN_POSITIVE {
            60. * (blue as Float - red as Float) / (max - min) as Float + 120.
        } else if (max - blue).abs() < Float::MIN_POSITIVE {
            60. * (red as Float - green as Float) / (max - min) as Float + 240.
        } else {
            0.
        };
        let saturation = 1.
            - (if (max - 0.).abs() < Float::EPSILON {
                0 as Float
            } else {
                min as Float / max as Float
            });
        HsvColor::new(hue, saturation * 100., max * 100.)
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn hsv_to_rgb() {
        test_utils::test_to_rgb_conversion(test_utils::RGB_HSV.iter())
    }

    #[test]
    fn rgb_to_hsv() {
        test_utils::test_conversion(test_utils::RGB_HSV.iter(), |actual_color, expected_hsv| {
            let actual_rgb: RgbColor = (*actual_color).into();
            let actual_hsv: HsvColor = actual_rgb.into();
            let HsvColor {
                hue: actual_h,
                saturation: actual_s,
                value: actual_v,
            } = actual_hsv;
            let (actual_h, actual_s, actual_v) =
                (actual_h.round(), actual_s.round(), actual_v.round());
            let HsvColor {
                hue: expected_h,
                saturation: expected_s,
                value: expected_v,
            } = *expected_hsv;
            assert!(
                test_utils::diff_less_than_f64(actual_h, expected_h, 1.),
                "wrong hue: {} -> {} != {}",
                actual_rgb,
                actual_hsv,
                expected_hsv
            );
            assert!(
                test_utils::diff_less_than_f64(actual_s, expected_s, 1.),
                "wrong saturation: {} -> {} != {}",
                actual_rgb,
                actual_hsv,
                expected_hsv
            );
            assert!(
                test_utils::diff_less_than_f64(actual_v, expected_v, 1.),
                "wrong brightness: {} -> {} != {}",
                actual_rgb,
                actual_hsv,
                expected_hsv
            );
        })
    }
}
