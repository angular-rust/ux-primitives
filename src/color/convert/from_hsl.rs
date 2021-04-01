use super::*;

// HSL -> RGB
impl From<HslColor> for RgbColor {
    fn from(hsl: HslColor) -> RgbColor  {
        match Result::<RgbColor, ColorError>::from(hsl) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting HslColor to RgbColor failed: {}", err)
        }
    }
}
impl From<HslColor> for Result<RgbColor, ColorError> {
    fn from(hsl: HslColor) -> Result<RgbColor, ColorError> {
        let HslColor { h: hue, s: saturation, l: lightness} = hsl;
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
                return Err(ColorError::DegreeOverflow);
            }
        };
        Ok(RgbColor {
            r: ((r_prime + m) * 255.).round() as u8,
            g: ((g_prime + m) * 255.).round() as u8,
            b: ((b_prime + m) * 255.).round() as u8
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use math::round::stochastic;

    lazy_static! {
        static ref TEST_COLORS: Vec<(Color, HslColor)> = vec!(
            (palette::BLUE_5,   HslColor::new(207.0, 86.0, 57.0)), // rgb(51, 154, 240)
            (palette::CYAN_6,   HslColor::new(187.0, 80.0, 42.0)), // rgb(21, 170, 191)
            (palette::TEAL_5,   HslColor::new(162.0, 73.0, 46.0)), // rgb(32, 201, 151)
            (palette::GREEN_4,  HslColor::new(130.0, 61.0, 64.0)), // rgb(105, 219, 124)
            (palette::LIME_1,   HslColor::new(80.0, 83.0, 88.0)), // rgb(233, 250, 200)
            (palette::LIME_6,   HslColor::new(85.0, 74.0, 45.0)), // rgb(130, 201, 30)
            (palette::YELLOW_4, HslColor::new(47.0, 100.0, 62.0)), // rgb(255, 212, 59)
            (palette::YELLOW_8, HslColor::new(35.0, 100.0, 47.0)), // rgb(240, 140, 0)
            (palette::ORANGE_2, HslColor::new(33.0, 100.0, 83.0)), // rgb(255, 216, 168)
            (palette::ORANGE_6, HslColor::new(27.0, 98.0, 54.0)), // rgb(253, 126, 20)
        )
    }

    #[test]
    fn to_rgb() {
        for (color, hsl) in TEST_COLORS.iter() {
            let expected_rgb: RgbColor = (*color).into();
            let actual_rgb: RgbColor = (*hsl).into();
        }
    }
}
