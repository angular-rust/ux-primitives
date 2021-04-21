use super::*;

pub trait HasAlpha<C> {
    fn get_color(&self) -> C;
    fn set_color(&mut self, color: C) -> Self;
    fn get_alpha(&self) -> f64;
    fn set_alpha(&mut self, alpha: f64) -> Self;
    fn get_opacity(&self) -> f64;
    fn set_opacity(&mut self, opacity: f64) -> Self;
    fn get_transparency(&self) -> f64;
    fn set_transparency(&mut self, transparency: f64) -> Self;
    fn split(&self) -> (C, f64) where Self: Sized {
        (self.get_color(), self.get_alpha())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Alpha<C: ColorSpace> {
    color: C,
    alpha: f64
}

impl<C: ColorSpace> Alpha<C> {
    pub fn new(color: C, alpha: f64) -> Self {
        Self { color, alpha }
    }
}


impl<C: ColorSpace> HasAlpha<C> for Alpha<C> {
    fn get_color(&self) -> C { self.color }
    fn get_alpha(&self) -> f64 { self.alpha }
    fn set_color(&mut self, color: C) -> Self { self.color = color; *self }
    fn set_alpha(&mut self, alpha: f64) -> Self { self.alpha = alpha; *self }
    fn get_opacity(&self) -> f64 { self.alpha * 100. }
    fn set_opacity(&mut self, opacity: f64) -> Self { self.alpha = opacity / 100.; *self }
    fn get_transparency(&self) -> f64 { (1. - self.alpha) * 100. }
    fn set_transparency(&mut self, transparency: f64) -> Self { self.alpha = 1. - transparency / 100.; *self }
    fn split(&self) -> (C, f64) { (self.color, self.alpha) }
}

impl HasAlpha<Self> for Color {
    fn get_color(&self) -> Self { self.clone().into() }
    fn set_color(&mut self, color: Self) -> Self {
        self.red = color.red;
        self.green = color.green;
        self.blue = color.blue;
        *self
    }
    fn get_alpha(&self) -> f64 { self.alpha }
    fn set_alpha(&mut self, alpha: f64) -> Self {
        self.alpha = alpha;
        *self
    }
    fn get_opacity(&self) -> f64 { self.alpha as f64 / 255. * 100. }
    fn set_opacity(&mut self, opacity: f64) -> Self {
        self.alpha = opacity / 100.;
        *self
    }
    fn get_transparency(&self) -> f64 {
        (1. - self.alpha) * 100.
    }
    fn set_transparency(&mut self, transparency: f64) -> Self {
        self.alpha = 1. - transparency / 100.;
        *self
    }
    fn split(&self) -> (Self, f64) {
        (self.get_color(), self.get_alpha())
    }
}

impl HasAlpha<RgbColor> for RgbaColor {
    fn get_color(&self) -> RgbColor { self.clone().into() }
    fn set_color(&mut self, color: RgbColor) -> Self {
        self.red = color.red;
        self.green = color.green;
        self.blue = color.blue;
        *self
    }
    fn get_alpha(&self) -> f64 { (self.alpha as f64) / 255.0 }
    fn set_alpha(&mut self, alpha: f64) -> Self {
        self.alpha = (alpha * 255.).round() as u8;
        *self
    }
    fn get_opacity(&self) -> f64 { self.alpha as f64 / 255. * 100. }
    fn set_opacity(&mut self, opacity: f64) -> Self {
        self.alpha = (opacity / 100.).round() as u8;
        *self
    }
    fn get_transparency(&self) -> f64 {
        (1. - self.alpha as f64 / 255.) * 100.
    }
    fn set_transparency(&mut self, transparency: f64) -> Self {
        self.alpha = (1. - transparency / 100.).round() as u8;
        *self
    }
    fn split(&self) -> (RgbColor, f64) {
        (self.get_color(), self.get_alpha())
    }
}

impl From<Alpha<RgbColor>> for Color {
    fn from(from_color_with_alpha: Alpha<RgbColor>) -> Self {
        let mut color: Self = from_color_with_alpha.get_color().into_color();
        color.set_alpha(from_color_with_alpha.get_alpha())
    }
}

impl From<Alpha<RgbColor>> for RgbaColor {
    fn from(from_color_with_alpha: Alpha<RgbColor>) -> Self {
        let mut color: Self = from_color_with_alpha.get_color().into_color();
        color.set_alpha(from_color_with_alpha.get_alpha())
    }
}


#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn new_rgba_from_rgb_with_alpha() {
        let RgbaColor {red, green, blue, alpha} = Alpha::new(
            RgbColor::new(200, 200, 200),
            0.50
        ).into_color();
        assert_eq!(red, 200);
        assert_eq!(green, 200);
        assert_eq!(blue, 200);
        assert_eq!(alpha, 128);
    }

    #[test]
    fn rgb_color_with_alpha() {
        let Alpha{ color: RgbColor { red, green, blue }, alpha }
            = Alpha::new(RgbColor::new(200, 200, 200), 0.5);
        assert_eq!(red, 200);
        assert_eq!(green, 200);
        assert_eq!(blue, 200);
        assert_eq!(alpha, 0.5);
    }
}