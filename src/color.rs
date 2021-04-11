use super::Color;
#[cfg(feature = "color_quantization")]
use super::RgbColor;

pub const WHITE: Color = Color::RGB(255, 255, 255);
pub const BLACK: Color = Color::RGB(0, 0, 0);

pub const GRAY_0: Color = Color::RGB(248, 249, 250);
pub const GRAY_1: Color = Color::RGB(241, 243, 245);
pub const GRAY_2: Color = Color::RGB(233, 236, 239);
pub const GRAY_3: Color = Color::RGB(222, 226, 230);
pub const GRAY_4: Color = Color::RGB(206, 212, 218);
pub const GRAY_5: Color = Color::RGB(173, 181, 189);
pub const GRAY_6: Color = Color::RGB(134, 142, 150);
pub const GRAY_7: Color = Color::RGB(73, 80, 87);
pub const GRAY_8: Color = Color::RGB(52, 58, 64);
pub const GRAY_9: Color = Color::RGB(33, 37, 41);

pub const RED_0: Color = Color::RGB(255, 245, 245);
pub const RED_1: Color = Color::RGB(255, 227, 227);
pub const RED_2: Color = Color::RGB(255, 201, 201);
pub const RED_3: Color = Color::RGB(255, 168, 168);
pub const RED_4: Color = Color::RGB(255, 135, 135);
pub const RED_5: Color = Color::RGB(255, 107, 107);
pub const RED_6: Color = Color::RGB(250, 82, 82);
pub const RED_7: Color = Color::RGB(240, 62, 62);
pub const RED_8: Color = Color::RGB(224, 49, 49);
pub const RED_9: Color = Color::RGB(201, 42, 42);

pub const PINK_0: Color = Color::RGB(255, 240, 246);
pub const PINK_1: Color = Color::RGB(255, 222, 235);
pub const PINK_2: Color = Color::RGB(252, 194, 215);
pub const PINK_3: Color = Color::RGB(250, 162, 193);
pub const PINK_4: Color = Color::RGB(247, 131, 172);
pub const PINK_5: Color = Color::RGB(240, 101, 149);
pub const PINK_6: Color = Color::RGB(230, 73, 128);
pub const PINK_7: Color = Color::RGB(214, 51, 108);
pub const PINK_8: Color = Color::RGB(194, 37, 92);
pub const PINK_9: Color = Color::RGB(166, 30, 77);

pub const GRAPE_0: Color = Color::RGB(248, 240, 252);
pub const GRAPE_1: Color = Color::RGB(243, 217, 250);
pub const GRAPE_2: Color = Color::RGB(238, 190, 250);
pub const GRAPE_3: Color = Color::RGB(229, 153, 247);
pub const GRAPE_4: Color = Color::RGB(218, 119, 242);
pub const GRAPE_5: Color = Color::RGB(204, 93, 232);
pub const GRAPE_6: Color = Color::RGB(190, 75, 219);
pub const GRAPE_7: Color = Color::RGB(174, 62, 201);
pub const GRAPE_8: Color = Color::RGB(156, 54, 181);
pub const GRAPE_9: Color = Color::RGB(134, 46, 156);

pub const VIOLET_0: Color = Color::RGB(243, 240, 255);
pub const VIOLET_1: Color = Color::RGB(229, 219, 255);
pub const VIOLET_2: Color = Color::RGB(208, 191, 255);
pub const VIOLET_3: Color = Color::RGB(177, 151, 252);
pub const VIOLET_4: Color = Color::RGB(151, 117, 250);
pub const VIOLET_5: Color = Color::RGB(132, 94, 247);
pub const VIOLET_6: Color = Color::RGB(121, 80, 242);
pub const VIOLET_7: Color = Color::RGB(112, 72, 232);
pub const VIOLET_8: Color = Color::RGB(103, 65, 217);
pub const VIOLET_9: Color = Color::RGB(95, 61, 196);

