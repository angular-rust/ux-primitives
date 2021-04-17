#![allow(
    clippy::float_cmp,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms
)]

use super::*;

// RGB -> HSV
impl From<RgbColor> for HsvColor {
    fn from(rgb: RgbColor) -> HsvColor {
        match Result::<HsvColor, ColorError>::from(rgb) {
            Ok(hsv) => hsv,
            Err(err) => panic!("Converting RgbColor to HslColor failed: {}", err),
        }
    }
}
impl From<RgbColor> for Result<HsvColor, ColorError> {
    fn from(rgb: RgbColor) -> Self {
        // https://ru.wikipedia.org/wiki/HSV_(%D1%86%D0%B2%D0%B5%D1%82%D0%BE%D0%B2%D0%B0%D1%8F_%D0%BC%D0%BE%D0%B4%D0%B5%D0%BB%D1%8C)#RGB_%E2%86%92_HSV
        let RgbColor { red, green, blue } = rgb;
        let (min, max) =
            [red, green, blue]
                .iter()
                .fold((255u8, 0u8), |acc, color_cmp| -> (u8, u8) {
                    (
                        if *color_cmp < acc.0 {
                            *color_cmp
                        } else {
                            acc.0
                        },
                        if *color_cmp > acc.1 {
                            *color_cmp
                        } else {
                            acc.1
                        },
                    )
                });
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
        let saturation = 1.
            - (if max == 0 {
                0f64
            } else {
                min as f64 / max as f64
            });
        let brightness = max as f64 / 255.;
        Ok(HsvColor::new(hue, saturation * 100., brightness * 100.))
    }
}

// RGB -> HSL
impl From<RgbColor> for HslColor {
    fn from(rgb: RgbColor) -> HslColor {
        match Result::<HslColor, ColorError>::from(rgb) {
            Ok(hsl) => hsl,
            Err(err) => panic!("Converting RgbColor to HslColor failed: {}", err),
        }
    }
}
impl From<RgbColor> for Result<HslColor, ColorError> {
    // FIXME: unit tests fail when calculating saturation
    fn from(rgb: RgbColor) -> Self {
        let RgbColor { red, green, blue } = rgb;
        let r_prime = red as f64 / 255.;
        let g_prime = green as f64 / 255.;
        let b_prime = blue as f64 / 255.;

        let c_max = [red, green, blue].iter().max().cloned().unwrap() as f64 / 255.;
        let c_min = [red, green, blue].iter().min().cloned().unwrap() as f64 / 255.;

        let delta = c_max - c_min;

        let hue = if (delta - 0.).abs() < f64::EPSILON {
            0.
        } else {
            match c_max {
                x if x == r_prime => 60. * (((g_prime - b_prime) / delta) % 6.),
                x if x == g_prime => 60. * (((b_prime - r_prime) / delta) + 2.),
                x if x == b_prime => 60. * (((r_prime - g_prime) / delta) + 4.),
                _ => unreachable!("Invalid hue calculation!"),
            }
            .round()
        };

        let lightness = (c_max + c_min) / 2.;

        let saturation = if (delta - 0.).abs() < f64::EPSILON {
            0.
        } else {
            (delta / (1. - ((2. * lightness) - 1.)) * 100.).round()
        };

        Ok(HslColor {
            hue,
            saturation,
            lightness: (lightness * 100.).round(),
        })
    }
}

impl From<RgbColor> for CmykColor {
    fn from(rgb: RgbColor) -> CmykColor {
        match Result::<CmykColor, ColorError>::from(rgb) {
            Ok(cmyk) => cmyk,
            Err(err) => panic!("Converting RgbColor to CmykColor failed: {}", err),
        }
    }
}
impl From<RgbColor> for Result<CmykColor, ColorError> {
    fn from(rgb: RgbColor) -> Self {
        let r_prime = rgb.red as f64 / 255.;
        let g_prime = rgb.green as f64 / 255.;
        let b_prime = rgb.blue as f64 / 255.;

        let key = 1.
            - [r_prime, g_prime, b_prime]
                .iter()
                .cloned()
                .fold(f64::NAN, f64::max);

        let apply = |v: f64| (((1. - v - key) / (1. - key)) * 100.).round();

        Ok(CmykColor {
            cyan: apply(r_prime),
            magenta: apply(g_prime),
            yellow: apply(b_prime),
            key: key * 100.,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use math::round::stochastic;

    lazy_static! {
        static ref TEST_DATA: Vec<(Color, &'static str, CmykColor, HslColor)> = {
            vec!(
                (Color::RGB(56, 217, 169), "#38d9a9", CmykColor::new(74.0, 0.0, 22.0, 15.0), HslColor::new(162.0, 68.0, 54.0)),
                (Color::RGB(178, 242, 187), "#b2f2bb", CmykColor::new(26.0, 0.0, 23.0, 5.0), HslColor::new(128.0, 71.0, 82.0)),
                (Color::RGB(230, 252, 245), "#e6fcf5", CmykColor::new(9.0, 0.0, 3.0, 1.0), HslColor::new(161.0, 79.0, 95.0)),
                (Color::RGB(18, 184, 134), "#12b886", CmykColor::new(90.0, 0.0, 27.0, 28.0), HslColor::new(162.0, 82.0, 40.0)),
                //(Color::RGB(___), "#______", CmykColor::new(___), HslColor::new(___)),
            )
        };
    }

    #[test]
    fn to_hsv() {
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
                test_utils::diff_less_than_f64(actual_h, expected_h, 2.),
                "{} -> {} != {}",
                actual_rgb,
                actual_hsv,
                expected_hsv
            );
            assert!(
                test_utils::diff_less_than_f64(actual_s, expected_s, 2.),
                "{} -> {} != {}",
                actual_rgb,
                actual_hsv,
                expected_hsv
            );
            assert!(
                test_utils::diff_less_than_f64(actual_v, expected_v, 2.),
                "{} -> {} != {}",
                actual_rgb,
                actual_hsv,
                expected_hsv
            );
        })
    }

    // TODO: This test fails. Need to fix algorithm
    // #[test]
    // fn to_hsl() {
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

    #[test]
    fn to_cmyk() {
        for (color, _, expected_cmyk, _) in TEST_DATA.iter() {
            let CmykColor {
                cyan: actual_cyan,
                magenta: actual_magenta,
                yellow: actual_yellow,
                key: actual_key,
            } = CmykColor::from(*color);

            let CmykColor {
                cyan: expected_cyan,
                magenta: expected_magenta,
                yellow: expected_yellow,
                key: expected_key,
            } = *expected_cmyk;

            assert_eq!(
                stochastic(actual_cyan, 0),
                expected_cyan,
                "wrong cyan in cmyk conversion from {}",
                color.to_hex_string()
            );
            assert_eq!(
                stochastic(actual_magenta, 0),
                expected_magenta,
                "wrong magenta in cmyk conversion from {}",
                color.to_hex_string()
            );
            assert_eq!(
                stochastic(actual_yellow, 0),
                expected_yellow,
                "wrong yellow in cmyk conversion from {}",
                color.to_hex_string()
            );
            assert_eq!(
                stochastic(actual_key, 0),
                expected_key,
                "wrong key in cmyk conversion from {}",
                color.to_hex_string()
            );
        }
    }
}
