use super::Color;
use super::palette::*;

pub fn palette_vec() -> Vec<Color> {
    vec!(
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
    )
}

pub fn color_from_short_rgb_u16(c: u16) -> Color {
    let (red, green, blue, _) = (
        ((c >> 8) + ((c >> 8) << 4)) as u8,
        ((c & 0x0f0) + ((c & 0x0f0) >> 4)) as u8,
        ((c & 0x00f) + ((c & 0x00f) << 4)) as u8,
        0xff
    );
    Color::RGB(red, green, blue)
}

pub fn color_from_rgb_u32(c: u32) -> Color {
    let (red, green, blue, _) = (
        (c >> 16) as u8,
        ((c & 0x00ff00) >> 8) as u8,
        (c & 0x0000ff) as u8,
        0xff
    );
    Color::RGB(red, green, blue)
}

pub fn color_from_short_rgba_u16(c: u16) -> Color {
    let (red, green, blue, alpha) = (
        ((c >> 12) + ((c >> 12) << 4)) as u8,
        (((c & 0x0f00) >> 4) + ((c & 0x0f00) >> 8)) as u8,
        ((c & 0x00f0) + ((c & 0x00f0) >> 4)) as u8,
        ((c & 0x000f) + ((c & 0x000f) << 4)) as u8
    );
    Color::RGBA(red, green, blue, alpha)
}

pub fn color_from_rgba_u32(c: u32) -> Color {
    let (red, green, blue, alpha) = (
        (c >> 24) as u8,
        ((c & 0x00ff0000) >> 16) as u8,
        ((c & 0x0000ff00) >> 8) as u8,
        (c & 0x000000ff) as u8
    );
    Color::RGBA(red, green, blue, alpha)
}
