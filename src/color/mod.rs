use std::fmt;

pub mod palette;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Color {
    // These 3 are specified solely by their name.
    Red,
    Green,
    Blue,
    /// A representation of the RGB (Red, Green, Blue) color format.
    RGB(u8, u8, u8),
    /// A representation of the RGBA (Red, Green, Blue, Alpha) color format.
    RGBA(u8, u8, u8, u8),
    /// A representation of the HSL (Hue, Saturation, Value) color format.
    HSV(u16, u8, u8),
    /// A representation of the HSL (Hue, Saturation, Lightness) color format.
    HSL(u16, u8, u8),
    /// A representation of the CMYK (Cyan, Magenta, Yellow) color format.
    CMY(u8, u8, u8),
    /// A representation of the CMYK (Cyan, Magenta, Yellow, Key) color format.
    CMYK(u8, u8, u8, u8),
}

#[derive(Debug)]
pub enum Error {
    PercentageOverflow,
    DegreeOverflow,
    Unimplemented,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "color(Red)"),
            Color::Green => write!(f, "color(Green)"),
            Color::Blue => write!(f, "color(Blue)"),
            Color::RGB(red, green, blue) => write!(f, "rgb({}, {}, {})", red, green, blue),
            Color::RGBA(red, green, blue, alpha) => {
                write!(f, "rgba({}, {}, {}, {})", red, green, blue, alpha)
            }
            Color::HSV(hue, saturation, value) => {
                write!(f, "hsv({}, {}, {})", hue, saturation, value)
            }
            Color::HSL(hue, saturation, lightness) => {
                write!(f, "hsl({}°, {}%, {}%)", hue, saturation, lightness)
            }
            Color::CMY(cyan, magenta, yellow) => {
                write!(f, "cmy({}%, {}%, {}%)", cyan, magenta, yellow)
            }
            Color::CMYK(cyan, magenta, yellow, key) => {
                write!(f, "cmyk({}%, {}%, {}%, {}%)", cyan, magenta, yellow, key)
            }
        }
    }
}

impl Color {
    pub fn to_rgb(&self) -> Result<Color, Error> {
        match self {
            Color::Red => Ok(Color::RGB(255, 0, 0)),
            Color::Green => Ok(Color::RGB(0, 255, 0)),
            Color::Blue => Ok(Color::RGB(0, 0, 255)),
            Color::RGB(_, _, _) => Ok(self.clone()),
            Color::RGBA(red, green, blue, _) => Ok(Color::RGB(*red, *green, *blue)),
            Color::HSV(_, _, _) => Err(Error::Unimplemented),
            Color::HSL(hue, saturation, lightness) => {
                let c = (1. - ((2. * (*lightness as f64 / 100.)) - 1.).abs())
                    * (*saturation as f64 / 100.);
                let x = c * (1. - ((((*hue as f64) / 60.) % 2.) - 1.).abs());
                let m = (*lightness as f64 / 100.) - (c / 2.);

                let (r_prime, g_prime, b_prime) = match *hue {
                    0..=59 => (c, x, 0.),
                    60..=119 => (x, c, 0.),
                    120..=179 => (0., c, x),
                    180..=239 => (0., x, c),
                    240..=299 => (x, 0., c),
                    300..=360 => (c, 0., x),
                    _ => {
                        return Err(Error::DegreeOverflow)
                    }
                };

                let apply = |v: f64| ((v + m) * 255.).round() as u8;
                let red = apply(r_prime);
                let green = apply(g_prime);
                let blue = apply(b_prime);

                Ok(Color::RGB(red, green, blue))
            }
            Color::CMY(_, _, _) => Err(Error::Unimplemented),
            Color::CMYK(cyan, magenta, yellow, key) => {
                let apply =
                    |v| (255. * (1f64 - v as f64 / 100.) * (1. - *key as f64 / 100.)).round() as u8;

                let red = apply(*cyan);
                let green = apply(*magenta);
                let blue = apply(*yellow);

                Ok(Color::RGB(red, green, blue))
            }
        }
    }

    pub fn to_cmyk(&self) -> Result<Color, Error> {
        unimplemented!()
    }

    pub fn to_hsl(&self) -> Result<Color, Error> {
        unimplemented!()
    }

    pub fn to_hex_string(&self) -> String {
        match self {
            Color::Red => format!("#ff0000"),
            Color::Green => format!("#00ff00"),
            Color::Blue => format!("#0000ff"),
            Color::RGB(red, green, blue) => format!("#{:0>2x}{:0>2x}{:0>2x}", red, green, blue),
            Color::RGBA(red, green, blue, alpha) => {
                format!("#{:0>2x}{:0>2x}{:0>2x}{:0>2x}", red, green, blue, alpha)
            }
            Color::HSV(_, _, _) => {
                self.to_rgb().unwrap().to_hex_string()
            }
            Color::HSL(_, _, _) => {
                self.to_rgb().unwrap().to_hex_string()
            }
            Color::CMY(_, _, _) => {
                self.to_rgb().unwrap().to_hex_string()
            }
            Color::CMYK(_, _, _, _) => {
                self.to_rgb().unwrap().to_hex_string()
            }
        }
    }
}
