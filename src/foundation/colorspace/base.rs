use std::{default, fmt};

use crate::prelude::color;

use super::prelude::*;

use super::*;

/// Basic color representation
#[derive(Clone, Copy, PartialEq, Debug)] // Eq, Hash 
#[repr(C)]
pub struct Color {
    /// Red component
    pub red: Float,
    /// Green component
    pub green: Float,
    /// Blue component
    pub blue: Float,
    /// Alpha component
    pub alpha: Float,
}

impl Color {
    /// Create color with floating point components
    pub fn new(red: Float, green: Float, blue: Float, alpha: Float) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Create solid color with byte components
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::new(
            red as Float / 255.,
            green as Float / 255.,
            blue as Float / 255.,
            1.,
        )
    }

    /// Create color with byte components
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::new(
            red as Float / 255.,
            green as Float / 255.,
            blue as Float / 255.,
            alpha as Float / 255.,
        )
    }

    /// Create solid color with using hsl color space
    pub fn hsl(hue: Float, saturation: Float, lightness: Float) -> Self {
        Self::from_color(HslColor::new(hue, saturation, lightness))
    }

    /// Create solid color with using hsl color space and alpha component
    pub fn hsla(hue: Float, saturation: Float, lightness: Float, alpha: Float) -> Self {
        let mut color = Self::from_color(HslColor::new(hue, saturation, lightness));
        color.alpha = alpha;
        color
    }

    /// Create solid color with using hsv color space
    pub fn hsv(hue: Float, saturation: Float, value: Float) -> Self {
        Self::from_color(HslColor::new(hue, saturation, value))
    }

    /// Create solid color with using hsv color space and alpha component
    pub fn hsva(hue: Float, saturation: Float, value: Float, alpha: Float) -> Self {
        let mut color = Self::from_color(HslColor::new(hue, saturation, value));
        color.alpha = alpha;
        color
    }

    /// Create solid color with using cmyk color space
    pub fn cmyk(cyan: Float, magenta: Float, yellow: Float, key: Float) -> Self {
        Self::from_color(CmykColor::new(cyan, magenta, yellow, key))
    }
    
    /// Create solid color with using cmyk color space and alpha component
    pub fn cmyka(cyan: Float, magenta: Float, yellow: Float, key: Float, alpha: Float) -> Self {
        let mut color = Self::from_color(CmykColor::new(cyan, magenta, yellow, key));
        color.alpha = alpha;
        color
    }

    /// Create solid color with using cmy color space
    pub fn cmy(cyan: Float, magenta: Float, yellow: Float) -> Self {
        Self::from_color(CmyColor::new(cyan, magenta, yellow))
    }
    
    /// Create solid color with using cmy color space and alpha component
    pub fn cmya(cyan: Float, magenta: Float, yellow: Float, alpha: Float) -> Self {
        let mut color = Self::from_color(CmyColor::new(cyan, magenta, yellow));
        color.alpha = alpha;
        color
    }

    /// Create solid color with using xyz color space
    #[cfg(feature = "experimental")]
    pub fn xyz(x: Float, y: Float, z: Float) -> Self {
        Self::from_color(XyzColor::new(x, y, z))
    }
    
    /// Create solid color with using xyz color space and alpha component
    #[cfg(feature = "experimental")]
    pub fn xyza(x: Float, y: Float, z: Float, alpha: Float) -> Self {
        let mut color = Self::from_color(XyzColor::new(x, y, z));
        color.alpha = alpha;
        color
    }

    /// Create solid color with using lab color space
    #[cfg(feature = "experimental")]
    pub fn lab(l: Float, a: Float, b: Float) -> Self {
        Self::from_color(XyzColor::new(l, a, b))
    }
    
    /// Create solid color with using lab color space and alpha component
    #[cfg(feature = "experimental")]
    pub fn laba(l: Float, a: Float, b: Float, alpha: Float) -> Self {
        let mut color = Self::from_color(XyzColor::new(l, a, b));
        color.alpha = alpha;
        color
    }

    // EMULATE creation of unicolor::Color enum

    /// Create solid color with using rgb color space
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn RGB(red: u8, green: u8, blue: u8) -> Self {
        Self::rgb(red, green, blue)
    }

    /// Create solid color with using lab rgb space and alpha component
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn RGBA(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::rgba(red, green, blue, alpha)
    }

    /// Create solid color with using hsl color space
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn HSL(hue: Float, saturation: Float, lightness: Float) -> Self {
        Self::hsl(hue, saturation, lightness)
    }

    /// Create solid color with using hsv color space
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn HSV(hue: Float, saturation: Float, value: Float) -> Self {
        Self::hsv(hue, saturation, value)
    }

    /// Create solid color with using cmyk color space
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn CMYK(cyan: Float, magenta: Float, yellow: Float, key: Float) -> Self {
        Self::cmyk(cyan, magenta, yellow, key)
    }

    /// Create solid color with using cmy color space
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn CMY(cyan: Float, magenta: Float, yellow: Float) -> Self {
        Self::cmy(cyan, magenta, yellow)
    }

    /// Create solid color with using xyz color space
    #[cfg(feature = "experimental")]
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn XYZ(x: Float, y: Float, z: Float) -> Self {
        Self::xyz(x, y, z)
    }

    /// Create solid color with using lab color space
    #[cfg(feature = "experimental")]
    #[allow(non_snake_case)]
    #[deprecated]
    pub fn LAB(l: Float, a: Float, b: Float) -> Self {
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


/// Represents color error
#[derive(Debug)]
pub enum ColorError {
    /// Percentage overflow error
    PercentageOverflow,
    /// Degree overflow error
    DegreeOverflow,
    /// Unimplementer error
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
