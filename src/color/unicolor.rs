use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    /// A representation of the RGB (Red, Green, Blue) color space.
    RGB(u8, u8, u8),
    /// A representation of the RGBA (Red, Green, Blue, Alpha) color space.
    RGBA(u8, u8, u8, u8),
    /// A representation of the HSL (Hue, Saturation, Value) color space.
    HSV(f64, f64, f64),
    // /// A representation of the HSL (Hue, Saturation, Value, Alpha) color space.
    // HSVA(f64, f64, f64, f64),
    /// A representation of the HSL (Hue, Saturation, Lightness) color space.
    HSL(f64, f64, f64),
    // /// A representation of the HSL (Hue, Saturation, Lightness, Alpha) color space.
    // HSLA(f64, f64, f64, f64),
    /// A representation of the CMYK (Cyan, Magenta, Yellow) color space.
    CMY(f64, f64, f64),
    /// A representation of the CMYK (Cyan, Magenta, Yellow, Key) color space.
    CMYK(f64, f64, f64, f64),

    /// representation of the XYZ color space
    #[cfg(feature = "experimental")]
    XYZ(f64, f64, f64),
    /// A representation of the L*a*b color space
    #[cfg(feature = "experimental")]
    LAB(f64, f64, f64),
}

impl Default for Color {
    fn default() -> Self { palette::BLACK }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::RGB(red, green, blue) => write!(f, "rgb({}, {}, {})", red, green, blue),
            Color::RGBA(red, green, blue, alpha) => {
                write!(f, "rgba({}, {}, {}, {})", red, green, blue, alpha)
            },
            Color::HSV(hue, saturation, value) => {
                write!(f, "hsv({}°, {}%, {}%)", hue, saturation, value)
            },
            Color::HSL(hue, saturation, lightness) => {
                write!(f, "hsl({}°, {}%, {}%)", hue, saturation, lightness)
            },
            Color::CMY(cyan, magenta, yellow) => {
                write!(f, "cmy({}%, {}%, {}%)", cyan, magenta, yellow)
            },
            Color::CMYK(cyan, magenta, yellow, key) => {
                write!(f, "cmyk({}%, {}%, {}%, {}%)", cyan, magenta, yellow, key)
            },
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => write!(f, "lab({}, {}, {})", l, a, b),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => write!(f, "xyz({}, {}, {})", x, y, z),
        }
    }
}

pub trait FromUniColor {
    fn from_uni_color(c: Color) -> Self;
}

impl<C> FromUniColor for C
    where C: From<Rgb>
{
    fn from_uni_color(c: Color) -> Self {
        let rgb = match c {
            Color::RGB(red, green, blue) => Rgb {
                red: red as f64 / 255.,
                green: green as f64 / 255.,
                blue: blue as f64 / 255. 
            },
            Color::RGBA(red, green, blue, _) => Rgb {
                red: red as f64 / 255.,
                green: green as f64 / 255.,
                blue: blue as f64 / 255.
            },
            Color::HSL(hue, saturation, lightness) => Rgb::from(HslColor{ hue, saturation, lightness }),
            Color::HSV(hue, saturation, value) => Rgb::from(HsvColor{ hue, saturation, value }),
            Color::CMYK(cyan, magenta, yellow, key) => Rgb::from(CmykColor{ cyan, magenta, yellow, key }),
            Color::CMY(cyan, magenta, yellow) => Rgb::from(CmyColor{ cyan, magenta, yellow }),
            #[cfg(feature = "experimental")]
            Color::LAB(l, a, b) => Rgb::from(LabColor{l, a, b}),
            #[cfg(feature = "experimental")]
            Color::XYZ(x, y, z) => Rgb::from(XyzColor{x, y, z})
        };
        C::from(rgb)
    }
}

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        Rgb::from_uni_color(c)
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Color::RGB(
            rgb.red as u8,
            rgb.green as u8,
            rgb.blue as u8
        )
    }
}

impl From<Color> for RgbColor {
    fn from(c: Color) -> Self {
        match c {
            Color::RGB(red, green, blue) => RgbColor{ red, green, blue },
            Color::RGBA(red, green, blue, _) => RgbColor{ red, green, blue },
            _ => Self::from_uni_color(c)
        }
    }
}

impl From<Color> for RgbaColor {
    fn from(c: Color) -> Self {
        match c {
            Color::RGB(red, green, blue) => RgbaColor{ red, green, blue, alpha: 255 },
            Color::RGBA(red, green, blue, alpha) => RgbaColor{ red, green, blue, alpha },
            _ => Self::from_uni_color(c)
        }
    }
}

impl From<Color> for CmyColor {
    fn from(c: Color) -> Self {
        match c {
            Color::CMY(cyan, magenta, yellow) => CmyColor { cyan, magenta, yellow },
            _ => Self::from_uni_color(c)
        }
    }
}

impl From<Color> for CmykColor {
    fn from(c: Color) -> Self {
        match c {
            Color::CMYK(cyan, magenta, yellow, key) => CmykColor { cyan, magenta, yellow, key },
            _ => Self::from_uni_color(c)
        }
    }
}

impl From<Color> for HslColor {
    fn from(c: Color) -> Self {
        match c {
            Color::HSL(hue, saturation, lightness) => HslColor { hue, saturation, lightness },
            _ => Self::from_uni_color(c)
        }
    }
}

impl From<Color> for HsvColor {
    fn from(c: Color) -> Self {
        match c {
            Color::HSL(hue, saturation, value) => HsvColor { hue, saturation, value },
            _ => Self::from_uni_color(c)
        }
    }
}

#[cfg(feature = "experimental")]
impl From<Color> for LabColor {
    fn from(c: Color) -> Self {
        match c {
            Color::LAB(l, a, b) => LabColor { l, a, b },
            _ => Self::from_uni_color(c)
        }
    }
}

#[cfg(feature = "experimental")]
impl From<Color> for XyzColor {
    fn from(c: Color) -> Self {
        match c {
            Color::XYZ(x, y, z) => XyzColor { x, y, z },
            _ => Self::from_uni_color(c)
        }
    }
}

impl From<RgbColor> for Color {
    fn from(c: RgbColor) -> Color { Color::RGB(c.red, c.green, c.blue) }
}
impl From<RgbaColor> for Color {
    fn from(c: RgbaColor) -> Color { Color::RGBA(c.red, c.green, c.blue, c.alpha) }
}
impl From<HslColor> for Color {
    fn from(c: HslColor) -> Color { Color::HSL(c.hue, c.saturation, c.lightness) }
}
impl From<HsvColor> for Color {
    fn from(c: HsvColor) -> Color { Color::HSV(c.hue, c.saturation, c.value) }
}
impl From<CmykColor> for Color {
    fn from(c: CmykColor) -> Color { Color::CMYK(c.cyan, c.magenta, c.yellow, c.key) }
}
impl From<CmyColor> for Color {
    fn from(c: CmyColor) -> Color { Color::CMY(c.cyan, c.magenta, c.yellow) }
}

#[cfg(feature = "experimental")]
impl From<LabColor> for Color {
    fn from(c: LabColor) -> Color { Color::LAB(c.l, c.a, c.b) }
}
#[cfg(feature = "experimental")]
impl From<XyzColor> for Color {
    fn from(c: XyzColor) -> Color { Color::XYZ(c.x, c.y, c.z) }
}
