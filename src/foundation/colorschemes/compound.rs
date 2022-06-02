#![allow(unused_imports)]
use crate::foundation::colorspace::{Color, HsvColor};

use super::{ryb_rotate, wrap};

/// Compound schemes (aka Split Complementary) are almost the same as complementary schemes.
///
/// Instead of using colors that are opposites, it uses colors on both sides of the opposite hue
pub struct Compound {
    colors: Vec<Color>,
    primary_color: Color,
}

impl Compound {
    /// Generate Compound scheme with your color
    pub fn new(primary: Color) -> Self {
        let mut instance = Self {
            colors: Vec::new(),
            primary_color: primary,
        };

        instance.generate();

        instance
    }

    fn generate(&mut self) {
        self.colors = vec![self.primary_color];
        let primary_hsb: HsvColor = self.primary_color.into();

        // value is brightness
        let mut c1: HsvColor = ryb_rotate(self.primary_color, 30.0);
        c1.value = wrap(primary_hsb.value, 25.0, 60.0, 25.0);
        self.colors.push(c1.into());

        let mut c2: HsvColor = ryb_rotate(self.primary_color, 30.0);
        c2.value = wrap(primary_hsb.value, 40.0, 10.0, 40.0);
        c2.saturation = wrap(primary_hsb.saturation, 40.0, 20.0, 40.0);
        self.colors.push(c2.into());

        let mut c3: HsvColor = ryb_rotate(self.primary_color, 160.0);
        c3.value = primary_hsb.value.max(20.0);
        c3.saturation = wrap(primary_hsb.saturation, 25.0, 10.0, 25.0);
        self.colors.push(c3.into());

        let mut c4: HsvColor = ryb_rotate(self.primary_color, 150.0);
        c4.value = wrap(primary_hsb.value, 30.0, 60.0, 30.0);
        c4.saturation = wrap(primary_hsb.saturation, 10.0, 80.0, 10.0);
        self.colors.push(c4.into());

        let mut c5: HsvColor = ryb_rotate(self.primary_color, 150.0);
        c5.value = wrap(primary_hsb.value, 40.0, 20.0, 40.0);
        c5.saturation = wrap(primary_hsb.saturation, 10.0, 80.0, 10.0);
        self.colors.push(c5.into());
    }

    /// Retrieve count colors of scheme
    pub fn num_of_colors(&self) -> usize {
        self.colors.len()
    }

    /// Set color by index
    pub fn get_color(&self, index: usize) -> Option<Color> {
        self.colors.get(index).copied()
    }

    /// Retrieve primary color of scheme
    pub fn primary_color(&self) -> Color {
        self.primary_color
    }

    /// Set the primary color of scheme
    pub fn set_primary_color(&mut self, val: Color) {
        self.primary_color = val;
        self.generate();
    }
}
