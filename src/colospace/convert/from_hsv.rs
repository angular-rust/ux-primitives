use super::*;

// HSV -> RGB
impl From<HsvColor> for RgbColor {
    fn from(hsv: HsvColor) -> RgbColor {
        match Result::<RgbColor, ColorError>::from(hsv) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting HsvColor to RgbColor failed: {}", err),
        }
    }
}
impl From<HsvColor> for Result<RgbColor, ColorError> {
    fn from(hsv: HsvColor) -> Result<RgbColor, ColorError> {
        //Err(ColorError::Unimplemented)
        let HsvColor {
            hue: arg_hue,
            saturation: arg_saturation,
            value: arg_value,
        } = hsv;
        let hue = normalize_hue(arg_hue);
        let value = percentage_to_fraction(arg_value);
        let saturation = percentage_to_fraction(arg_saturation);

        // https://ru.wikipedia.org/wiki/HSV_(%D1%86%D0%B2%D0%B5%D1%82%D0%BE%D0%B2%D0%B0%D1%8F_%D0%BC%D0%BE%D0%B4%D0%B5%D0%BB%D1%8C)#HSV_%E2%86%92_RGB
        let min = (1. - saturation) * value;
        let a = (value - min) * ((hue % 60.) / 60.);

        let (red_c, green_c, blue_c) = if (0. ..60.).contains(&hue) {
            (value, min + a, min)
        } else if (60. ..120.).contains(&hue) {
            (value - a, value, min)
        } else if (120. ..180.).contains(&hue) {
            (min, value, min + a)
        } else if (180. ..240.).contains(&hue) {
            (min, value - a, value)
        } else if (240. ..300.).contains(&hue) {
            (min + a, min, value)
        } else if (300. ..360.).contains(&hue) {
            (value, min, value - a)
        } else {
            return Err(ColorError::DegreeOverflow); // unreachable
        };
        Ok(RgbColor::new(
            (red_c * 255.) as u8,
            (green_c * 255.) as u8,
            (blue_c * 255.) as u8,
        ))
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
