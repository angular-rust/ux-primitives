use super::*;

// HSV -> RGB
impl From<HsvColor> for RgbColor {
    fn from(hsv: HsvColor) -> RgbColor  {
        match Result::<RgbColor, ColorError>::from(hsv) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting HsvColor to RgbColor failed: {}", err)
        }
    }
}
impl From<HsvColor> for Result<RgbColor, ColorError> {
    fn from(_hsv: HsvColor) -> Result<RgbColor, ColorError> {
        Err(ColorError::Unimplemented)
        // let HsvColor { h: arg_hue, s: arg_saturation, v: arg_value} = hsv;
        // let hue = arg_hue % 360.0;
        // let value = if arg_value > 100.0 { 1.0 } else { arg_value / 100.0 };
        // let saturation = if arg_saturation > 100.0 { 1.0 } else { arg_saturation / 100.0 };
        //
        // // color components [0.0 <=> 1.0] - with value = 100
        // let weak_cmp = (hue / 60.0);
        // let (red_c, green_c, blue_c) = if hue >= 0. && hue < 60. {
        //     (1.0, weak_cmp, 0.0)
        // } else if hue >= 60. && hue < 120. {
        //     (weak_cmp, 1.0, 0.0)
        // } else if hue >= 120. && hue < 180. {
        //     (0.0, 1.0, weak_cmp)
        // } else if hue >= 180. && hue < 240. {
        //     (0.0, weak_cmp, 1.0)
        // } else if hue >= 240. && hue < 300. {
        //     (weak_cmp, 0.0, 1.0)
        // } else if hue >= 300. && hue < 360. {
        //     (weak_cmp, 1.0, 0.0)
        // } else {
        //     panic!("{}", ColorError::DegreeOverflow);
        // };
        //
        // // color components with real value
        // let (red_v, green_v, blue_v) = (
        //     red_c * (value / 100.0),
        //     green_c * (value / 100.0),
        //     blue_c * (value / 100.0)
        // );
        // // largest color component
        // let max_c = 1.0 * (value / 100.0);
        // // apply saturation:
    }
}