pub const INDIGO_0: Color = Color::RGB(237, 242, 255);
pub const INDIGO_1: Color = Color::RGB(219, 228, 255);
pub const INDIGO_2: Color = Color::RGB(186, 200, 255);
pub const INDIGO_3: Color = Color::RGB(145, 167, 255);
pub const INDIGO_4: Color = Color::RGB(116, 143, 252);
pub const INDIGO_5: Color = Color::RGB(92, 124, 250);
pub const INDIGO_6: Color = Color::RGB(76, 110, 245);
pub const INDIGO_7: Color = Color::RGB(66, 99, 235);
pub const INDIGO_8: Color = Color::RGB(59, 91, 219);
pub const INDIGO_9: Color = Color::RGB(54, 79, 199);

pub const BLUE_0: Color = Color::RGB(231, 245, 255);
pub const BLUE_1: Color = Color::RGB(208, 235, 255);
pub const BLUE_2: Color = Color::RGB(165, 216, 255);
pub const BLUE_3: Color = Color::RGB(116, 192, 252);
pub const BLUE_4: Color = Color::RGB(77, 171, 247);
pub const BLUE_5: Color = Color::RGB(51, 154, 240);
pub const BLUE_6: Color = Color::RGB(34, 139, 230);
pub const BLUE_7: Color = Color::RGB(28, 126, 214);
pub const BLUE_8: Color = Color::RGB(25, 113, 194);
pub const BLUE_9: Color = Color::RGB(24, 100, 171);

pub const CYAN_0: Color = Color::RGB(227, 250, 252);
pub const CYAN_1: Color = Color::RGB(197, 246, 250);
pub const CYAN_2: Color = Color::RGB(153, 233, 242);
pub const CYAN_3: Color = Color::RGB(102, 217, 232);
pub const CYAN_4: Color = Color::RGB(59, 201, 219);
pub const CYAN_5: Color = Color::RGB(34, 184, 207);
pub const CYAN_6: Color = Color::RGB(21, 170, 191);
pub const CYAN_7: Color = Color::RGB(16, 152, 173);
pub const CYAN_8: Color = Color::RGB(12, 133, 153);
pub const CYAN_9: Color = Color::RGB(11, 114, 133);

pub const TEAL_0: Color = Color::RGB(230, 252, 245);
pub const TEAL_1: Color = Color::RGB(195, 250, 232);
pub const TEAL_2: Color = Color::RGB(150, 242, 215);
pub const TEAL_3: Color = Color::RGB(99, 230, 190);
pub const TEAL_4: Color = Color::RGB(56, 217, 169);
pub const TEAL_5: Color = Color::RGB(32, 201, 151);
pub const TEAL_6: Color = Color::RGB(18, 184, 134);
pub const TEAL_7: Color = Color::RGB(12, 166, 120);
pub const TEAL_8: Color = Color::RGB(9, 146, 104);
pub const TEAL_9: Color = Color::RGB(8, 127, 91);

pub const GREEN_0: Color = Color::RGB(235, 251, 238);
pub const GREEN_1: Color = Color::RGB(211, 249, 216);
pub const GREEN_2: Color = Color::RGB(178, 242, 187);
pub const GREEN_3: Color = Color::RGB(140, 233, 154);
pub const GREEN_4: Color = Color::RGB(105, 219, 124);
pub const GREEN_5: Color = Color::RGB(81, 207, 102);
pub const GREEN_6: Color = Color::RGB(64, 192, 87);
pub const GREEN_7: Color = Color::RGB(55, 178, 77);
pub const GREEN_8: Color = Color::RGB(47, 158, 68);
pub const GREEN_9: Color = Color::RGB(43, 138, 62);

