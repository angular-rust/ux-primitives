use super::*;

pub trait WithAlpha<C> {
    fn new(color: C, alpha: f64) -> Self;
    fn color(self) -> C;
    fn set_color(self, color: C) -> Self;
    fn alpha(self) -> f64;
    fn set_alpha(self, alpha: f64) -> Self;
    fn split(self) -> (C, f64);
}

#[derive(Clone, Copy, Debug)]
pub struct Alpha<C> {
    color: C,
    alpha: f64
}

impl<C> Alpha<C> {
    pub fn new(color: C, alpha: f64) -> Self {
        Self { color, alpha }
    }
}


impl<C> WithAlpha<C> for Alpha<C> {
    fn new(color: C, alpha: f64) -> Self { Alpha { color, alpha } }
    fn color(self) -> C { self.color }
    fn set_color(mut self, color: C) -> Self { self.color = color; self }
    fn alpha(self) -> f64 { self.alpha }
    fn set_alpha(mut self, alpha: f64) -> Self { self.alpha = alpha; self }
    fn split(self) -> (C, f64) { (self.color, self.alpha) }
}

impl WithAlpha<RgbColor> for RgbaColor {
    fn new(color: RgbColor, alpha: f64) -> Self {
        let RgbColor { red, green, blue } = color;
        RgbaColor { red, green, blue, alpha: (alpha * 255.).round() as u8 }
    }
    fn color(self) -> RgbColor { self.into() }
    fn set_color(mut self, color: RgbColor) -> Self {
        self.red = color.red;
        self.green = color.green;
        self.blue = color.blue;
        self
    }
    fn alpha(self) -> f64 { (self.alpha as f64) / 255.0 }
    fn set_alpha(mut self, alpha: f64) -> Self {
        self.alpha = (alpha * 255.).round() as u8; self
    }
    fn split(self) -> (RgbColor, f64) { (self.color(), self.alpha()) }
}


#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn new_rgba_from_rgb_with_alpha() {
        let RgbaColor {red, green, blue, alpha} = WithAlpha::new(RgbColor::new(200, 200, 200), 0.50);
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

    #[test]
    fn lin_rgb_with_alpha() {
        let Alpha {color: Rgb {red, green, blue}, alpha }
            = Alpha::new(Rgb::new(0.5, 0.5, 0.5), 0.5);
        assert_eq!(red, 0.5);
        assert_eq!(green, 0.5);
        assert_eq!(blue, 0.5);
        assert_eq!(alpha, 0.5);
    }
}