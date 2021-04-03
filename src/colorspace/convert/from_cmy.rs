use super::*;

// CMY -> RGB
impl From<CmyColor> for RgbColor {
    fn from(_: CmyColor) -> Self {
        // TODO: implement CMY -> RGB
        panic!("{}: CMY -> RGB", ColorError::Unimplemented);
    }
}
