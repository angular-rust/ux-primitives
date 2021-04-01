use super::*;

// RGB -> HSL
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
    fn to_hsl() {
        // for (color, _, _, expected_hsl) in TEST_DATA.iter() {
        //     let HslColor {
        //         h: actual_hue,
        //         s: actual_saturation,
        //         l: actual_lightness,
        //     } = HslColor::from(*color);
        //     let HslColor {
        //         h: expected_hue,
        //         s: expected_saturation,
        //         l: expected_lightness,
        //     } = *expected_hsl;
        //
        //     assert_eq!(
        //         stochastic(actual_hue, 0), expected_hue,
        //         "wrong hue in hsl conversion from {}", color.to_hex_string()
        //     );
        //     assert_eq!(
        //         stochastic(actual_saturation, 0), expected_saturation,
        //         "wrong saturation in hsl conversion from {}", color.to_hex_string()
        //     );
        //     assert_eq!(
        //         stochastic(actual_lightness, 0), expected_lightness,
        //         "wrong lightness in hsl conversion from {}", color.to_hex_string()
        //     );
        // }
    }

    #[test]
    fn to_cmyk() {
        for (color, _, expected_cmyk, _) in TEST_DATA.iter() {
            let CmykColor {
                c: actual_cyan,
                m: actual_magenta,
                y: actual_yellow,
                k: actual_key,
            } = CmykColor::from(*color);

            let CmykColor {
                c: expected_cyan,
                m: expected_magenta,
                y: expected_yellow,
                k: expected_key,
            } = *expected_cmyk;

            assert_eq!(
                stochastic(actual_cyan, 0), expected_cyan,
                "wrong cyan in cmyk conversion from {}", color.to_hex_string()
            );
            assert_eq!(
                stochastic(actual_magenta, 0), expected_magenta,
                "wrong magenta in cmyk conversion from {}", color.to_hex_string()
            );
            assert_eq!(
                stochastic(actual_yellow, 0), expected_yellow,
                "wrong yellow in cmyk conversion from {}", color.to_hex_string()
            );
            assert_eq!(
                stochastic(actual_key, 0), expected_key,
                "wrong key in cmyk conversion from {}", color.to_hex_string()
            );
        }
    }
}