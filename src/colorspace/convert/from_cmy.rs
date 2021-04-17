use super::*;

// CMY -> RGB
impl From<CmyColor> for RgbColor {
    fn from(cmy: CmyColor) -> RgbColor {
        match Result::<RgbColor, ColorError>::from(cmy) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting CmyColor to RgbColor failed: {}", err),
        }
    }
}
impl From<CmyColor> for Result<RgbColor, ColorError> {
    fn from(_: CmyColor) -> Result<RgbColor, ColorError> {
        // TODO: implement CMY -> RGB
        Err(ColorError::Unimplemented)
    }
}