pub const LIME_0: Color = Color::RGB(244, 252, 227);
pub const LIME_1: Color = Color::RGB(233, 250, 200);
pub const LIME_2: Color = Color::RGB(216, 245, 162);
pub const LIME_3: Color = Color::RGB(192, 235, 117);
pub const LIME_4: Color = Color::RGB(169, 227, 75);
pub const LIME_5: Color = Color::RGB(148, 216, 45);
pub const LIME_6: Color = Color::RGB(130, 201, 30);
pub const LIME_7: Color = Color::RGB(116, 184, 22);
pub const LIME_8: Color = Color::RGB(102, 168, 15);
pub const LIME_9: Color = Color::RGB(92, 148, 13);

pub const YELLOW_0: Color = Color::RGB(255, 249, 219);
pub const YELLOW_1: Color = Color::RGB(255, 243, 191);
pub const YELLOW_2: Color = Color::RGB(255, 236, 153);
pub const YELLOW_3: Color = Color::RGB(255, 224, 102);
pub const YELLOW_4: Color = Color::RGB(255, 212, 59);
pub const YELLOW_5: Color = Color::RGB(252, 196, 25);
pub const YELLOW_6: Color = Color::RGB(250, 176, 5);
pub const YELLOW_7: Color = Color::RGB(245, 159, 0);
pub const YELLOW_8: Color = Color::RGB(240, 140, 0);
pub const YELLOW_9: Color = Color::RGB(230, 119, 0);

pub const ORANGE_0: Color = Color::RGB(255, 244, 230);
pub const ORANGE_1: Color = Color::RGB(255, 232, 204);
pub const ORANGE_2: Color = Color::RGB(255, 216, 168);
pub const ORANGE_3: Color = Color::RGB(255, 192, 120);
pub const ORANGE_4: Color = Color::RGB(255, 169, 77);
pub const ORANGE_5: Color = Color::RGB(255, 146, 43);
pub const ORANGE_6: Color = Color::RGB(253, 126, 20);
pub const ORANGE_7: Color = Color::RGB(247, 103, 7);
pub const ORANGE_8: Color = Color::RGB(232, 89, 12);
pub const ORANGE_9: Color = Color::RGB(217, 72, 15);

#[cfg(feature = "color_quantization")]
lazy_static! {
    static ref PALETTE: Vec<Color> = vec!(
        BLACK, WHITE,
        GRAY_0, GRAY_1, GRAY_2, GRAY_3, GRAY_4, GRAY_5, GRAY_6, GRAY_7, GRAY_8, GRAY_9,
        RED_0, RED_1, RED_2, RED_3, RED_4, RED_5, RED_6, RED_7, RED_8, RED_9,
        PINK_0, PINK_1, PINK_2, PINK_3, PINK_4, PINK_5, PINK_6, PINK_7, PINK_8, PINK_9,
        GRAPE_0, GRAPE_1, GRAPE_2, GRAPE_3, GRAPE_4, GRAPE_5, GRAPE_6, GRAPE_7, GRAPE_8, GRAPE_9,
        VIOLET_0, VIOLET_1, VIOLET_2, VIOLET_3, VIOLET_4, VIOLET_5, VIOLET_6, VIOLET_7, VIOLET_8, VIOLET_9,
        INDIGO_0, INDIGO_1, INDIGO_2, INDIGO_3, INDIGO_4, INDIGO_5, INDIGO_6, INDIGO_7, INDIGO_8, INDIGO_9,
        BLUE_0, BLUE_1, BLUE_2, BLUE_3, BLUE_4, BLUE_5, BLUE_6, BLUE_7, BLUE_8, BLUE_9,
        CYAN_0, CYAN_1, CYAN_2, CYAN_3, CYAN_4, CYAN_5, CYAN_6, CYAN_7, CYAN_8, CYAN_9,
        TEAL_0, TEAL_1, TEAL_2, TEAL_3, TEAL_4, TEAL_5, TEAL_6, TEAL_7, TEAL_8, TEAL_9,
        GREEN_0, GREEN_1, GREEN_2, GREEN_3, GREEN_4, GREEN_5, GREEN_6, GREEN_7, GREEN_8, GREEN_9,
        LIME_0, LIME_1, LIME_2, LIME_3, LIME_4, LIME_5, LIME_6, LIME_7, LIME_8, LIME_9,
        YELLOW_0, YELLOW_1, YELLOW_2, YELLOW_3, YELLOW_4, YELLOW_5, YELLOW_6, YELLOW_7, YELLOW_8, YELLOW_9,
        ORANGE_0, ORANGE_1, ORANGE_2, ORANGE_3, ORANGE_4, ORANGE_5, ORANGE_6, ORANGE_7, ORANGE_8, ORANGE_9,
    );
}

