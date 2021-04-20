use super::*;

pub trait FromColor<Fr> {
    fn from_color(color: Fr) -> Self;
}

pub trait IntoColor<To> {
    fn into_color(self) -> To;
}

impl<Fr, To> FromColor<Fr> for To
    where To: From<Rgb>,
          Fr: Into<Rgb>
{
    fn from_color(from_color: Fr) -> To {
        To::from(from_color.into())
    }
}

impl<Fr, To> IntoColor<To> for Fr
    where To: FromColor<Fr>
{
    fn into_color(self) -> To {
        To::from_color(self)
    }
}

pub trait ColorSpace: Clone + Copy + From<Rgb> + Into<Rgb> {}
pub trait NonRgbSpace: ColorSpace {}
pub trait NonRadialSpace: ColorSpace {}
pub trait NonSaturationSpace: ColorSpace {}

impl ColorSpace for Rgb {}
impl ColorSpace for RgbColor {}
impl ColorSpace for RgbaColor {}
impl ColorSpace for HslColor {}
impl ColorSpace for HsvColor {}
impl ColorSpace for CmykColor {}
impl ColorSpace for CmyColor {}
#[cfg(feature = "experimental")]
impl ColorSpace for LabColor {}
#[cfg(feature = "experimental")]
impl ColorSpace for XyzColor {}

impl NonRgbSpace for HslColor {}
impl NonRgbSpace for HsvColor {}
impl NonRgbSpace for CmykColor {}
impl NonRgbSpace for CmyColor {}
#[cfg(feature = "experimental")]
impl NonRgbSpace for LabColor {}
#[cfg(feature = "experimental")]
impl NonRgbSpace for XyzColor {}

impl NonRadialSpace for Rgb {}
impl NonRadialSpace for RgbColor {}
impl NonRadialSpace for RgbaColor {}
impl NonRadialSpace for CmykColor {}
impl NonRadialSpace for CmyColor {}
#[cfg(feature = "experimental")]
impl NonRadialSpace for LabColor {}
#[cfg(feature = "experimental")]
impl NonRadialSpace for XyzColor {}

impl NonSaturationSpace for Rgb {}
impl NonSaturationSpace for RgbColor {}
impl NonSaturationSpace for RgbaColor {}
impl NonSaturationSpace for CmykColor {}
impl NonSaturationSpace for CmyColor {}
#[cfg(feature = "experimental")]
impl NonSaturationSpace for LabColor {}
#[cfg(feature = "experimental")]
impl NonSaturationSpace for XyzColor {}

// RGB -> ALL
impl From<RgbColor> for RgbaColor { fn from(c: RgbColor) -> Self { RgbaColor { red: c.red, green: c.green, blue: c.blue, alpha: 255 } } }
impl From<RgbColor> for HslColor { fn from(c: RgbColor) -> Self { Self::from_color(c) } }
impl From<RgbColor> for HsvColor { fn from(c: RgbColor) -> Self { Self::from_color(c) } }
impl From<RgbColor> for CmykColor { fn from(c: RgbColor) -> Self { Self::from_color(c) } }
impl From<RgbColor> for CmyColor { fn from(c: RgbColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<RgbColor> for LabColor { fn from(c: RgbColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<RgbColor> for XyzColor { fn from(c: RgbColor) -> Self { Self::from_color(c) } }

// RGBA -> ALL
impl From<RgbaColor> for RgbColor { fn from(c: RgbaColor) -> Self { RgbColor { red: c.red, green: c.green, blue: c.blue } } }
impl From<RgbaColor> for HslColor { fn from(c: RgbaColor) -> Self { Self::from_color(c) } }
impl From<RgbaColor> for HsvColor { fn from(c: RgbaColor) -> Self { Self::from_color(c) } }
impl From<RgbaColor> for CmykColor { fn from(c: RgbaColor) -> Self { Self::from_color(c) } }
impl From<RgbaColor> for CmyColor { fn from(c: RgbaColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<RgbaColor> for LabColor { fn from(c: RgbaColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<RgbaColor> for XyzColor { fn from(c: RgbaColor) -> Self { Self::from_color(c) } }

// HSL -> ALL
impl From<HslColor> for RgbColor { fn from(c: HslColor) -> Self { Self::from_color(c) } }
impl From<HslColor> for RgbaColor { fn from(c: HslColor) -> Self { Self::from_color(c) } }
impl From<HslColor> for HsvColor { fn from(c: HslColor) -> Self { Self::from_color(c) } }
impl From<HslColor> for CmykColor { fn from(c: HslColor) -> Self { Self::from_color(c) } }
impl From<HslColor> for CmyColor { fn from(c: HslColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<HslColor> for LabColor { fn from(c: HslColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<HslColor> for XyzColor { fn from(c: HslColor) -> Self { Self::from_color(c) } }

// HSV -> ALL
impl From<HsvColor> for RgbColor { fn from(c: HsvColor) -> Self { Self::from_color(c) } }
impl From<HsvColor> for RgbaColor { fn from(c: HsvColor) -> Self { Self::from_color(c) } }
impl From<HsvColor> for HslColor { fn from(c: HsvColor) -> Self { Self::from_color(c) } }
impl From<HsvColor> for CmykColor { fn from(c: HsvColor) -> Self { Self::from_color(c) } }
impl From<HsvColor> for CmyColor { fn from(c: HsvColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<HsvColor> for LabColor { fn from(c: HsvColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<HsvColor> for XyzColor { fn from(c: HsvColor) -> Self { Self::from_color(c) } }

// CMYK -> ALL
impl From<CmykColor> for RgbColor { fn from(c: CmykColor) -> Self { Self::from_color(c) } }
impl From<CmykColor> for RgbaColor { fn from(c: CmykColor) -> Self { Self::from_color(c) } }
impl From<CmykColor> for HslColor { fn from(c: CmykColor) -> Self { Self::from_color(c) } }
impl From<CmykColor> for HsvColor { fn from(c: CmykColor) -> Self { Self::from_color(c) } }
impl From<CmykColor> for CmyColor { fn from(c: CmykColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<CmykColor> for LabColor { fn from(c: CmykColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<CmykColor> for XyzColor { fn from(c: CmykColor) -> Self { Self::from_color(c) } }

// CMY -> ALL
impl From<CmyColor> for RgbColor { fn from(c: CmyColor) -> Self { Self::from_color(c) } }
impl From<CmyColor> for RgbaColor { fn from(c: CmyColor) -> Self { Self::from_color(c) } }
impl From<CmyColor> for HslColor { fn from(c: CmyColor) -> Self { Self::from_color(c) } }
impl From<CmyColor> for HsvColor { fn from(c: CmyColor) -> Self { Self::from_color(c) } }
impl From<CmyColor> for CmykColor { fn from(c: CmyColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<CmyColor> for LabColor { fn from(c: CmyColor) -> Self { Self::from_color(c) } }
#[cfg(feature = "experimental")]
impl From<CmyColor> for XyzColor { fn from(c: CmyColor) -> Self { Self::from_color(c) } }

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn into_color_self() {
        let rgb1 = Rgb::new(200., 200., 200.);
        let rgb2: Rgb = rgb1.into_color();
        assert_eq!(rgb1.red, rgb2.red);
        assert_eq!(rgb1.green, rgb2.green);
        assert_eq!(rgb1.blue, rgb2.blue);
        assert_eq!(rgb1.get_hue(), rgb2.get_hue());
        assert_eq!(rgb1.get_hsl_saturation(), rgb2.get_hsl_saturation());
    }
}