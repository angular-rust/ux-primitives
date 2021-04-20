use super::*;

pub trait GetHue {
    fn get_hue(self) -> f64;
}
pub trait SetHue {
    fn set_hue(&mut self, hue: f64) -> Self;
}
pub trait GetSaturation {
    fn get_saturation(self) -> f64;
}
pub trait SetSaturation {
    fn set_saturation(&mut self, saturation: f64) -> Self;
}

pub trait HasHue: Clone + Copy + GetHue + SetHue {}
pub trait HasSaturation: Clone + Copy + GetSaturation + SetSaturation {}

pub trait Lighten: Sized {
    fn lighten(self, delta: f64) -> Self;
    fn darken(self, delta: f64) -> Self {
        self.lighten(-delta)
    }
}

pub trait AdjustHue: Sized {
    fn adjust_hue(self, delta: f64) -> Self;
    fn complement(self) -> Self {
        self.adjust_hue(180.)
    }
}

pub trait Saturate: Sized {
    fn saturate(self, delta: f64) -> Self;
    fn desaturate(self, delta: f64) -> Self {
        self.saturate(-delta)
    }
}

pub trait Grayscale: Saturate {
    fn grayscale(self) -> Self { self.saturate(-100.) }
}

pub trait Invert: Sized {
    fn invert(self) -> Self;
}

pub trait Round {
    fn round(self) -> Self;
}

pub trait Adjust: Lighten + AdjustHue + Saturate + Grayscale {}

impl GetHue for HslColor {
    fn get_hue(self) -> f64 { self.hue }
}
impl SetHue for HslColor {
    fn set_hue(&mut self, hue: f64) -> Self { self.hue = hue_bound(hue); *self }
}
impl GetHue for HsvColor {
    fn get_hue(self) -> f64 { self.hue }
}
impl SetHue for HsvColor {
    fn set_hue(&mut self, hue: f64) -> Self {
        self.hue = hue_bound(hue);
        *self
    }
}
impl GetSaturation for HslColor {
    fn get_saturation(self) -> f64 { self.saturation }
}
impl SetSaturation for HslColor {
    fn set_saturation(&mut self, saturation: f64) -> Self { self.saturation = percentage_bound(saturation); *self }
}
impl GetSaturation for HsvColor {
    fn get_saturation(self) -> f64 { self.saturation }
}
impl SetSaturation for HsvColor {
    fn set_saturation(&mut self, saturation: f64) -> Self { self.saturation = percentage_bound(saturation); *self }
}
impl HasHue for HslColor {}
impl HasHue for HsvColor {}

impl<C: NonRadialSpace> GetHue for C {
    fn get_hue(self) -> f64 {
        let hsv: HsvColor = self.into_color();
        hsv.get_hue()
    }
}
impl<C: NonRadialSpace> SetHue for C {
    fn set_hue(&mut self, hue: f64) -> Self {
        let mut hsv: HsvColor = (*self).into_color();
        hsv.set_hue(hue);
        *self = hsv.into_color();
        *self
    }
}

impl<C: NonSaturationSpace> GetSaturation for C {
    fn get_saturation(self) -> f64 {
        let hsv: HsvColor = self.into_color();
        hsv.get_saturation()
    }
}
impl<C: NonSaturationSpace + FromColor<HsvColor> + IntoColor<HsvColor>> SetSaturation for C {
    fn set_saturation(&mut self, saturation: f64) -> Self {
        let mut hsv: HsvColor = (*self).into_color();
        hsv.set_saturation(saturation);
        *self = hsv.into_color();
        *self
    }
}


impl<C: FromColor<HslColor> + IntoColor<HslColor>> Lighten for C {
    fn lighten(self, delta: f64) -> Self {
        if (delta.abs() - 100.) > f64::EPSILON {
            panic!("lighten failed: {}. Actual delta is {}", ColorError::PercentageOverflow, delta)
        }
        let hsl: HslColor = self.into_color();
        C::from_color(HslColor {
            hue: hsl.hue,
            saturation: hsl.saturation,
            lightness: utils::percentage_bound(hsl.lightness + delta)
        })
    }
}

impl<C: Clone + GetHue + SetHue> AdjustHue for C {
    fn adjust_hue(self, delta: f64) -> Self {
        self.clone().set_hue(self.get_hue() + delta)
    }
}

