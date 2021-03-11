use super::Color;
use std::collections::HashMap;

lazy_static! {
    static ref COLORS: HashMap<&'static str, Color> = {
        let colors: Vec<(&'static str, u32)> = vec![
            // css 1
            ("black",   0x000000),
            ("silver",  0xc0c0c0),
            ("gray",    0x808080),
            ("white",   0xffffff),
            ("maroon",  0x800000),
            ("red",     0xff0000),
            ("purple",  0x800080),
            ("fuchsia", 0xff00ff),
            ("green",   0x008000),
            ("lime",    0x00ff00),
            ("olive",   0x808000),
            ("yellow",  0xffff00),
            ("navy",    0x000080),
            ("blue",    0x0000ff),
            ("teal",    0x008080),
            ("aqua",    0x00ffff),

            // css2 rev 1
            ("orange",          0xffa500),
            ("aliceblue",       0xf0f8ff),
            ("antiquewhite",    0xfaebd7),
            ("aquamarine",      0x7fffd4),
            ("azure",           0xf0ffff),
            ("beige",           0xf5f5dc),
            ("bisque",          0xffe4c4),
            ("blanchedalmond",  0xffebcd),
            ("blueviolet",      0x8a2be2),
            ("brown",           0xa52a2a),
            ("burlywood",       0xdeb887),
            ("cadetblue",       0x5f9ea0),
            ("chartreuse",      0x7fff00),
            ("chocolate",       0xd2691e),
            ("coral",           0xff7f50),
            ("cornflowerblue",  0x6495ed),
            ("cornsilk",        0xfff8dc),
            ("crimson",         0xdc143c),
            ("cyan",            0x00ffff),
            ("darkblue",        0x00008b),
            ("darkcyan",        0x008b8b),
            ("darkgoldenrod",   0xb8860b),
            ("darkgray",        0xa9a9a9),
            ("darkgreen",       0x006400),
            ("darkgrey",        0xa9a9a9),
            ("darkkhaki",       0xbdb76b),
            ("darkmagenta",     0x8b008b),
            ("darkolivegreen",  0x556b2f),
            ("darkorange",      0xff8c00),
            ("darkorchid",      0x9932cc),
            ("darkred",         0x8b0000),
            ("darksalmon",      0xe9967a),
            ("darkseagreen",    0x8fbc8f),
            ("darkslateblue",   0x483d8b),
            ("darkslategray",   0x2f4f4f),
            ("darkslategrey",   0x2f4f4f),
            ("darkturquoise",   0x00ced1),
            ("darkviolet",      0x9400d3),
            ("deeppink",        0xff1493),
            ("deepskyblue",     0x00bfff),
            ("dimgray",         0x696969),
            ("dimgrey",         0x696969),
            ("dodgerblue",      0x1e90ff),
            ("firebrick",       0xb22222),
            ("floralwhite",     0xfffaf0),
            ("forestgreen",     0x228b22),
            ("gainsboro",       0xdcdcdc),
            ("ghostwhite",      0xf8f8ff),
            ("gold",            0xffd700),
            ("goldenrod",       0xdaa520),
            ("greenyellow",     0xadff2f),
            ("grey",            0x808080),
            ("honeydew",        0xf0fff0),
            ("hotpink",         0xff69b4),
            ("indianred",       0xcd5c5c),
            ("indigo",          0x4b0082),
            ("ivory",           0xfffff0),
            ("khaki",           0xf0e68c),
            ("lavender",        0xe6e6fa),
            ("lavenderblush",   0xfff0f5),
            ("lawngreen",       0x7cfc00),
            ("lemonchiffon",    0xfffacd),
            ("lightblue",       0xadd8e6),
            ("lightcoral",      0xf08080),
            ("lightcyan",       0xe0ffff),
            ("lightgoldenrodyellow", 0xfafad2),

            // css3
            ("lightgray",           0xd3d3d3),
            ("lightgreen",          0x90ee90),
            ("lightgrey",           0xd3d3d3),
            ("lightpink",           0xffb6c1),
            ("lightsalmon",         0xffa07a),
            ("lightseagreen",       0x20b2aa),
            ("lightskyblue",        0x87cefa),
            ("lightslategray",      0x778899),
            ("lightslategrey",      0x778899),
            ("lightsteelblue",      0xb0c4de),
            ("lightyellow",         0xffffe0),
            ("limegreen",           0x32cd32),
            ("linen",               0xfaf0e6),
            ("magenta",             0xff00ff),
            ("mediumaquamarine",    0x66cdaa),
            ("mediumblue",          0x0000cd),
            ("mediumorchid",        0xba55d3),
            ("mediumpurple",        0x9370db),
            ("mediumseagreen",      0x3cb371),
            ("mediumslateblue",     0x7b68ee),
            ("mediumspringgreen",   0x00fa9a),
            ("mediumturquoise",     0x48d1cc),
            ("mediumvioletred",     0xc71585),
            ("midnightblue",        0x191970),
            ("mintcream",           0xf5fffa),
            ("mistyrose",           0xffe4e1),
            ("moccasin",            0xffe4b5),
            ("navajowhite",         0xffdead),
            ("oldlace",             0xfdf5e6),
            ("olivedrab",           0x6b8e23),
            ("orangered",           0xff4500),
            ("orchid",              0xda70d6),
            ("palegoldenrod",       0xeee8aa),
            ("palegreen",           0x98fb98),
            ("paleturquoise",       0xafeeee),
            ("palevioletred",       0xdb7093),
            ("papayawhip",          0xffefd5),
            ("peachpuff",           0xffdab9),
            ("peru",                0xcd853f),
            ("pink",                0xffc0cb),
            ("plum",                0xdda0dd),
            ("powderblue",          0xb0e0e6),
            ("rosybrown",           0xbc8f8f),
            ("royalblue",           0x4169e1),
            ("saddlebrown",         0x8b4513),
            ("salmon",              0xfa8072),
            ("sandybrown",          0xf4a460),
            ("seagreen",            0x2e8b57),
            ("seashell",            0xfff5ee),
            ("sienna",              0xa0522d),
            ("skyblue",             0x87ceeb),
            ("slateblue",           0x6a5acd),
            ("slategray",           0x708090),
            ("slategrey",           0x708090),
            ("snow",                0xfffafa),
            ("springgreen",         0x00ff7f),
            ("steelblue",           0x4682b4),
            ("tan",                 0xd2b48c),
            ("thistle",             0xd8bfd8),
            ("tomato",              0xff6347),
            ("turquoise",           0x40e0d0),
            ("violet",              0xee82ee),
            ("wheat",               0xf5deb3),
            ("whitesmoke",          0xf5f5f5),
            ("yellowgreen",         0x9acd32),

            // css4
            ("rebeccapurple",       0x663399),
        ];
        let mut map = HashMap::new();
        for (color_name, color_u32) in colors.iter() {
            map.insert(*color_name, (*color_u32).into());
        }
        map
    };
}

