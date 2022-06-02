#![allow(unused_imports)]
use crate::foundation::colorspace::{Color, HsvColor};

use super::ryb_rotate;

/// The analog colors are those colors which lie on either side of any given color. 
///
/// Often these are color schemes found in nature. 
/// An application that makes use of analogous colors usually feels harmonious. 
/// The secondary color, as described above, can often be an analogous color.
#[derive(Debug, Clone)]
pub struct Analogous {
    angle: f32,
    contrast: f32,

    colors: Vec<Color>,
    primary_color: Color,
}

impl Analogous {
    /// Generate Analogous scheme with your color, 10 degree angle and 25 percent of contrast
    pub fn new(primary: Color) -> Self {
        Self::with_parameters(primary, Some(10.0), Some(25.0))
    }

    /// Generate Analogous scheme with specified parameters
    pub fn with_parameters(primary: Color, angle: Option<f32>, contrast: Option<f32>) -> Self {
        let mut instance = Self {
            colors: Vec::new(),
            primary_color: primary,
            angle: angle.unwrap_or(10.0),
            contrast: contrast.unwrap_or(25.0),
        };
        instance.generate();

        instance
    }

    fn generate(&mut self) {
        self.colors = vec![self.primary_color];
        let primary_hsb: HsvColor = self.primary_color.into();

        const ARRAY: [[f32; 2]; 4] = [[1.0, 2.2], [2.0, 1.0], [-1.0, -0.5], [-2.0, 1.0]];

        for idx in 0..ARRAY.len() {
            let one = ARRAY[idx][0];
            let two = ARRAY[idx][1];

            // set hue
            let mut new_hsb: HsvColor = ryb_rotate(primary_hsb.into(), self.angle * one);

            // value is brightness
            let t: f32 = 0.44 - two * 0.1;
            if primary_hsb.value - self.contrast * two < t {
                new_hsb.value = t * 100.0;
            } else {
                new_hsb.value = primary_hsb.value - self.contrast * two;
            }

            // set saturation
            new_hsb.saturation -= 5.0;

            self.colors.push(new_hsb.into());
        }
    }

    /// Retrieve angle of scheme
    pub fn angle(&self) -> f32 {
        self.angle
    }

    /// Set the angle of scheme
    pub fn set_angle(&mut self, value: f32) {
        self.angle = value;
        self.generate();
    }

    /// Retrieve contrast of scheme
    pub fn contrast(&self) -> f32 {
        self.contrast
    }

    /// Set the contrast
    pub fn set_contrast(&mut self, value: f32) {
        self.contrast = value;
        self.generate();
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
    pub fn set_primary_color(&mut self, value: Color) {
        self.primary_color = value;
        self.generate();
    }
}
