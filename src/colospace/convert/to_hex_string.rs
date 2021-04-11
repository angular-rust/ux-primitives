use super::*;

pub trait ToHexString {
    fn to_hex_string(&self) -> String;
}

impl<C: Into<RgbColor> + Clone> ToHexString for C {
    fn to_hex_string(&self) -> String {
        let RgbColor {r, g, b} = self.clone().into();
        format!("#{:0>2x}{:0>2x}{:0>2x}", r, g, b)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref TEST_DATA: Vec<(Color, &'static str)> = {//, CmykColor, HslColor)> = {
            vec!(
                (Color::RGB(56, 217, 169), "#38d9a9"),//, CmykColor::new(74.0, 0.0, 22.0, 15.0), HslColor::new(162.0, 68.0, 54.0)),
                (Color::RGB(178, 242, 187), "#b2f2bb"),//, CmykColor::new(26.0, 0.0, 23.0, 5.0), HslColor::new(128.0, 71.0, 82.0)),
                (Color::RGB(230, 252, 245), "#e6fcf5"),//, CmykColor::new(9.0, 0.0, 3.0, 1.0), HslColor::new(161.0, 79.0, 95.0)),
                (Color::RGB(18, 184, 134), "#12b886"),//, CmykColor::new(90.0, 0.0, 27.0, 28.0), HslColor::new(162.0, 82.0, 40.0)),
                //(Color::RGB(___), "#______", CmykColor::new(___), HslColor::new(___)),
            )
        };
    }

    #[test]
    fn from_rgb() {
        for (color, hex_str) in TEST_DATA.iter() {
            assert_eq!(color.to_hex_string(), String::from(*hex_str));
        }
    }
}