impl From<u32> for Color {
    fn from(color_u32: u32) -> Self {
        Self::from_rgb_u32(color_u32)
    }
}

impl From<&str> for Color {
    fn from(color_str: &str) -> Self {
        if color_str.starts_with('#') {
            Self::from_hex_str(color_str)
        } else {
            Self::from_css_name(color_str)
        }
    }
}

impl Color {
    pub fn from_css_name(name: &str) -> Self {
        COLORS.get(name).expect("Unknown css name of color").clone()
    }

    pub fn from_hex_str(color: &str) -> Self {
        let color_hex_str = if color.len() > 9 {
            &color[0..9]
        } else { color };
        let panic_string = format!("Color hex invalid format of string \"{}\"", color_hex_str);
        if !color_hex_str.starts_with('#') {
            panic!("Color hex should start from \"#\" character");
        };
        let color_hex_str = &color_hex_str[1..];
        let color_len = color_hex_str.len();
        if color_len < 3 {
            panic!(panic_string);
        }
        let color_u32 = u32::from_str_radix(color_hex_str, 16).expect(&*panic_string);
        match color_len {
            3 => Self::from_short_rgb_u16(color_u32 as u16),
            6 => Self::from_rgb_u32(color_u32),
            4 => Self::from_short_rgba_u16(color_u32 as u16),
            8 => Self::from_rgba_u32(color_u32),
            _ => panic!(panic_string)
        }
    }

    pub fn from_short_rgb_u16(c: u16) -> Self {
        let (red, green, blue, _) = Self::from_short_rgb_u16_to_tuple(c);
        Self::RGB(red, green, blue)
    }

