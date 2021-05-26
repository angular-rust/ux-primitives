use super::*;

pub trait GetHue {
    fn get_hue(self) -> Float;
}
pub trait SetHue {
    fn set_hue(&mut self, hue: Float) -> Self;
}
pub trait HasHue: Clone + Copy + GetHue + SetHue {}

pub trait HasSaturation: Clone + Copy {
    fn get_saturation(self) -> Float;
    fn set_saturation(&mut self, saturation: Float) -> Self;
}

pub trait GetRadialSaturation:
    FromColor<HslColor> + IntoColor<HslColor> + FromColor<HsvColor> + IntoColor<HsvColor>
{
    fn get_hsl_saturation(self) -> Float;
    fn get_hsv_saturation(self) -> Float;
}
pub trait SetRadialSaturation:
    FromColor<HslColor> + IntoColor<HslColor> + FromColor<HsvColor> + IntoColor<HsvColor>
{
    fn set_hsl_saturation(&mut self, saturation: Float) -> Self;
    fn set_hsv_saturation(&mut self, saturation: Float) -> Self;
}

pub trait Lighten: Sized {
    fn lighten(self, delta: Float) -> Self;
    fn darken(self, delta: Float) -> Self {
        self.lighten(-delta)
    }
}

pub trait AdjustHue: Sized {
    fn adjust_hue(self, delta: Float) -> Self;
    fn complement(self) -> Self {
        self.adjust_hue(180.)
    }
}

pub trait Saturate: Sized {
    fn saturate(self, delta: Float) -> Self;
    fn desaturate(self, delta: Float) -> Self {
        self.saturate(-delta)
    }
}

pub trait Grayscale: Saturate {
    fn grayscale(self) -> Self {
        self.saturate(-100.)
    }
}
impl<C: Saturate> Grayscale for C {}

pub trait Invert: Sized {
    fn invert(self) -> Self;
}

pub trait Adjust: Lighten + AdjustHue + Saturate + Grayscale {}
impl<C: Lighten + AdjustHue + Saturate + Grayscale> Adjust for C {}

impl GetHue for HslColor {
    fn get_hue(self) -> Float {
        self.hue
    }
}
impl SetHue for HslColor {
    fn set_hue(&mut self, hue: Float) -> Self {
        self.hue = hue_bound(hue);
        *self
    }
}
impl GetHue for HsvColor {
    fn get_hue(self) -> Float {
        self.hue
    }
}
impl SetHue for HsvColor {
    fn set_hue(&mut self, hue: Float) -> Self {
        self.hue = hue_bound(hue);
        *self
    }
}

impl HasHue for HslColor {}
impl HasHue for HsvColor {}

impl HasSaturation for HslColor {
    fn get_saturation(self) -> Float {
        self.saturation
    }
    fn set_saturation(&mut self, saturation: Float) -> Self {
        self.saturation = clamp(saturation, 0., 100.);
        *self
    }
}
impl HasSaturation for HsvColor {
    fn get_saturation(self) -> Float {
        self.saturation
    }
    fn set_saturation(&mut self, saturation: Float) -> Self {
        self.saturation = clamp(saturation, 0., 100.);
        *self
    }
}

impl<C: NonRadialSpace> GetHue for C {
    fn get_hue(self) -> Float {
        let hsv: HsvColor = self.into_color();
        hsv.get_hue()
    }
}
impl<C: NonRadialSpace> SetHue for C {
    fn set_hue(&mut self, hue: Float) -> Self {
        let mut hsv: HsvColor = (*self).into_color();
        hsv.set_hue(hue);
        *self = hsv.into_color();
        *self
    }
}

impl<C> GetRadialSaturation for C
where
    C: Clone
        + Copy
        + FromColor<HslColor>
        + IntoColor<HslColor>
        + FromColor<HsvColor>
        + IntoColor<HsvColor>,
{
    fn get_hsl_saturation(self) -> Float {
        let hsl: HslColor = self.into_color();
        hsl.get_saturation()
    }
    fn get_hsv_saturation(self) -> Float {
        let hsv: HsvColor = self.into_color();
        hsv.get_saturation()
    }
}
impl<C> SetRadialSaturation for C
where
    C: Clone
        + Copy
        + FromColor<HslColor>
        + IntoColor<HslColor>
        + FromColor<HsvColor>
        + IntoColor<HsvColor>,
{
    fn set_hsl_saturation(&mut self, saturation: Float) -> Self {
        let mut hsl: HslColor = (*self).into_color();
        *self = hsl.set_saturation(saturation).into_color();
        *self
    }
    fn set_hsv_saturation(&mut self, saturation: Float) -> Self {
        let mut hsv: HsvColor = (*self).into_color();
        *self = hsv.set_saturation(saturation).into_color();
        *self
    }
}

impl<C: FromColor<HslColor> + IntoColor<HslColor>> Lighten for C {
    fn lighten(self, delta: Float) -> Self {
        let hsl: HslColor = self.into_color();
        let lightness = hsl.lightness + utils::clamp(delta, -100., 100.);
        C::from_color(HslColor {
            hue: hsl.hue,
            saturation: hsl.saturation,
            lightness: utils::clamp(lightness, 0., 100.),
        })
    }
}

impl<C: Clone + GetHue + SetHue> AdjustHue for C {
    fn adjust_hue(self, delta: Float) -> Self {
        self.clone().set_hue(self.get_hue() + delta)
    }
}

impl<C: Clone + GetRadialSaturation + SetRadialSaturation> Saturate for C {
    fn saturate(self, delta: Float) -> Self {
        self.clone()
            .set_hsl_saturation(self.get_hsl_saturation() + clamp(delta, -100., 100.))
    }
}

