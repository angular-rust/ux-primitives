use crate::normalize_hue;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HsvColor {
    pub hue: f64,        // hue
    pub saturation: f64, // saturation
    pub value: f64,      // value
}

impl HsvColor {
    pub fn new(h: f64, s: f64, v: f64) -> Self {
        Self {
            hue: normalize_hue(h),
            saturation: if s > 100. {
                100.
            } else if s < 0. {
                0.
            } else {
                s
            },
            value: if s > 100. {
                100.
            } else if s < 0. {
                0.
            } else {
                v
            },
        }
    }
}

impl fmt::Display for HsvColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsv({}, {}, {})", self.hue, self.saturation, self.value)
    }
}
