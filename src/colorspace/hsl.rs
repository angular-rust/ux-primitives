use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HslColor {
    pub hue: f64,        // hue
    pub saturation: f64, // saturation
    pub lightness: f64,  // lightness
}

impl HslColor {
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self {
            hue: h % 360.0,
            saturation: if s > 100.0 { 100.0 } else { s },
            lightness: if s > 100.0 { 100.0 } else { l },
        }
    }
}

impl fmt::Display for HslColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hsl({}°, {}%, {}%)",
            self.hue, self.saturation, self.lightness
        )
    }
}
