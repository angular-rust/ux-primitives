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
            Err(err) => panic!("Converting Color to RgbColor failed: {}", err),
        }
    }
}
impl From<Color> for Result<RgbColor, ColorError> {
    fn from(c: Color) -> Result<RgbColor, ColorError> {
        match c {
            Color::RGB(red, green, blue) => Ok(RgbColor { red, green, blue }),
            Color::RGBA(red, green, blue, _) => Ok(RgbColor { red, green, blue }),
            Color::HSL(hue, saturation, lightness) => HslColor {
                hue,
                saturation,
                lightness,
            }
            .into(),
            Color::HSV(hue, saturation, value) => HsvColor {
                hue,
                saturation,
                value,
            }
            .into(),
            Color::CMYK(cyan, magenta, yellow, key) => CmykColor {
                cyan,
                magenta,
                yellow,
                key,
            }
            .into(),
            Color::CMY(cyan, magenta, yellow) => CmyColor {
                cyan,
                magenta,
                yellow,
            }
            .into(),
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => LabColor { l, a, b }.into(),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => XyzColor { x, y, z }.into(),
        }
    }
}

// Color enum -> RgbaColor
impl From<Color> for RgbaColor {
    fn from(c: Color) -> RgbaColor {
        match Result::<RgbaColor, ColorError>::from(c) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting Color to RgbaColor failed: {}", err),
        }
    }
}
impl From<Color> for Result<RgbaColor, ColorError> {
    fn from(c: Color) -> Result<RgbaColor, ColorError> {
        match c {
            Color::RGB(red, green, blue) => Ok(RgbaColor {
                red,
                green,
                blue,
                alpha: 0xFF,
            }),
            Color::RGBA(red, green, blue, alpha) => Ok(RgbaColor {
                red,
                green,
                blue,
                alpha,
            }),
            Color::HSL(hue, saturation, lightness) => RgbColor::from(HslColor {
                hue,
                saturation,
                lightness,
            })
            .into(),
            Color::HSV(hue, saturation, value) => RgbColor::from(HsvColor {
                hue,
                saturation,
                value,
            })
            .into(),
            Color::CMYK(cyan, magenta, yellow, k) => RgbColor::from(CmykColor {
                cyan,
                magenta,
                yellow,
                key: k,
            })
            .into(),
            Color::CMY(cyan, magenta, yellow) => RgbColor::from(CmyColor {
                cyan,
                magenta,
                yellow,
            })
            .into(),
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => RgbColor::from(LabColor { l, a, b }).into(),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => RgbColor::from(XyzColor { x, y, z }).into(),
        }
    }
}

// Color enum -> HslColor
impl From<Color> for HslColor {
    fn from(c: Color) -> HslColor {
        match Result::<HslColor, ColorError>::from(c) {
            Ok(hsl) => hsl,
            Err(err) => panic!("Converting Color to CmykColor failed: {}", err),
        }
    }
}
impl From<Color> for Result<HslColor, ColorError> {
    fn from(c: Color) -> Result<HslColor, ColorError> {
        match c {
            Color::RGB(red, green, blue) => RgbColor { red, green, blue }.into(),
            Color::RGBA(red, green, blue, _) => RgbColor { red, green, blue }.into(),
            Color::HSL(hue, saturation, lightness) => Ok(HslColor {
                hue,
                saturation,
                lightness,
            }),
            Color::HSV(hue, saturation, value) => RgbColor::from(HsvColor {
                hue,
                saturation,
                value,
            })
            .into(),
            Color::CMYK(cyan, magenta, yellow, key) => RgbColor::from(CmykColor {
                cyan,
                magenta,
                yellow,
                key,
            })
            .into(),
            Color::CMY(cyan, magenta, yellow) => RgbColor::from(CmykColor {
                cyan,
                magenta,
                yellow,
                key: 0.0,
            })
            .into(),
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => RgbColor::from(LabColor { l, a, b }).into(),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => RgbColor::from(XyzColor { x, y, z }).into(),
        }
    }
}

// Color enum -> CmykColor
impl From<Color> for CmykColor {
    fn from(c: Color) -> CmykColor {
        match Result::<CmykColor, ColorError>::from(c) {
            Ok(cmyk) => cmyk,
            Err(err) => panic!("Converting Color to CmykColor failed: {}", err),
        }
    }
}
impl From<Color> for Result<CmykColor, ColorError> {
    fn from(c: Color) -> Result<CmykColor, ColorError> {
        match c {
            Color::RGB(red, green, blue) => RgbColor { red, green, blue }.into(),
            Color::RGBA(red, green, blue, _) => RgbColor { red, green, blue }.into(),
            Color::HSL(hue, saturation, lightness) => RgbColor::from(HslColor {
                hue,
                saturation,
                lightness,
            })
            .into(),
            Color::HSV(hue, saturation, value) => RgbColor::from(HsvColor {
                hue,
                saturation,
                value,
            })
            .into(),
            Color::CMYK(c, m, y, k) => Ok(CmykColor {
                cyan: c,
                magenta: m,
                yellow: y,
                key: k,
            }),
            Color::CMY(c, m, y) => Ok(CmykColor {
                cyan: c,
                magenta: m,
                yellow: y,
                key: 0.0,
            }),
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => RgbColor::from(LabColor { l, a, b }).into(),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => RgbColor::from(XyzColor { x, y, z }).into(),
        }
    }
}
