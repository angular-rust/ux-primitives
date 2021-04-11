use super::Color;

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
