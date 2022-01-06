#![allow(unused_imports)]
use crate::foundation::colorspace::{Color, HsvColor};

use super::ryb_rotate;

/// The complementary colors are the colors which are directly opposite from one another on the color wheel. 
///
/// Complementary colors are contrasting and stand out against each other. 
/// Often it is a good idea to use a complementary color as the highlight color, as described above.
#[derive(Debug, Clone)]
pub struct Complementary {
    colors: Vec<Color>,
    primary_color: Color,
}

impl Complementary {
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

        let mut contrasting: HsvColor = primary_hsb;

        // value is brightness
        if primary_hsb.value > 40.0 {
            contrasting.value = 10.0 + primary_hsb.value * 0.25;
        } else {
            contrasting.value = 100.0 - primary_hsb.value * 0.25;
        }
        self.colors.push(contrasting.into());

        // supporting
        let mut supporting: HsvColor = primary_hsb;

        supporting.value = 30.0 + primary_hsb.value;
        supporting.saturation = 10.0 + primary_hsb.saturation * 0.3;
        self.colors.push(supporting.into());

        // complement
        let complement: HsvColor = ryb_rotate(self.primary_color, 180.0);
        self.colors.push(complement.into());

        // contrasting complement
        let mut contrasting_complement: HsvColor = complement.clone();

        if complement.value > 30.0 {
            contrasting_complement.value = 10.0 + complement.value * 0.25;
        } else {
            contrasting_complement.value = 100.0 - complement.value * 0.25;
        }
        self.colors.push(contrasting_complement.into());

        // supporting complement
        let mut supporting_complement: HsvColor = complement;

        supporting_complement.value = 30.0 + complement.value;
        supporting_complement.saturation = 10.0 + complement.saturation * 0.3;
        self.colors.push(supporting_complement.into());
    }

    pub fn num_of_colors(&self) -> usize {
        self.colors.len()
    }

    pub fn get_color(&self, index: usize) -> Option<Color> {
        self.colors.get(index).copied()
    }

    pub fn primary_color(&self) -> Color {
        self.primary_color
    }

    pub fn set_primary_color(&mut self, val: Color) {
        self.primary_color = val;
        self.generate();
    }
}