impl<C: ColorTransition> Invert for C {
    fn invert(self) -> Self {
        let Color {
            red,
            green,
            blue,
            alpha,
        } = self.into();
        C::from(Color {
            red: 1. - red,
            green: 1. - green,
            blue: 1. - blue,
            alpha,
        })
    }
}

// Implementations for colors with alpha channel
impl<C: Lighten + ColorSpace> Lighten for Alpha<C> {
    fn lighten(self, delta: Float) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.lighten(delta), alpha)
    }
}
impl<C: AdjustHue + ColorSpace> AdjustHue for Alpha<C> {
    fn adjust_hue(self, delta: Float) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.adjust_hue(delta), alpha)
    }
}
impl<C: Saturate + ColorSpace> Saturate for Alpha<C> {
    fn saturate(self, delta: Float) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.saturate(delta), alpha)
    }
}
impl<C: Invert + ColorSpace> Invert for Alpha<C> {
    fn invert(self) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.invert(), alpha)
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    // use math::round::half_down;

    // [x] make test for all adjustments
    // [x] add adjustments impl for alpha::Alpha
    // [x] replace unicolor::Color enum by Color struct with rgba floats (0.0 - 1.0)
    // [ ] add tests of adjustments for alpha::Alpha
    // [ ] add tests of adjustments for Color

    #[test]
    fn adjust_hue_for_rgb() {
        let red_rgb = RgbColor::new(255, 0, 0);
        let green_rgb = red_rgb.adjust_hue(120.);
        assert_eq!(
            green_rgb.red, 0,
            "wrong red cmp of green color: {}",
            green_rgb
        );
        assert_eq!(
            green_rgb.green, 255,
            "wrong green cmp of green color: {}",
            green_rgb
        );
        assert_eq!(
            green_rgb.blue, 0,
            "wrong blue cmp of green color: {}",
            green_rgb
        );
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

    // #[test]
    // fn lighten_for_rgb_color() {
    //     let base = RgbColor::new(204, 0, 0); // hsl(0, 100, 40)
    //     let lighten = base.lighten(5.); // rgb(230, 0, 0) <- hsl(0, 100, 45)
    //     let darken = base.darken(10.); // rgb(153, 0, 0) <- hsl(0, 100, 30)
    //     assert_eq!(half_down(HslColor::from(lighten).lightness as f64, 0), 45.);
    //     assert_eq!(half_down(HslColor::from(darken).lightness as f64, 10), 30.);
    //     assert_eq!(lighten.red, 230);
    //     assert_eq!(darken.red, 153);
    // }

    // #[test]
    // fn lighten_for_lin_rgb() {
    //     let base: Color = RgbColor::new(204, 0, 0).into(); // hsl(0, 100, 40)
    //     let lighten = base.lighten(5.); // rgb(230, 0, 0) <- hsl(0, 100, 45)
    //     let darken = base.darken(10.); // rgb(153, 0, 0) <- hsl(0, 100, 30)
    //     assert_eq!(half_down(HslColor::from(lighten).lightness as f64, 0), 45.);
    //     assert_eq!(half_down(HslColor::from(darken).lightness as f64, 10), 30.);
    //     assert_eq!(RgbColor::from(lighten).red, 230);
    //     assert_eq!(RgbColor::from(darken).red, 153);
    // }

    #[test]
    fn saturate() {
        let base = RgbColor::new(127, 63, 191); // hsl(270, 50, 50)
        let saturated = base.saturate(20.); // rgb(127, 38, 216) <- hsl(270, 70, 50)
        let desaturated = base.desaturate(20.); // rgb(127, 89, 165) <- hsl(270, 30, 50)
        assert_eq!(base.get_hsl_saturation().round(), 50.);
        assert_eq!(saturated.get_hsl_saturation().round(), 70.);
        assert_eq!(desaturated.get_hsl_saturation().round(), 31.);
    }

    #[test]
    fn grayscale() {
        let base = RgbColor::new(102, 61, 142); // hsl(270, 40, 40)
        let grayscale = base.grayscale();
        assert_eq!(grayscale.red, 102);
        assert_eq!(grayscale.green, 102);
        assert_eq!(grayscale.blue, 102);
    }

    #[test]
    fn get_saturation() {
        let color = RgbColor::new(102, 61, 142); // hsl(270, 40, 40), hsv(270, 57, 55)
        assert_eq!(color.get_hsl_saturation().round(), 40.);
        assert_eq!(color.get_hsv_saturation().round(), 57.);
    }

    #[test]
    fn complement() {
        let base: CmykColor = HslColor::new(60., 50., 50.).into();
        assert_eq!(base.get_hue(), 60.);
        let base_rgb = RgbColor::from(base);
        assert_eq!(base_rgb.red, 191);
        assert_eq!(base_rgb.green, 191);
        assert_eq!(base_rgb.blue, 63);

        let complemented = base.complement();
        assert_eq!(complemented.get_hue(), 240.);
        let complemented_rgb: RgbColor = complemented.into();
        assert_eq!(complemented_rgb.red, 63);
        assert_eq!(complemented_rgb.green, 63);
        assert_eq!(complemented_rgb.blue, 191);
    }

    #[test]
    fn invert() {
        let color = RgbColor::new(100, 60, 255);
        let inverted = color.invert();
        assert_eq!(inverted.red, 155);
        assert_eq!(inverted.green, 195);
        assert_eq!(inverted.blue, 0);
    }
}
