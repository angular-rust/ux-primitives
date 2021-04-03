use super::*;

// L*a*b -> RGB
impl From<LabColor> for RgbColor {
    fn from(_: LabColor) -> Self {
        // TODO: implement L*a*b -> RGB
        panic!("{}: L*a*b -> RGB", ColorError::Unimplemented)
    }
}
