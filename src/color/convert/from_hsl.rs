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


//noinspection DuplicatedCode
#[cfg(test)]
mod test {
    use super::*;
    //use math::round;

    #[test]
    fn to_rgb() {
        test_utils::test_to_rgb_conversion(test_utils::RGB_HSL.iter())
    }
}