#[cfg(feature = "color_quantization")]
impl Color {
    pub fn distance(&self, other: Self) -> f64 {
        let RgbColor { r: s_red, g: s_green, b: s_blue } = (*self).into();
        if let Self::RGB(p_red, p_green, p_blue) = other {
            (((p_red as i32 - s_red as i32).pow(2)
                + (p_green as i32 - s_green as i32).pow(2)
                + (p_blue as i32 - s_blue as i32).pow(2)
            ) as f64).sqrt().abs()
        } else { panic!("Palette color should be RGB") }
    }

    fn quantize(&self) -> Self {
        let mut min_color_distance = ((0xFF_u32.pow(2) + 0xFF_u32.pow(2) + 0xFF_u32.pow(2)) as f64).sqrt();
        let mut min_distance_color: Option<&Color> = None;
        for color in PALETTE.iter() {
            let color_distance = self.distance(*color);
            if color_distance < min_color_distance {
                min_color_distance = color_distance;
                min_distance_color = Some(color);
            }
        }
        *min_distance_color.expect("In this palette not found color which distance is smaller than distance from black to white")
    }
}

#[cfg(feature = "color_quantization")]
#[cfg(test)]
pub mod test {
    use super::*;
    use math::round::stochastic;

    #[test]
    fn calc_distance() {
        //println!("distance: YELLOW_0 -> LINE_0 = {}", YELLOW_0.distance(LIME_0));
        assert_eq!(stochastic(YELLOW_0.distance(LIME_0), 12), 13.928388277184);

        let stochastic_scale= 10;
        for delta in [2u8, 3u8, 4u8].iter() {
            let delta = *delta;
            for src_color in [TEAL_1, TEAL_2, TEAL_3, TEAL_4, TEAL_5, TEAL_6, TEAL_7, TEAL_8].iter() {
                if let Color::RGB(s_red, s_green, s_blue) = *src_color {
                    let dst_color = Color::RGB(s_red + delta, s_green + delta, s_blue + delta);
                    assert_eq!(
                        stochastic(src_color.distance(dst_color), stochastic_scale),
                        stochastic(
                            (((delta as i32).pow(2)
                                + (delta as i32).pow(2)
                                + (delta as i32).pow(2)) as f64).sqrt(),
                            stochastic_scale
                        )
                    )
                } else { unreachable!() }
            }
        }
    }

    #[test]
    fn quantization() {
        for delta in [2u8, 3u8, 4u8].iter() {
            let delta = *delta;
            for palette_color in [CYAN_2, CYAN_3, CYAN_4, CYAN_5, CYAN_6, CYAN_7].iter() {
                if let Color::RGB(p_red, p_green, p_blue) = *palette_color {
                    let test_color = Color::RGB(p_red + delta, p_green + delta, p_blue + delta);
                    // println!("test_color = {}", test_color.to_hex_string());
                    let found_color = test_color.quantize();
                    // println!("test_color = {}; found_color = {} == {} ?",
                    //          test_color.to_hex_string(),
                    //          found_color.to_hex_string(),
                    //          palette_color.to_hex_string());
                    if let Color::RGB(f_red, f_green, f_blue) = found_color {
                        assert_eq!(f_red, p_red);
                        assert_eq!(f_green, p_green);
                        assert_eq!(f_blue, p_blue);
                    } else { unreachable!() }
                } else { unreachable!() }
            }
        }
    }
}
