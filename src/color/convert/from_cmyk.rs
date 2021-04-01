use super::*;

// CMYK -> RGB
impl From<CmykColor> for RgbColor {
    fn from(cmyk: CmykColor) -> RgbColor  {
        match Result::<RgbColor, ColorError>::from(cmyk) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting CmykColor to RgbColor failed: {}", err)
        }
    }
}
impl From<CmykColor> for Result<RgbColor, ColorError> {
    fn from(cmyk: CmykColor) -> Result<RgbColor, ColorError> {
        let apply = |v| {
            (255.0
                * (1.0f64 - v as f64 / 100.0)
                * (1.0 - cmyk.k as f64 / 100.0)
            ).round() as u8
        };
        Ok(RgbColor { r: apply(cmyk.c), g: apply(cmyk.m), b: apply(cmyk.y) })
    }
}
