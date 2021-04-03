use super::*;

// XYZ -> RGB
impl From<XyzColor> for RgbColor {
    fn from(_: XyzColor) -> Self {
        // TODO: implement L*a*b -> RGB
        panic!("{}: XYZ -> RGB", ColorError::Unimplemented)
    }
}
