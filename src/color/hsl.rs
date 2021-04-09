use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HslColor {
    pub h: f64, // hue
    pub s: f64, // saturation
    pub l: f64, // lightness
}

impl HslColor {
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self {
            h: h % 360.0,
            s: if s > 100.0 { 100.0 } else { s },
            l: if s > 100.0 { 100.0 } else { l }
        }
    }
}

impl fmt::Display for HslColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsl({}°, {}%, {}%)", self.h, self.s, self.l)
    }
}
