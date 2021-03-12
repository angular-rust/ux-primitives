use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HsvColor {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}

impl fmt::Display for HsvColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsv({}, {}, {})", self.hue, self.saturation, self.value)
    }
}
