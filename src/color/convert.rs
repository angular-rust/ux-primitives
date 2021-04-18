use super::*;

pub trait FromColor<Fr> {
    fn from_color(color: Fr) -> Self;
}

pub trait IntoColor<To> {
    fn into_color(self) -> To;
}

impl<Fr, To> IntoColor<To> for Fr
    where To: FromColor<Fr>
{
    fn into_color(self) -> To {
        To::from_color(self)
    }
}

// impl<C> FromColor<C> for C {
//     fn from_color(color: C) -> C { color }
// }

impl<Fr, To> FromColor<Fr> for To
    where To: From<Rgb>,
          Fr: Into<Rgb>
{
    fn from_color(from_color: Fr) -> To {
        To::from(from_color.into())
    }
}

pub trait ColorSpace: Clone + Copy + From<Rgb> + Into<Rgb> {}

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

}