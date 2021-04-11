use std::fmt;
use crate::normalize_hue;


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HsvColor {
    pub h: f64, // hue
    pub s: f64, // saturation
    pub v: f64, // value
}

impl HsvColor {
    pub fn new(h: f64, s: f64, v: f64) -> Self {
        Self {
            h: normalize_hue(h),
            s: if s > 100. { 100. } else if s < 0. { 0. } else { s },
            v: if s > 100. { 100. } else if s < 0. { 0. } else { v }
        }
    }
}

impl fmt::Display for HsvColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsv({}, {}, {})", self.h, self.s, self.v)
    }
}
