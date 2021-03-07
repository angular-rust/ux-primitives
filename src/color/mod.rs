use std::{convert::From, fmt};

pub mod palette;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

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

impl From<Color> for RgbColor {
    fn from(item: Color) -> Self {
        if let Color::RGB(r, g, b) = item.to_rgb().unwrap() {
            RgbColor { r, g, b }
        } else {
            unreachable!()
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
                    _ => return Err(Error::DegreeOverflow),
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
        match self {
            Color::Red => Color::RGB(255, 0, 0).to_cmyk(),
            Color::Green => Color::RGB(0, 255, 0).to_cmyk(),
            Color::Blue => Color::RGB(0, 0, 255).to_cmyk(),
            Color::RGB(red, green, blue) => {
                let r_prime = *red as f64 / 255.;
                let g_prime = *green as f64 / 255.;
                let b_prime = *blue as f64 / 255.;

                let key = 1.
                    - [r_prime, g_prime, b_prime]
                        .iter()
                        .cloned()
                        .fold(f64::NAN, f64::max);

                let apply = |v: f64| (((1. - v - key) / (1. - key)) * 100.).round();
                let cyan = apply(r_prime);
                let magenta = apply(g_prime);
                let yellow = apply(b_prime);

                Ok(Color::CMYK(cyan as u8, magenta as u8, yellow as u8, (key * 100.) as u8))
            }
            Color::RGBA(_, _, _, _) => self.to_rgb().unwrap().to_cmyk(),
            Color::HSV(_, _, _) => Err(Error::Unimplemented),
            Color::HSL(_, _, _) => self.to_rgb().unwrap().to_cmyk(),
            Color::CMY(_, _, _) => Err(Error::Unimplemented),
            Color::CMYK(_, _, _, _) => Ok(self.clone()),
        }
    }

    pub fn to_hsl(&self) -> Result<Color, Error> {
        match self {
            Color::Red => Color::RGB(255, 0, 0).to_hsl(),
            Color::Green => Color::RGB(0, 255, 0).to_hsl(),
            Color::Blue => Color::RGB(0, 0, 255).to_hsl(),
            Color::RGB(red, green, blue) => {
                let r_prime = *red as f64 / 255.;
                let g_prime = *green as f64 / 255.;
                let b_prime = *blue as f64 / 255.;

                let c_max = [*red, *green, *blue].iter().max().cloned().unwrap() as f64 / 255.;
                let c_min = [*red, *green, *blue].iter().min().cloned().unwrap() as f64 / 255.;

                let delta = c_max - c_min;

                let hue = if (delta - 0.) < f64::EPSILON {
                    0
                } else {
                    match c_max {
                        x if x == r_prime => 60. * (((g_prime - b_prime) / delta) % 6.),
                        x if x == g_prime => 60. * (((b_prime - r_prime) / delta) + 2.),
                        x if x == b_prime => 60. * (((r_prime - g_prime) / delta) + 4.),
                        _ => panic!("Invalid hue calculation!"),
                    }
                    .round() as u16
                };

                let lightness = (c_max + c_min) / 2.;

                let saturation = if (delta - 0.) < f64::EPSILON {
                    0
                } else {
                    (delta / (1. - ((2. * lightness) - 1.)) * 100.).round() as u8
                };

                Ok(Color::HSL(
                    hue,
                    saturation,
                    (lightness * 100.).round() as u8,
                ))
            }
            Color::RGBA(_, _, _, _) => self.to_rgb().unwrap().to_hsl(),
            Color::HSV(_, _, _) => self.to_rgb().unwrap().to_hsl(),
            Color::HSL(_, _, _) => Ok(self.clone()),
            Color::CMY(_, _, _) => self.to_rgb().unwrap().to_hsl(),
            Color::CMYK(_, _, _, _) => self.to_rgb().unwrap().to_hsl(),
        }
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
            Color::HSV(_, _, _) => self.to_rgb().unwrap().to_hex_string(),
            Color::HSL(_, _, _) => self.to_rgb().unwrap().to_hex_string(),
            Color::CMY(_, _, _) => self.to_rgb().unwrap().to_hex_string(),
            Color::CMYK(_, _, _, _) => self.to_rgb().unwrap().to_hex_string(),
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;

	#[test]
	fn test_rgb_to_string() {
		// let rgb = Rgb::new(30, 50, 60);
		// assert_eq!(rgb.to_string(), String::from("rgb(30, 50, 60)"));
	}

	#[test]
	fn test_rgb_to_hex_string() {
		// let hex = Rgb::new(30, 50, 60).to_hex_string();
		// assert_eq!(hex, String::from("#1e323c"));
	}

	#[test]
	fn test_rgb_to_cmyk() {
		// let rgb = Rgb::new(30, 50, 60).to_cmyk();
		// assert_eq!(rgb, Cmyk::new_unchecked(50, 17, 0, 76));
	}

	#[test]
	fn test_rgb_to_hsl() {
		// let hsl = Rgb::new(204, 153, 102).to_hsl();
		// assert_eq!(hsl, Hsl::new_unchecked(30, 50, 60));
	}

    #[test]
	fn test_hsl_to_string() {
		// let hsl = Hsl::new_unchecked(100, 100, 100);
		// assert_eq!(hsl.to_string(), String::from("hsl(100°, 100%, 100%)"));
	}

	#[test]
	fn test_hsl_to_hex_string() {
		// let hex = Hsl::new_unchecked(30, 50, 60).to_hex_string();
		// assert_eq!(hex, String::from("#cc9966"));
	}

	#[test]
	fn test_hsl_to_rgb() {
		// let rgb = Hsl::new_unchecked(30, 50, 60).to_rgb();
		// assert_eq!(rgb, Rgb::new(204, 153, 102));
	}

	#[should_panic]
	#[test]
	fn test_hsl_checked_hsl() {
		// Hsl::new(361, 101, 101).unwrap();
	}

    #[test]
	fn test_cmyk_to_string() {
		// let cmyk = Cmyk::new(30, 50, 60, 40).unwrap();
		// assert_eq!(cmyk.to_string(), String::from("cmyk(30%, 50%, 60%, 40%)"));
	}

	#[test]
	fn test_cmyk_to_hex_string() {
		// let hex = Cmyk::new(30, 50, 60, 40).unwrap().to_hex_string();
		// assert_eq!(hex, String::from("#6b4d3d"));
	}

	#[test]
	fn test_cmyk_to_rgb() {
		// let hex = Cmyk::new(30, 50, 60, 40).unwrap().to_rgb();
		// assert_eq!(hex, Rgb::new(107, 77, 61));
	}

	#[should_panic]
	#[test]
	fn test_cmyk_checked_cmyk() {
		// Cmyk::new(255, 255, 255, 255).unwrap();
	}
}