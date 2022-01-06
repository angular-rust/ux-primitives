#![allow(unused_imports)]
use crate::foundation::colorspace::{Color, HsvColor};

use super::ryb_rotate;

/// Split complementary is a color and the analogous colors to its complement color. 
///
/// Using split complementary colors can give you a design with a high degree of contrast, 
/// yet still not as extreme as a real complementary color. 
/// It also results in greater harmony than the use of the direct complementary.
#[derive(Debug, Clone)]
pub struct SplitComplementary {
    colors: Vec<Color>,
    primary_color: Color,
}

impl SplitComplementary {
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

        // value is brightness
        let mut c1: HsvColor = ryb_rotate(self.primary_color, 150.0);
        c1.value += 10.0;
        self.colors.push(c1.into());

        let mut c2: HsvColor = ryb_rotate(self.primary_color, 210.0);
        c2.value += 10.0;
        self.colors.push(c2.into());
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
