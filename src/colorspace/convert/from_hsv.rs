use super::*;

// HSV -> RGB
impl From<HsvColor> for RgbColor {
    fn from(hsv: HsvColor) -> Self {
        //Err(ColorError::Unimplemented)
        let HsvColor { h: arg_hue, s: arg_saturation, v: arg_value} = hsv;
        let hue = normalize_hue(arg_hue);
        let value = percentage_to_fraction(arg_value);
        let saturation = percentage_to_fraction(arg_saturation);

        // https://ru.wikipedia.org/wiki/HSV_(%D1%86%D0%B2%D0%B5%D1%82%D0%BE%D0%B2%D0%B0%D1%8F_%D0%BC%D0%BE%D0%B4%D0%B5%D0%BB%D1%8C)#HSV_%E2%86%92_RGB
        let min = (1. - saturation) * value;
        let a = (value - min) * ((hue % 60.) / 60.);

        let (red_c, green_c, blue_c) = if hue >= 0. && hue < 60. {
            (value, min + a, min)
        } else if hue >= 60. && hue < 120. {
            (value - a, value, min)
        } else if hue >= 120. && hue < 180. {
            (min, value, min + a)
        } else if hue >= 180. && hue < 240. {
            (min, value - a, value)
        } else if hue >= 240. && hue < 300. {
            (min + a, min, value)
        } else if hue >= 300. && hue < 360. {
            (value, min, value - a)
        } else {
            unreachable!("HSV -> RGB: {}", ColorError::DegreeOverflow);
        };
        RgbColor::new(
            (red_c * 255.) as u8,
            (green_c * 255.) as u8,
            (blue_c * 255.) as u8
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_rgb() {
        test_utils::test_to_rgb_conversion(test_utils::RGB_HSV.iter())
    }
}
