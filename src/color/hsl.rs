use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsl({}°, {}%, {}%)", self.hue, self.saturation, self.lightness)
    }
}