    pub fn from_rgb_u32(c: u32) -> Self {
        let (red, green, blue, _) = Self::from_rgb_u32_to_tuple(c);
        Self::RGB(red, green, blue)
    }

    pub fn from_short_rgba_u16(c: u16) -> Self {
        let (red, green, blue, alpha) = Self::from_short_rgba_u16_to_tuple(c);
        Self::RGBA(red, green, blue, alpha)
    }

    pub fn from_rgba_u32(c: u32) -> Self {
        let (red, green, blue, alpha) = Self::from_rgba_u32_to_tuple(c);
        Self::RGBA(red, green, blue, alpha)
    }

    #[inline]
    fn from_short_rgb_u16_to_tuple(c: u16) -> (u8, u8, u8, u8) {
        (
            ((c >> 8) + ((c >> 8) << 4)) as u8,
            ((c & 0x0f0) + ((c & 0x0f0) >> 4)) as u8,
            ((c & 0x00f) + ((c & 0x00f) << 4)) as u8,
            0xff
        )
    }

    #[inline]
    fn from_rgb_u32_to_tuple(c: u32) -> (u8, u8, u8, u8) {
        (
            (c >> 16) as u8,
            ((c & 0x00ff00) >> 8) as u8,
            (c & 0x0000ff) as u8,
            0xff
        )
    }

    #[inline]
    fn from_short_rgba_u16_to_tuple(c: u16) -> (u8, u8, u8, u8) {
        (
            ((c >> 12) + ((c >> 12) << 4)) as u8,
            (((c & 0x0f00) >> 4) + ((c & 0x0f00) >> 8)) as u8,
            ((c & 0x00f0) + ((c & 0x00f0) >> 4)) as u8,
            ((c & 0x000f) + ((c & 0x000f) << 4)) as u8
        )
    }

    #[inline]
    fn from_rgba_u32_to_tuple(c: u32) -> (u8, u8, u8, u8) {
        (
            (c >> 24) as u8,
            ((c & 0x00ff0000) >> 16) as u8,
            ((c & 0x0000ff00) >> 8) as u8,
            (c & 0x000000ff) as u8
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_get_color_from_impl() {
        let test_data: Vec<(&str, &str, u32, Color)> = vec![
            ("palegoldenrod",  "#eee8aa", 0xeee8aa, Color::RGB(0xee, 0xe8, 0xaa)),
            ("palegreen",      "#98fb98", 0x98fb98, Color::RGB(0x98, 0xfb, 0x98)),
            ("paleturquoise",  "#afeeee", 0xafeeee, Color::RGB(0xaf, 0xee, 0xee)),
            ("palevioletred",  "#db7093", 0xdb7093, Color::RGB(0xdb, 0x70, 0x93)),
            ("papayawhip",     "#ffefd5", 0xffefd5, Color::RGB(0xff, 0xef, 0xd5)),
            ("peachpuff",      "#ffdab9", 0xffdab9, Color::RGB(0xff, 0xda, 0xb9)),
            ("peru",           "#cd853f", 0xcd853f, Color::RGB(0xcd, 0x85, 0x3f)),
            ("pink",           "#ffc0cb", 0xffc0cb, Color::RGB(0xff, 0xc0, 0xcb)),
            ("plum",           "#dda0dd", 0xdda0dd, Color::RGB(0xdd, 0xa0, 0xdd)),
            ("powderblue",     "#b0e0e6", 0xb0e0e6, Color::RGB(0xb0, 0xe0, 0xe6)),
        ];
        for (color_name, color_hex_str, color_u32, color) in test_data.iter() {
            if let Color::RGB(expected_red, expected_green, expected_blue) = *color {
                let actual_colors_vec: Vec<Color> = vec![
                    (*color_name).into(),
                    (*color_hex_str).into(),
                    (*color_u32).into()
                ];
                for actual_color in actual_colors_vec {
                    if let Color::RGB(actual_red, actual_green, actual_blue) = actual_color {
                        assert_eq!(actual_red, expected_red);
                        assert_eq!(actual_green, expected_green);
                        assert_eq!(actual_blue, expected_blue);
                    } else {
                        assert!(false, "Actual color should be in RGB notation");
                    }
                }
            } else {
                assert!(false, "Expected color should be in RGB notation");
            }
        }
    }
}