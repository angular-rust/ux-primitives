use super::*;

// Color enum -> RgbColor
impl From<Color> for RgbColor {
    fn from(c: Color) -> Self {
        match c {
            Color::RGB(r, g, b) => RgbColor { r, g, b },
            Color::RGBA(r, g, b, _) => RgbColor { r, g, b },
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
    fn from(c: Color) -> Self {
        match c {
            Color::RGB(r, g, b) => RgbaColor { r, g, b, a: 0xFF},
            Color::RGBA(r, g, b, a) => RgbaColor { r, g, b, a },
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
    fn from(c: Color) -> Self {
        match c {
            Color::RGB(r, g, b) => RgbColor { r, g, b }.into(),
            Color::RGBA(r, g, b, _) => RgbColor { r, g, b }.into(),
            Color::HSL(h, s, l) => HslColor{h, s, l},
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
    fn from(c: Color) -> Self {
        match c {
            Color::RGB(r, g, b) => RgbColor { r, g, b }.into(),
            Color::RGBA(r, g, b, _) => RgbColor { r, g, b }.into(),
            Color::HSL(h, s, l) => RgbColor::from(HslColor{h, s, l}).into(),
            Color::HSV(h, s, v) => RgbColor::from(HsvColor{h, s, v}).into(),
            Color::CMYK(c, m, y, k) => CmykColor{c, m, y, k},
            Color::CMY(c, m, y) => CmykColor{c, m, y, k: 0.0},
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => RgbColor::from(LabColor{l, a, b}).into(),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => RgbColor::from(XyzColor{x, y, z}).into()
        }
    }
}
