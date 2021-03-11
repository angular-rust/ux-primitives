use super::Color;
use std::collections::HashMap;

impl Color {
    pub fn from_css_name(name: &str) -> Self {
        COLORS.get(name).expect("Unknown css name of color").clone()
    }

    pub fn from_hex_str(color: &str) -> Self {
        let color= &color[0..9];
        let panic_string = format!("Color hex invalid format of string \"{}\"", color);
        if !color.starts_with('#') {
            panic!("Color hex should start from \"#\" character");
        };
        let color_len = color.len();
        if color_len < 3 {
            panic!(panic_string);
        }
        let color_u32 = u32::from_str_radix(&color[1..], 16).expect(&*panic_string);
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

    fn from_short_rgba_u16(c: u16) -> Self {
        let (red, green, blue, alpha) = Self::from_short_rgba_u16_to_tuple(c);
        Self::RGBA(red, green, blue, alpha)
    }

    fn from_rgba_u32(c: u32) -> Self {
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

lazy_static! {
    static ref COLORS: HashMap<&'static str, Color> = {
        let mut c = HashMap::new();

        // css 1
        c.insert("black",   Color::from_rgb_u32(0x000000));
        c.insert("silver",  Color::from_rgb_u32(0xc0c0c0));
        c.insert("gray",    Color::from_rgb_u32(0x808080));
        c.insert("white",   Color::from_rgb_u32(0xffffff));
        c.insert("maroon",  Color::from_rgb_u32(0x800000));
        c.insert("red",     Color::from_rgb_u32(0xff0000));
        c.insert("purple",  Color::from_rgb_u32(0x800080));
        c.insert("fuchsia", Color::from_rgb_u32(0xff00ff));
        c.insert("green",   Color::from_rgb_u32(0x008000));
        c.insert("lime",    Color::from_rgb_u32(0x00ff00));
        c.insert("olive",   Color::from_rgb_u32(0x808000));
        c.insert("yellow",  Color::from_rgb_u32(0xffff00));
        c.insert("navy",    Color::from_rgb_u32(0x000080));
        c.insert("blue",    Color::from_rgb_u32(0x0000ff));
        c.insert("teal",    Color::from_rgb_u32(0x008080));
        c.insert("aqua",    Color::from_rgb_u32(0x00ffff));

        // css2 rev 1
        c.insert("orange",          Color::from_rgb_u32(0xffa500));
        c.insert("aliceblue",       Color::from_rgb_u32(0xf0f8ff));
        c.insert("antiquewhite",    Color::from_rgb_u32(0xfaebd7));
        c.insert("aquamarine",      Color::from_rgb_u32(0x7fffd4));
        c.insert("azure",           Color::from_rgb_u32(0xf0ffff));
        c.insert("beige",           Color::from_rgb_u32(0xf5f5dc));
        c.insert("bisque",          Color::from_rgb_u32(0xffe4c4));
        c.insert("blanchedalmond",  Color::from_rgb_u32(0xffebcd));
        c.insert("blueviolet",      Color::from_rgb_u32(0x8a2be2));
        c.insert("brown",           Color::from_rgb_u32(0xa52a2a));
        c.insert("burlywood",       Color::from_rgb_u32(0xdeb887));
        c.insert("cadetblue",       Color::from_rgb_u32(0x5f9ea0));
        c.insert("chartreuse",      Color::from_rgb_u32(0x7fff00));
        c.insert("chocolate",       Color::from_rgb_u32(0xd2691e));
        c.insert("coral",           Color::from_rgb_u32(0xff7f50));
        c.insert("cornflowerblue",  Color::from_rgb_u32(0x6495ed));
        c.insert("cornsilk",        Color::from_rgb_u32(0xfff8dc));
        c.insert("crimson",         Color::from_rgb_u32(0xdc143c));
        c.insert("cyan",            Color::from_rgb_u32(0x00ffff));
        c.insert("darkblue",        Color::from_rgb_u32(0x00008b));
        c.insert("darkcyan",        Color::from_rgb_u32(0x008b8b));
        c.insert("darkgoldenrod",   Color::from_rgb_u32(0xb8860b));
        c.insert("darkgray",        Color::from_rgb_u32(0xa9a9a9));
        c.insert("darkgreen",       Color::from_rgb_u32(0x006400));
        c.insert("darkgrey",        Color::from_rgb_u32(0xa9a9a9));
        c.insert("darkkhaki",       Color::from_rgb_u32(0xbdb76b));
        c.insert("darkmagenta",     Color::from_rgb_u32(0x8b008b));
        c.insert("darkolivegreen",  Color::from_rgb_u32(0x556b2f));
        c.insert("darkorange",      Color::from_rgb_u32(0xff8c00));
        c.insert("darkorchid",      Color::from_rgb_u32(0x9932cc));
        c.insert("darkred",         Color::from_rgb_u32(0x8b0000));
        c.insert("darksalmon",      Color::from_rgb_u32(0xe9967a));
        c.insert("darkseagreen",    Color::from_rgb_u32(0x8fbc8f));
        c.insert("darkslateblue",   Color::from_rgb_u32(0x483d8b));
        c.insert("darkslategray",   Color::from_rgb_u32(0x2f4f4f));
        c.insert("darkslategrey",   Color::from_rgb_u32(0x2f4f4f));
        c.insert("darkturquoise",   Color::from_rgb_u32(0x00ced1));
        c.insert("darkviolet",      Color::from_rgb_u32(0x9400d3));
        c.insert("deeppink",        Color::from_rgb_u32(0xff1493));
        c.insert("deepskyblue",     Color::from_rgb_u32(0x00bfff));
        c.insert("dimgray",         Color::from_rgb_u32(0x696969));
        c.insert("dimgrey",         Color::from_rgb_u32(0x696969));
        c.insert("dodgerblue",      Color::from_rgb_u32(0x1e90ff));
        c.insert("firebrick",       Color::from_rgb_u32(0xb22222));
        c.insert("floralwhite",     Color::from_rgb_u32(0xfffaf0));
        c.insert("forestgreen",     Color::from_rgb_u32(0x228b22));
        c.insert("gainsboro",       Color::from_rgb_u32(0xdcdcdc));
        c.insert("ghostwhite",      Color::from_rgb_u32(0xf8f8ff));
        c.insert("gold",            Color::from_rgb_u32(0xffd700));
        c.insert("goldenrod",       Color::from_rgb_u32(0xdaa520));
        c.insert("greenyellow",     Color::from_rgb_u32(0xadff2f));
        c.insert("grey",            Color::from_rgb_u32(0x808080));
        c.insert("honeydew",        Color::from_rgb_u32(0xf0fff0));
        c.insert("hotpink",         Color::from_rgb_u32(0xff69b4));
        c.insert("indianred",       Color::from_rgb_u32(0xcd5c5c));
        c.insert("indigo",          Color::from_rgb_u32(0x4b0082));
        c.insert("ivory",           Color::from_rgb_u32(0xfffff0));
        c.insert("khaki",           Color::from_rgb_u32(0xf0e68c));
        c.insert("lavender",        Color::from_rgb_u32(0xe6e6fa));
        c.insert("lavenderblush",   Color::from_rgb_u32(0xfff0f5));
        c.insert("lawngreen",       Color::from_rgb_u32(0x7cfc00));
        c.insert("lemonchiffon",    Color::from_rgb_u32(0xfffacd));
        c.insert("lightblue",       Color::from_rgb_u32(0xadd8e6));
        c.insert("lightcoral",      Color::from_rgb_u32(0xf08080));
        c.insert("lightcyan",       Color::from_rgb_u32(0xe0ffff));
        c.insert("lightgoldenrodyellow", Color::from_rgb_u32(0xfafad2));

        //css3
        c.insert("lightgray",           Color::from_rgb_u32(0xd3d3d3));
        c.insert("lightgreen",          Color::from_rgb_u32(0x90ee90));
        c.insert("lightgrey",           Color::from_rgb_u32(0xd3d3d3));
        c.insert("lightpink",           Color::from_rgb_u32(0xffb6c1));
        c.insert("lightsalmon",         Color::from_rgb_u32(0xffa07a));
        c.insert("lightseagreen",       Color::from_rgb_u32(0x20b2aa));
        c.insert("lightskyblue",        Color::from_rgb_u32(0x87cefa));
        c.insert("lightslategray",      Color::from_rgb_u32(0x778899));
        c.insert("lightslategrey",      Color::from_rgb_u32(0x778899));
        c.insert("lightsteelblue",      Color::from_rgb_u32(0xb0c4de));
        c.insert("lightyellow",         Color::from_rgb_u32(0xffffe0));
        c.insert("limegreen",           Color::from_rgb_u32(0x32cd32));
        c.insert("linen",               Color::from_rgb_u32(0xfaf0e6));
        c.insert("magenta",             Color::from_rgb_u32(0xff00ff));
        c.insert("mediumaquamarine",    Color::from_rgb_u32(0x66cdaa));
        c.insert("mediumblue",          Color::from_rgb_u32(0x0000cd));
        c.insert("mediumorchid",        Color::from_rgb_u32(0xba55d3));
        c.insert("mediumpurple",        Color::from_rgb_u32(0x9370db));
        c.insert("mediumseagreen",      Color::from_rgb_u32(0x3cb371));
        c.insert("mediumslateblue",     Color::from_rgb_u32(0x7b68ee));
        c.insert("mediumspringgreen",   Color::from_rgb_u32(0x00fa9a));
        c.insert("mediumturquoise",     Color::from_rgb_u32(0x48d1cc));
        c.insert("mediumvioletred",     Color::from_rgb_u32(0xc71585));
        c.insert("midnightblue",        Color::from_rgb_u32(0x191970));
        c.insert("mintcream",           Color::from_rgb_u32(0xf5fffa));
        c.insert("mistyrose",           Color::from_rgb_u32(0xffe4e1));
        c.insert("moccasin",            Color::from_rgb_u32(0xffe4b5));
        c.insert("navajowhite",         Color::from_rgb_u32(0xffdead));
        c.insert("oldlace",             Color::from_rgb_u32(0xfdf5e6));
        c.insert("olivedrab",           Color::from_rgb_u32(0x6b8e23));
        c.insert("orangered",           Color::from_rgb_u32(0xff4500));
        c.insert("orchid",              Color::from_rgb_u32(0xda70d6));
        c.insert("palegoldenrod",       Color::from_rgb_u32(0xeee8aa));
        c.insert("palegreen",           Color::from_rgb_u32(0x98fb98));
        c.insert("paleturquoise",       Color::from_rgb_u32(0xafeeee));
        c.insert("palevioletred",       Color::from_rgb_u32(0xdb7093));
        c.insert("papayawhip",          Color::from_rgb_u32(0xffefd5));
        c.insert("peachpuff",           Color::from_rgb_u32(0xffdab9));
        c.insert("peru",                Color::from_rgb_u32(0xcd853f));
        c.insert("pink",                Color::from_rgb_u32(0xffc0cb));
        c.insert("plum",                Color::from_rgb_u32(0xdda0dd));
        c.insert("powderblue",          Color::from_rgb_u32(0xb0e0e6));
        c.insert("rosybrown",           Color::from_rgb_u32(0xbc8f8f));
        c.insert("royalblue",           Color::from_rgb_u32(0x4169e1));
        c.insert("saddlebrown",         Color::from_rgb_u32(0x8b4513));
        c.insert("salmon",              Color::from_rgb_u32(0xfa8072));
        c.insert("sandybrown",          Color::from_rgb_u32(0xf4a460));
        c.insert("seagreen",            Color::from_rgb_u32(0x2e8b57));
        c.insert("seashell",            Color::from_rgb_u32(0xfff5ee));
        c.insert("sienna",              Color::from_rgb_u32(0xa0522d));
        c.insert("skyblue",             Color::from_rgb_u32(0x87ceeb));
        c.insert("slateblue",           Color::from_rgb_u32(0x6a5acd));
        c.insert("slategray",           Color::from_rgb_u32(0x708090));
        c.insert("slategrey",           Color::from_rgb_u32(0x708090));
        c.insert("snow",                Color::from_rgb_u32(0xfffafa));
        c.insert("springgreen",         Color::from_rgb_u32(0x00ff7f));
        c.insert("steelblue",           Color::from_rgb_u32(0x4682b4));
        c.insert("tan",                 Color::from_rgb_u32(0xd2b48c));
        c.insert("thistle",             Color::from_rgb_u32(0xd8bfd8));
        c.insert("tomato",              Color::from_rgb_u32(0xff6347));
        c.insert("turquoise",           Color::from_rgb_u32(0x40e0d0));
        c.insert("violet",              Color::from_rgb_u32(0xee82ee));
        c.insert("wheat",               Color::from_rgb_u32(0xf5deb3));
        c.insert("whitesmoke",          Color::from_rgb_u32(0xf5f5f5));
        c.insert("yellowgreen",         Color::from_rgb_u32(0x9acd32));

        //css4
        c.insert("rebeccapurple",       Color::from_rgb_u32(0x663399));

        c
    };
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_get_color_by_name_silver() {
        if let Color::RGB(red, green, blue) = Color::from_css_name("silver") {
            assert_eq!(red, 192);
            assert_eq!(green, 192);
            assert_eq!(blue, 192);
        } else {
            assert!(false, "Color match failed")
        }
    }

    #[test]
    fn test_get_color_by_name_yellowgreen() {
        if let Color::RGB(red, green, blue) = Color::from_css_name("yellowgreen") {
            assert_eq!(red, 0x9a);
            assert_eq!(green, 0xcd);
            assert_eq!(blue, 0x32);
        } else {
            assert!(false, "Color match failed")
        }
    }
}