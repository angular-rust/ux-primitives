use super::*;

// XYZ -> RGB
impl From<XyzColor> for RgbColor {
    fn from(xyz: XyzColor) -> RgbColor  {
        match Result::<RgbColor, ColorError>::from(xyz) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting XyzColor to RgbColor failed: {}", err)
        }
    }
}
impl From<XyzColor> for Result<RgbColor, ColorError> {
    fn from(_: XyzColor) -> Result<RgbColor, ColorError> {
        // TODO: implement L*a*b -> RGB
        Err(ColorError::Unimplemented)
    }
}
