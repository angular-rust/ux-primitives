use super::*;

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

trait Adjust: Lighten + AdjustHue + Saturate + Grayscale {}


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

impl AdjustHue for HslColor {
    fn adjust_hue(self, delta: f64) -> Self {
        Self {
            hue: hue_bound(self.hue + delta),
            saturation: self.saturation,
            lightness: self.lightness
        }
    }
}
impl AdjustHue for HsvColor {
    fn adjust_hue(self, delta: f64) -> Self {
        Self {
            hue: hue_bound(self.hue + delta),
            saturation: self.saturation,
            value: self.value
        }
    }
}
impl<C: NonRadialSpace> AdjustHue for C {
    fn adjust_hue(self, delta: f64) -> Self {
        let hsv: HsvColor = self.into_color();
        C::from_color(hsv.adjust_hue(delta))
    }
}


impl Saturate for HslColor {
    fn saturate(self, delta: f64) -> Self {
        if (delta.abs() - 100.) > f64::EPSILON {
            panic!("saturate failed: {}. Actual delta is {}", ColorError::PercentageOverflow, delta)
        }
        HslColor {
            hue: self.hue,
            saturation: utils::percentage_bound(self.saturation + delta),
            lightness: self.lightness
        }
    }
}
impl Saturate for HsvColor {
    fn saturate(self, delta: f64) -> Self {
        if (delta.abs() - 100.) > f64::EPSILON {
            panic!("saturate failed: {}. Actual delta is {}", ColorError::PercentageOverflow, delta)
        }
        HsvColor {
            hue: self.hue,
            saturation: utils::percentage_bound(self.saturation + delta),
            value: self.value
        }
    }
}
impl<C: NonSaturationSpace + FromColor<HsvColor> + IntoColor<HsvColor>> Saturate for C {
    fn saturate(self, delta: f64) -> Self {
        let with_saturation: HsvColor = self.clone().into_color();
        C::from_color(with_saturation.set_saturation(delta))
    }
}

impl Grayscale for HslColor {}
impl Grayscale for HsvColor {}
impl<C: NonSaturationSpace + FromColor<HsvColor> + IntoColor<HsvColor>> Grayscale for C {}

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
            alpha: 255 - self.alpha,
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

}