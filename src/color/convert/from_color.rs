use super::*;

// impl<C> From<Color> for Result<C, ColorError>
//     where C: ColorSpace
// {
//     fn from(c: Color) -> Self {
//         match c {
//             Color::RGB(r, g, b) => Ok(RgbColor { r, g, b }),
//             Color::RGBA(r, g, b, _) => Ok(RgbColor { r, g, b }),
//             Color::HSL(h, s, l) => HslColor{h, s, l}.into(),
//             Color::HSV(h, s, v) => HsvColor{h, s, v}.into(),
//             Color::CMYK(c, m, y, k) => CmykColor{c, m, y, k}.into(),
//             Color::CMY(c, m, y) => CmyColor{c, m, y}.into(),
//             #[cfg(feature = "experimental")]
//             Color::LAB(l, a, b) => LabColor{l, a, b}.into(),
//             #[cfg(feature = "experimental")]
//             Color::XYZ(x, y, z) => XyzColor{x, y, z}.into()
//         }
//     }
// }

// Color enum -> RgbColor
impl From<Color> for RgbColor {
    fn from(c: Color) -> RgbColor {
        match Result::<RgbColor, ColorError>::from(c) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting Color to RgbColor failed: {}", err)
        }
    }
}
impl From<Color> for Result<RgbColor, ColorError> {
    fn from(c: Color) -> Result<RgbColor, ColorError> {
        match c {
            Color::RGB(r, g, b) => Ok(RgbColor { r, g, b }),
            Color::RGBA(r, g, b, _) => Ok(RgbColor { r, g, b }),
            Color::HSL(h, s, l) => HslColor{h, s, l}.into(),
            Color::HSV(h, s, v) => HsvColor{h, s, v}.into(),
            Color::CMYK(c, m, y, k) => CmykColor{c, m, y, k}.into(),
            Color::CMY(c, m, y) => CmyColor{c, m, y}.into(),
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => LabColor{l, a, b}.into(),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => XyzColor{x, y, z}.into()
        }
    }
}

// Color enum -> RgbaColor
impl From<Color> for RgbaColor {
    fn from(c: Color) -> RgbaColor {
        match Result::<RgbaColor, ColorError>::from(c) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting Color to RgbaColor failed: {}", err)
        }
    }
}
impl From<Color> for Result<RgbaColor, ColorError> {
    fn from(c: Color) -> Result<RgbaColor, ColorError> {
        match c {
            Color::RGB(r, g, b) => Ok(RgbaColor { r, g, b, a: 0xFF}),
            Color::RGBA(r, g, b, a) => Ok(RgbaColor { r, g, b, a }),
            Color::HSL(h, s, l) => RgbColor::from(HslColor{h, s, l}).into(),
            Color::HSV(h, s, v) => RgbColor::from(HsvColor{h, s, v}).into(),
            Color::CMYK(c, m, y, k) => RgbColor::from(CmykColor{c, m, y, k}).into(),
            Color::CMY(c, m, y) => RgbColor::from(CmyColor{c, m, y}).into(),
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => RgbColor::from(LabColor{l, a, b}).into(),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => RgbColor::from(XyzColor{x, y, z}).into(),
        }
    }
}

// Color enum -> HslColor
impl From<Color> for HslColor {
    fn from(c: Color) -> HslColor {
        match Result::<HslColor, ColorError>::from(c) {
            Ok(hsl) => hsl,
            Err(err) => panic!("Converting Color to CmykColor failed: {}", err)
        }
    }
}
impl From<Color> for Result<HslColor, ColorError> {
    fn from(c: Color) -> Result<HslColor, ColorError> {
        match c {
            Color::RGB(r, g, b) => RgbColor { r, g, b }.into(),
            Color::RGBA(r, g, b, _) => RgbColor { r, g, b }.into(),
            Color::HSL(h, s, l) => Ok(HslColor{h, s, l}),
            Color::HSV(h, s, v) => RgbColor::from(HsvColor{h, s, v}).into(),
            Color::CMYK(c, m, y, k) => RgbColor::from(CmykColor{c, m, y, k}).into(),
            Color::CMY(c, m, y) => RgbColor::from(CmykColor{c, m, y, k: 0.0}).into(),
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => RgbColor::from(LabColor{l, a, b}).into(),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => RgbColor::from(XyzColor{x, y, z}).into()
        }
    }
}

// Color enum -> CmykColor
impl From<Color> for CmykColor {
    fn from(c: Color) -> CmykColor {
        match Result::<CmykColor, ColorError>::from(c) {
            Ok(cmyk) => cmyk,
            Err(err) => panic!("Converting Color to CmykColor failed: {}", err)
        }
    }
}
impl From<Color> for Result<CmykColor, ColorError> {
    fn from(c: Color) -> Result<CmykColor, ColorError> {
        match c {
            Color::RGB(r, g, b) => RgbColor { r, g, b }.into(),
            Color::RGBA(r, g, b, _) => RgbColor { r, g, b }.into(),
            Color::HSL(h, s, l) => RgbColor::from(HslColor{h, s, l}).into(),
            Color::HSV(h, s, v) => RgbColor::from(HsvColor{h, s, v}).into(),
            Color::CMYK(c, m, y, k) => Ok(CmykColor{c, m, y, k}),
            Color::CMY(c, m, y) => Ok(CmykColor{c, m, y, k: 0.0}),
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => RgbColor::from(LabColor{l, a, b}).into(),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => RgbColor::from(XyzColor{x, y, z}).into()
        }
    }
}