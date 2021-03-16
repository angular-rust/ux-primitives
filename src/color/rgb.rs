use std::fmt;
use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RgbColor {
    pub r: u8, pub g: u8, pub b: u8,
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl ToHexString for RgbColor {
    fn to_hex_string(&self) -> String {
        format!("#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
    }
}

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
            Color::LAB(l, a, b) => LabColor{l, a, b}.into(),
        }
    }
}

// RGBA -> RGB
impl From<RgbaColor> for RgbColor {
    fn from(rgb: RgbaColor) -> RgbColor {
        RgbColor {r: rgb.r, g: rgb.g, b: rgb.b }
    }
}

// HLS -> RGB
impl From<HslColor> for RgbColor {
    fn from(hls: HslColor) -> RgbColor  {
        match Result::<RgbColor, ColorError>::from(hls) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting HslColor to RgbColor failed: {}", err)
        }
    }
}
impl From<HslColor> for Result<RgbColor, ColorError> {
    fn from(hsl: HslColor) -> Result<RgbColor, ColorError> {
        let HslColor { h: hue, s: saturation, l: lightness} = hsl;
        let c = (1. - ((2. * (lightness as f64 / 100.)) - 1.).abs())
            * (saturation as f64 / 100.);
        let x = c * (1. - ((((hue as f64) / 60.) % 2.) - 1.).abs());
        let m = (lightness as f64 / 100.) - (c / 2.);

        let (r_prime, g_prime, b_prime) = {
            if hue >= 0. && hue < 60. {
                (c, x, 0.)
            } else if hue >= 60. && hue < 120. {
                (x, c, 0.)
            } else if hue >= 120. && hue < 180. {
                (0., c, x)
            } else if hue >= 180. && hue < 240. {
                (0., x, c)
            } else if hue >= 240. && hue < 300. {
                (x, 0., c)
            } else if hue >= 300. && hue < 360. {
                (c, 0., x)
            } else {
                return Err(ColorError::DegreeOverflow);
            }
        };

        let apply = |v: f64| ((v + m) * 255.).round() as u8;
        Ok(RgbColor { r: apply(r_prime), g: apply(g_prime), b: apply(b_prime) })
    }
}


// CMYK -> RGB
impl From<CmykColor> for RgbColor {
    fn from(cmyk: CmykColor) -> RgbColor  {
        match Result::<RgbColor, ColorError>::from(cmyk) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting CmykColor to RgbColor failed: {}", err)
        }
    }
}
impl From<CmykColor> for Result<RgbColor, ColorError> {
    fn from(cmyk: CmykColor) -> Result<RgbColor, ColorError> {
        let apply = |v| {
            (255.0
                * (1.0f64 - v as f64 / 100.0)
                * (1.0 - cmyk.k as f64 / 100.0)
            ).round() as u8
        };
        Ok(RgbColor { r: apply(cmyk.c), g: apply(cmyk.m), b: apply(cmyk.y) })
    }
}

// HSV -> RGB
impl From<HsvColor> for RgbColor {
    fn from(hsv: HsvColor) -> RgbColor  {
        match Result::<RgbColor, ColorError>::from(hsv) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting HsvColor to RgbColor failed: {}", err)
        }
    }
}
impl From<HsvColor> for Result<RgbColor, ColorError> {
    fn from(_: HsvColor) -> Result<RgbColor, ColorError> {
        // TODO: implement HSV -> RGB
        Err(ColorError::Unimplemented)
    }
}

// CMY -> RGB
impl From<CmyColor> for RgbColor {
    fn from(cmy: CmyColor) -> RgbColor  {
        match Result::<RgbColor, ColorError>::from(cmy) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting CmyColor to RgbColor failed: {}", err)
        }
    }
}
impl From<CmyColor> for Result<RgbColor, ColorError> {
    fn from(_: CmyColor) -> Result<RgbColor, ColorError> {
        // TODO: implement CMY -> RGB
        Err(ColorError::Unimplemented)
    }
}

// L*a*b -> RGB
impl From<LabColor> for RgbColor {
    fn from(lab: LabColor) -> RgbColor  {
        match Result::<RgbColor, ColorError>::from(lab) {
            Ok(rgb) => rgb,
            Err(err) => panic!("Converting LabColor to RgbColor failed: {}", err)
        }
    }
}
impl From<LabColor> for Result<RgbColor, ColorError> {
    fn from(_: LabColor) -> Result<RgbColor, ColorError> {
        // TODO: implement L*a*b -> RGB
        Err(ColorError::Unimplemented)
    }
}