impl<C: Clone + GetSaturation + SetSaturation> Saturate for C {
    fn saturate(self, delta: f64) -> Self {
        self.clone().set_saturation(self.get_saturation() + delta)
    }
}
impl<C: Clone + GetSaturation + SetSaturation> Grayscale for C {}

impl Invert for RgbColor {
    fn invert(self) -> Self {
        RgbColor {
            red: 255 - self.red,
            green: 255 - self.green,
            blue: 255 - self.blue
        }
    }
}
impl Invert for RgbaColor {
    fn invert(self) -> Self {
        RgbaColor {
            red: 255 - self.red,
            green: 255 - self.green,
            blue: 255 - self.blue,
            alpha: self.alpha,
        }
    }
}

impl<C: NonRgbSpace> Invert for C {
    fn invert(self) -> Self {
        let Rgb { red, green, blue } = self.into();
        C::from(Rgb {
            red: 255. - red,
            green: 255. - green,
            blue: 255. - blue,
        })
    }
}

impl Round for Rgb {
    fn round(self) -> Self {
        Rgb {
            red: self.red.round(),
            green: self.green.round(),
            blue: self.blue.round(),
        }
    }
}

impl Round for HslColor {
    fn round(self) -> Self {
        HslColor {
            hue: self.hue.round(),
            saturation: self.saturation.round(),
            lightness: self.lightness.round(),
        }
    }
}

impl Round for HsvColor {
    fn round(self) -> Self {
        HsvColor {
            hue: self.hue.round(),
            saturation: self.saturation.round(),
            value: self.value.round(),
        }
    }
}

impl Round for CmykColor {
    fn round(self) -> Self {
        CmykColor {
            cyan: self.cyan.round(),
            magenta: self.magenta.round(),
            yellow: self.yellow.round(),
            key: self.key.round(),
        }
    }
}

impl Round for CmyColor {
    fn round(self) -> Self {
        CmyColor {
            cyan: self.cyan.round(),
            magenta: self.magenta.round(),
            yellow: self.yellow.round(),
        }
    }
}

// Implementations for colors with alpha channel
impl<C: Lighten> Lighten for Alpha<C> {
    fn lighten(self, delta: f64) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.lighten(delta), alpha)
    }
}
impl<C: AdjustHue> AdjustHue for Alpha<C> {
    fn adjust_hue(self, delta: f64) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.adjust_hue(delta), alpha)
    }
}
impl<C: Saturate> Saturate for Alpha<C> {
    fn saturate(self, delta: f64) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.saturate(delta), alpha)
    }
}
impl<C: Grayscale> Grayscale for Alpha<C> {}
impl<C: Invert> Invert for Alpha<C> {
    fn invert(self) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.invert(), alpha)
    }
}
impl<C: Round> Round for Alpha<C> {
    fn round(self) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.round(), alpha)
    }
}


#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn adjust_hue_for_rgb() {
        let red_rgb = RgbColor::new(255, 0, 0);
        let green_rgb = red_rgb.adjust_hue(120.);
        assert_eq!(green_rgb.red, 0, "wrong red cmp of green color: {}", green_rgb);
        assert_eq!(green_rgb.green, 255, "wrong green cmp of green color: {}", green_rgb);
        assert_eq!(green_rgb.blue, 0, "wrong blue cmp of green color: {}", green_rgb);
    }

    #[test]
    fn adjust_hue_for_hsv() {
        let red_hsv: HsvColor = RgbColor::new(255, 0, 0).into_color();
        let green_hsv = red_hsv.adjust_hue(120.);
        assert_eq!(red_hsv.hue, 0.);
        assert_eq!(red_hsv.saturation, 100.);
        assert_eq!(red_hsv.value, 100.);
        assert_eq!(green_hsv.hue, 120.);
        assert_eq!(green_hsv.saturation, 100.);
        assert_eq!(green_hsv.value, 100.);
        let green_rgb: RgbColor = green_hsv.into();
        assert_eq!(green_rgb.red, 0, "wrong red: {}", green_rgb);
        assert_eq!(green_rgb.green, 255, "wrong green: {}", green_rgb);
        assert_eq!(green_rgb.blue, 0, "wrong blue: {}", green_rgb);
    }
}