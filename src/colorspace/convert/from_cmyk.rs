use super::*;

// CMYK -> RGB
impl From<CmykColor> for RgbColor {
    fn from(cmyk: CmykColor) -> Self {
        let apply = |v| {
            (255.0
                * (1.0f64 - v as f64 / 100.0)
                * (1.0 - cmyk.k as f64 / 100.0)
            ).round() as u8
        };
        RgbColor { r: apply(cmyk.c), g: apply(cmyk.m), b: apply(cmyk.y) }
    }
}
