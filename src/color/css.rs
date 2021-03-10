use super::Color;

impl Color {
    fn from_hex_str(color: &str) -> Color {
        let color= &color[0..9];
        let panic_string = format!("Color hex invalid format of string \"{}\"", color);
        if !color.starts_with('#') {
            panic!("Color hex should start from \"#\" character");
        };
        let color_len = color.len();
        if color_len < 3 {
            panic!(panic_string);
        }
        let c = u32::from_str_radix(&color[1..], 16).expect(&*panic_string);
        let (red, green, blue, alpha) = match color_len {
            3 => (
                (c >> 8) + ((c >> 8) << 4),
                (c & 0x0f0) + ((c & 0x0f0) >> 4),
                (c & 0x00f) + ((c & 0x00f) << 4),
                0xff
            ),
            6 => (
                c >> 16,
                (c & 0x00ff00) >> 8,
                c & 0x0000ff,
                0xff
            ),
            4 => (
                (c >> 12) + ((c >> 12) << 4),
                ((c & 0x0f00) >> 4) + ((c & 0x0f00) >> 8),
                (c & 0x00f0) + ((c & 0x00f0) >> 4),
                (c & 0x000f) + ((c & 0x000f) << 4)
            ),
            8 => (
                c >> 24,
                (c & 0x00ff0000) >> 16,
                (c & 0x0000ff00) >> 8,
                c & 0x000000ff
            ),
            _ => panic!(panic_string)
        };
        if color_len == 4 || color_len == 8 {
            Color::RGBA(red as u8, green as u8, blue as u8, alpha as u8)
        } else {
            Color::RGB(red as u8, green as u8, blue as u8)
        }
    }
}