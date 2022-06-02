#![allow(unused_imports)]
use crate::foundation::colorspace::{Color, HsvColor};

use super::{ryb_rotate, wrap};

/// A monotone color scheme is just one single hue and its variations in terms of tints, shades and saturation. 
///
/// Using saturation and tint/shade variations of a color is always good. 
/// However, in most cases I would advise against using a fully monochromatic scheme, as there is a risk of monotony. 
/// Using it with pure white or black can be efficient, though.
#[derive(Debug, Clone)]
pub struct Monochrome {
    colors: Vec<Color>,
    primary_color: Color,
}

impl Monochrome {
    /// Generate Monochrome scheme with your color
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
        let mut c1: HsvColor = primary_hsb;
        c1.value = wrap(primary_hsb.value, 50.0, 20.0, 30.0);
        c1.saturation = wrap(primary_hsb.saturation, 30.0, 10.0, 20.0);
        self.colors.push(c1.into());

        let mut c2: HsvColor = primary_hsb;
        c2.value = wrap(primary_hsb.value, 20.0, 20.0, 60.0);
        self.colors.push(c2.into());

        let mut c3: HsvColor = primary_hsb;
        c3.value = (primary_hsb.value + (100.0 - primary_hsb.value) * 0.2).max(20.0);
        c3.saturation = wrap(primary_hsb.saturation, 30.0, 10.0, 30.0);
        self.colors.push(c3.into());

        let mut c4: HsvColor = primary_hsb;
        c4.value = wrap(primary_hsb.value, 50.0, 20.0, 30.0);
        self.colors.push(c4.into());
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
