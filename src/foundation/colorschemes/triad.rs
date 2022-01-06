#![allow(unused_imports)]
use crate::foundation::colorspace::{Color, HsvColor};

use super::ryb_rotate;

/// Triad colors are three hues equidistant on the color wheel. 
///
/// When you want a design that is colorful and yet balanced, a triad color scheme might be the way to go.
#[derive(Debug, Clone)]
pub struct Triad {
    angle: f32,

    colors: Vec<Color>,
    primary_color: Color,
}

impl Triad {
    /// Generate Triad scheme with your color and 120 degree angle
    pub fn new(primary: Color) -> Self {
        Self::with_parameters(primary, Some(120.0))
    }

    pub fn with_parameters(primary: Color, angle: Option<f32>) -> Self {
        let mut instance = Self {
            colors: Vec::new(),
            primary_color: primary,
            angle: angle.unwrap_or(120.0),
        };
        instance.generate();

        instance
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn set_angle(&mut self, value: f32) {
        self.angle = value;
        self.generate();
    }

    fn generate(&mut self) {
        self.colors = vec![self.primary_color];

        // value is brightness
        let mut c1: HsvColor = ryb_rotate(self.primary_color, self.angle);

        c1.value += 10.0;
        self.colors.push(c1.into());

        let mut c2: HsvColor = ryb_rotate(self.primary_color, -self.angle);

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
