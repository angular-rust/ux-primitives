use super::*;

// L*a*b -> RGB
impl From<LabColor> for RgbColor {
    fn from(lab: LabColor) -> RgbColor {
        match Result::<RgbColor, ColorError>::from(lab) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting LabColor to RgbColor failed: {}", err),
        }
    }
}
impl From<LabColor> for Result<RgbColor, ColorError> {
    fn from(_: LabColor) -> Result<RgbColor, ColorError> {
        // TODO: implement L*a*b -> RGB
        Err(ColorError::Unimplemented)
    }
}
