use super::prelude::*;

use super::*;

/// Defines get alpha getters functionality
pub trait GetAlpha<C> {
    /// Retrieve the alpha component
    fn get_alpha(&self) -> Float;
    /// Retrieve the alpha component as opacity
    fn get_opacity(&self) -> Float;
    /// Retrieve the alpha component as transparency
    fn get_transparency(&self) -> Float;
}

/// Defines alpha setters functionality
pub trait SetAlpha<C> {
    /// Set the alpha component
    fn set_alpha(&mut self, alpha: Float) -> &Self;
    /// Set the alpha component as opacity
    fn set_opacity(&mut self, opacity: Float) -> &Self;
    /// Set the alpha component as transparency
    fn set_transparency(&mut self, transparency: Float) -> &Self;
}

/// Defines alpha functionality
pub trait HasAlpha<C>: GetAlpha<C> + SetAlpha<C> {
    /// Retrieve the color component
    fn get_color(&self) -> C;

    /// Set the color component
    fn set_color(&mut self, color: C) -> &Self;
    
    /// Retrieve the color and aplha components
    fn split(&self) -> (C, Float)
    where
        Self: Sized,
    {
        (self.get_color(), self.get_alpha())
    }
}

/// Defines alpha adjustment functionality
pub trait AdjustAlpha<C>: Clone + SetAlpha<C> {
    
    /// Set the alpha component
    fn alpha(&self, alpha: Float) -> Self {
        let mut color = self.clone();
        color.set_alpha(alpha);
        color
    }

    /// Set the opacity
    fn opacity(&self, opacity: Float) -> Self {
        let mut color = self.clone();
        color.set_opacity(opacity);
        color
    }

    /// Set the transparency
    fn transparency(&self, transparency: Float) -> Self {
        let mut color = self.clone();
        color.set_transparency(transparency);
        color
    }
}


/// Represents the color with aplha component
#[derive(Clone, Copy, Debug)]
pub struct Alpha<C: ColorSpace> {
    color: C,
    alpha: Float,
}

impl<C: ColorSpace> Alpha<C> {
    /// Create color with alpha
    pub fn new(color: C, alpha: Float) -> Self {
        Self { color, alpha }
    }
}

impl<C: ColorSpace> GetAlpha<C> for Alpha<C> {
    fn get_alpha(&self) -> Float {
        self.alpha
    }
    fn get_opacity(&self) -> Float {
        self.alpha * 100.
    }
    fn get_transparency(&self) -> Float {
        (1. - self.alpha) * 100.
    }
}

impl<C: ColorSpace> SetAlpha<C> for Alpha<C> {
    fn set_alpha(&mut self, alpha: Float) -> &Self {
        self.alpha = alpha;
        self
    }
    fn set_opacity(&mut self, opacity: Float) -> &Self {
        self.alpha = utils::clamp(opacity, 0., 100.) / 100.;
        self
    }
    fn set_transparency(&mut self, transparency: Float) -> &Self {
        self.alpha = 1. - utils::clamp(transparency, 0., 100.) / 100.;
        self
    }
}

impl<C: ColorSpace> HasAlpha<C> for Alpha<C> {
    fn get_color(&self) -> C {
        self.color
    }
    fn set_color(&mut self, color: C) -> &Self {
        self.color = color;
        self
    }
    fn split(&self) -> (C, Float) {
        (self.color, self.alpha)
    }
}

impl GetAlpha<Self> for Color {
    fn get_alpha(&self) -> Float {
        self.alpha
    }
    fn get_opacity(&self) -> Float {
        self.alpha * 100.
    }
    fn get_transparency(&self) -> Float {
        (1. - self.alpha) * 100.
    }
}

impl SetAlpha<Self> for Color {
    fn set_alpha(&mut self, alpha: Float) -> &Self {
        self.alpha = alpha;
        self
    }
    fn set_opacity(&mut self, opacity: Float) -> &Self {
        self.alpha = opacity / 100.;
        self
    }
    fn set_transparency(&mut self, transparency: Float) -> &Self {
        self.alpha = 1. - transparency / 100.;
        self
    }
}

impl HasAlpha<Self> for Color {
    fn get_color(&self) -> Self {
        *self
    }
    fn set_color(&mut self, color: Self) -> &Self {
        self.red = color.red;
        self.green = color.green;
        self.blue = color.blue;
        self
    }
    fn split(&self) -> (Self, Float) {
        (self.get_color(), self.get_alpha())
    }
}

impl GetAlpha<RgbColor> for RgbaColor {
    fn get_alpha(&self) -> Float {
        (self.alpha as Float) / 255.0
    }
    fn get_opacity(&self) -> Float {
        self.alpha as Float / 255. * 100.
    }
    fn get_transparency(&self) -> Float {
        (1. - self.alpha as Float / 255.) * 100.
    }
}

impl SetAlpha<RgbColor> for RgbaColor {
    fn set_alpha(&mut self, alpha: Float) -> &Self {
        self.alpha = (alpha * 255.).round() as u8;
        self
    }
    fn set_opacity(&mut self, opacity: Float) -> &Self {
        self.alpha = (opacity / 100.).round() as u8;
        self
    }
    fn set_transparency(&mut self, transparency: Float) -> &Self {
        self.alpha = (1. - transparency / 100.).round() as u8;
        self
    }
}

impl HasAlpha<RgbColor> for RgbaColor {
    fn get_color(&self) -> RgbColor {
        (*self).into()
    }
    fn set_color(&mut self, color: RgbColor) -> &Self {
        self.red = color.red;
        self.green = color.green;
        self.blue = color.blue;
        self
    }
    fn split(&self) -> (RgbColor, Float) {
        (self.get_color(), self.get_alpha())
    }
}

impl From<Alpha<RgbColor>> for Color {
    fn from(from_color_with_alpha: Alpha<RgbColor>) -> Self {
        let mut color: Self = from_color_with_alpha.get_color().into_color();
        *color.set_alpha(from_color_with_alpha.get_alpha())
    }
}

impl From<Alpha<RgbColor>> for RgbaColor {
    fn from(from_color_with_alpha: Alpha<RgbColor>) -> Self {
        let mut color: Self = from_color_with_alpha.get_color().into_color();
        *color.set_alpha(from_color_with_alpha.get_alpha())
    }
}

#[cfg(test)]
mod test {
    use super::super::prelude::*;
    
    use super::super::*;

    #[test]
    fn new_rgba_from_rgb_with_alpha() {
        let RgbaColor {
            red,
            green,
            blue,
            alpha,
        } = prelude::Alpha::new(RgbColor::new(200, 200, 200), 0.50).into_color();
        assert_eq!(red, 200);
        assert_eq!(green, 200);
        assert_eq!(blue, 200);
        assert_eq!(alpha, 128);
    }

    #[test]
    fn rgb_color_with_alpha() {
        let prelude::Alpha {
            color: RgbColor { red, green, blue },
            alpha,
        } = prelude::Alpha::new(RgbColor::new(200, 200, 200), 0.5);
        assert_eq!(red, 200);
        assert_eq!(green, 200);
        assert_eq!(blue, 200);
        assert_eq!(alpha, 0.5);
    }
}
