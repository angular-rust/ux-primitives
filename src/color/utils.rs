use super::Color;

#[inline]
pub fn normalize_hue(hue: f64) -> f64 {
    let hue_rest = hue % 360.0;
    if hue_rest < 0.0 { 360.0 - hue_rest } else { hue_rest }
}

#[inline]
pub fn percentage_to_fraction(v: f64) -> f64 {
    if v > 100. { 1. }
    else if v < 0. { 0. }
    else { v / 100. }
}

#[inline]
pub fn color_from_short_rgb_u16(c: u16) -> Color {
    let (red, green, blue, _) = (
        ((c >> 8) + ((c >> 8) << 4)) as u8,
        ((c & 0x0f0) + ((c & 0x0f0) >> 4)) as u8,
        ((c & 0x00f) + ((c & 0x00f) << 4)) as u8,
        0xff
    );
    Color::RGB(red, green, blue)
}

#[inline]
pub fn color_from_rgb_u32(c: u32) -> Color {
    let (red, green, blue, _) = (
        (c >> 16) as u8,
        ((c & 0x00ff00) >> 8) as u8,
        (c & 0x0000ff) as u8,
        0xff
    );
    Color::RGB(red, green, blue)
}

#[inline]
pub fn color_from_short_rgba_u16(c: u16) -> Color {
    let (red, green, blue, alpha) = (
        ((c >> 12) + ((c >> 12) << 4)) as u8,
        (((c & 0x0f00) >> 4) + ((c & 0x0f00) >> 8)) as u8,
        ((c & 0x00f0) + ((c & 0x00f0) >> 4)) as u8,
        ((c & 0x000f) + ((c & 0x000f) << 4)) as u8
    );
    Color::RGBA(red, green, blue, alpha)
}

#[inline]
pub fn color_from_rgba_u32(c: u32) -> Color {
    let (red, green, blue, alpha) = (
        (c >> 24) as u8,
        ((c & 0x00ff0000) >> 16) as u8,
        ((c & 0x0000ff00) >> 8) as u8,
        (c & 0x000000ff) as u8
    );
    Color::RGBA(red, green, blue, alpha)
}
