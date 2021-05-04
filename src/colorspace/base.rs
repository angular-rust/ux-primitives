use super::*;
use crate::color;
use num_traits::Float;
use std::{default, fmt};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Color<T: Float = f64> {
    pub(crate) red: T,
    pub(crate) green: T,
    pub(crate) blue: T,
    pub(crate) alpha: T,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::new(
            red as f64 / 255.,
            green as f64 / 255.,
            blue as f64 / 255.,
            1.,
        )
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::new(
            red as f64 / 255.,
            green as f64 / 255.,
            blue as f64 / 255.,
            alpha as f64 / 255.,
        )
    }

    pub fn hsl(hue: f64, saturation: f64, lightness: f64) -> Self {
        Self::from_color(HslColor::new(hue, saturation, lightness))
    }

    pub fn hsla(hue: f64, saturation: f64, lightness: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(HslColor::new(hue, saturation, lightness));
        color.alpha = alpha;
        color
    }

    pub fn hsv(hue: f64, saturation: f64, value: f64) -> Self {
        Self::from_color(HslColor::new(hue, saturation, value))
    }

    pub fn hsva(hue: f64, saturation: f64, value: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(HslColor::new(hue, saturation, value));
        color.alpha = alpha;
        color
    }

    pub fn cmyk(cyan: f64, magenta: f64, yellow: f64, key: f64) -> Self {
        Self::from_color(CmykColor::new(cyan, magenta, yellow, key))
    }
    //noinspection SpellCheckingInspection
    pub fn cmyka(cyan: f64, magenta: f64, yellow: f64, key: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(CmykColor::new(cyan, magenta, yellow, key));
        color.alpha = alpha;
        color
    }

    pub fn cmy(cyan: f64, magenta: f64, yellow: f64) -> Self {
        Self::from_color(CmyColor::new(cyan, magenta, yellow))
    }
    //noinspection SpellCheckingInspection
    pub fn cmya(cyan: f64, magenta: f64, yellow: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(CmyColor::new(cyan, magenta, yellow));
        color.alpha = alpha;
        color
    }

    #[cfg(feature = "experimental")]
    pub fn xyz(x: f64, y: f64, z: f64) -> Self {
        Self::from_color(XyzColor::new(x, y, z))
    }
    //noinspection SpellCheckingInspection
    #[cfg(feature = "experimental")]
    pub fn xyza(x: f64, y: f64, z: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(XyzColor::new(x, y, z));
        color.alpha = alpha;
        color
    }

    #[cfg(feature = "experimental")]
    pub fn lab(l: f64, a: f64, b: f64) -> Self {
        Self::from_color(XyzColor::new(l, a, b))
    }
    //noinspection SpellCheckingInspection
    #[cfg(feature = "experimental")]
    pub fn laba(l: f64, a: f64, b: f64, alpha: f64) -> Self {
        let mut color = Self::from_color(XyzColor::new(l, a, b));
        color.alpha = alpha;
        color
    }

    // EMULATE creation of unicolor::Color enum

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn RGB(red: u8, green: u8, blue: u8) -> Self {
        Self::rgb(red, green, blue)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn RGBA(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::rgba(red, green, blue, alpha)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn HSL(hue: f64, saturation: f64, lightness: f64) -> Self {
        Self::hsl(hue, saturation, lightness)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn HSV(hue: f64, saturation: f64, value: f64) -> Self {
        Self::hsv(hue, saturation, value)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn CMYK(cyan: f64, magenta: f64, yellow: f64, key: f64) -> Self {
        Self::cmyk(cyan, magenta, yellow, key)
    }

    #[allow(non_snake_case)]
    #[deprecated]
    pub fn CMY(cyan: f64, magenta: f64, yellow: f64) -> Self {
        Self::cmy(cyan, magenta, yellow)
    }

    #[cfg(feature = "experimental")]
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn XYZ(x: f64, y: f64, z: f64) -> Self {
        Self::xyz(x, y, z)
    }

    #[cfg(feature = "experimental")]
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn LAB(l: f64, a: f64, b: f64) -> Self {
        Self::lab(l, a, b)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl default::Default for Color {
    fn default() -> Self {
        color::BLACK
    }
}

#[derive(Debug)]
pub enum ColorError {
    PercentageOverflow,
    DegreeOverflow,
    Unimplemented,
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PercentageOverflow => write!(
                f,
                "Overflow of Color percentage value (can't be greater than 100%)"
            ),
            Self::DegreeOverflow => write!(
                f,
                "Overflow of Hue in hsl(v) color space (can't be greater than 360 deg"
            ),
            Self::Unimplemented => write!(f, "Unimplemented color conversion"),
        }
    }
}

#[cfg(any(feature = "color_quantization", test))]
impl Color {
    pub fn distance(&self, other: Self) -> f64 {
        let RgbColor {
            red: s_red,
            green: s_green,
            blue: s_blue,
        } = (*self).into();
        let RgbColor {
            red: p_red,
            green: p_green,
            blue: p_blue,
        } = other.into();
        (((p_red as i32 - s_red as i32).pow(2)
            + (p_green as i32 - s_green as i32).pow(2)
            + (p_blue as i32 - s_blue as i32).pow(2)) as f64)
            .sqrt()
            .abs()
    }

    pub fn quantize(&self) -> Self {
        let mut min_color_distance =
            ((0xFF_u32.pow(2) + 0xFF_u32.pow(2) + 0xFF_u32.pow(2)) as f64).sqrt();
        let mut min_distance_color: Option<&Color> = None;
        for color in color::PALETTE.iter() {
            let color_distance = self.distance(*color);
            if color_distance < min_color_distance {
                min_color_distance = color_distance;
                min_distance_color = Some(color);
            }
        }
        *min_distance_color.expect("In this palette not found color which distance is smaller than distance from black to white")
    }
}

#[cfg(test)]
pub mod test {
    use super::super::RgbColor;
    use super::*;
    use math::round::stochastic;

    #[test]
    fn calc_distance() {
        //println!("distance: YELLOW_0 -> LINE_0 = {}", YELLOW_0.distance(LIME_0));
        assert_eq!(
            stochastic(color::YELLOW_0.distance(color::LIME_0), 12),
            13.928388277184
        );

        let stochastic_scale = 10;
        for delta in [2u8, 3u8, 4u8].iter() {
            let delta = *delta;
            for src_color in [
                color::TEAL_1,
                color::TEAL_2,
                color::TEAL_3,
                color::TEAL_4,
                color::TEAL_5,
                color::TEAL_6,
                color::TEAL_7,
                color::TEAL_8,
            ]
            .iter()
            {
                let RgbColor {
                    red: s_red,
                    green: s_green,
                    blue: s_blue,
                } = (*src_color).into();
                let dst_color = Color::rgb(s_red + delta, s_green + delta, s_blue + delta);
                assert_eq!(
                    stochastic(src_color.distance(dst_color), stochastic_scale),
                    stochastic(
                        (((delta as i32).pow(2) + (delta as i32).pow(2) + (delta as i32).pow(2))
                            as f64)
                            .sqrt(),
                        stochastic_scale
                    )
                )
            }
        }
    }

    #[test]
    fn quantization() {
        for delta in [2u8, 3u8, 4u8].iter() {
            let delta = *delta;
            for palette_color in [
                color::CYAN_2,
                color::CYAN_3,
                color::CYAN_4,
                color::CYAN_5,
                color::CYAN_6,
                color::CYAN_7,
            ]
            .iter()
            {
                let RgbColor {
                    red: p_red,
                    green: p_green,
                    blue: p_blue,
                } = (*palette_color).into();
                let test_color = Color::rgb(p_red + delta, p_green + delta, p_blue + delta);
                // println!("test_color = {}", test_color.to_hex_string());
                let found_color = test_color.quantize();
                // println!("test_color = {}; found_color = {} == {} ?",
                //          test_color.to_hex_string(),
                //          found_color.to_hex_string(),
                //          palette_color.to_hex_string());
                let RgbColor {
                    red: f_red,
                    green: f_green,
                    blue: f_blue,
                } = found_color.into();
                assert_eq!(f_red, p_red);
                assert_eq!(f_green, p_green);
                assert_eq!(f_blue, p_blue);
            }
        }
    }
}
