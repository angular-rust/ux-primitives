#![allow(unused_imports)]
use crate::foundation::colorspace::{Color, HsvColor};

use super::ryb_rotate;

/// A tetradic color palette has four individual colors or hues if you will: a base color and three more colors.
///
/// All equidistant from the base color on the color wheel.
#[derive(Debug, Clone)]
pub struct Tetrad {
    angle: f32,
    alt: bool,

    colors: Vec<Color>,
    primary_color: Color,
}

impl Tetrad {
    /// Generate Analogous scheme with your color, 90 degree angle and without alt
    pub fn new(primary: Color) -> Self {
        Self::with_parameters(primary, Some(90.0), Some(false))
    }

    pub fn with_parameters(primary: Color, angle: Option<f32>, alt: Option<bool>) -> Self {
        let mut instance = Self {
            colors: Vec::new(),
            primary_color: primary,
            angle: angle.unwrap_or(90.0),
            alt: alt.unwrap_or_default(),
        };

        instance.generate();

        instance
    }

    fn generate(&mut self) {
        self.colors = vec![self.primary_color];
        let primary_hsb: HsvColor = self.primary_color.into();

        // value is brightness
        let mut c1: HsvColor = ryb_rotate(self.primary_color, self.angle);
        let mut multiplier: f32;
        if !self.alt {
            if primary_hsb.value < 50.0 {
                c1.value += 20.0;
            } else {
                c1.value -= 20.0;
            }
        } else {
            multiplier = (50.0 - primary_hsb.value) / 50.0;
            c1.value += (20.0 * multiplier).max(-20.0).min(20.0);
        }

        self.colors.push(c1.into());

        let mut c2: HsvColor = ryb_rotate(self.primary_color, self.angle * 2.0);
        if !self.alt {
            if primary_hsb.value > 50.0 {
                c2.value += 10.0;
            } else {
                c2.value -= 10.0;
            }
        } else {
            multiplier = (50.0 - primary_hsb.value) / 50.0;
            c2.value += (10.0 * multiplier).max(-10.0).min(10.0);
        }

        self.colors.push(c2.into());

        let mut c3: HsvColor = ryb_rotate(self.primary_color, self.angle * 3.0);
        c3.value += 10.0;
        self.colors.push(c3.into());
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn set_angle(&mut self, value: f32) {
        self.angle = value;
        self.generate();
    }

    pub fn alt(&self) -> bool {
        self.alt
    }

    pub fn set_alt(&mut self, val: bool) {
        self.alt = val;
        self.generate();
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
