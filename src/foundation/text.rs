/// The horizontal alignment and layout of multiple lines of text.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

/// The font-weight property sets how thick or thin characters in text should be displayed
///
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin = 100,
    UltraLight = 200,
    Light = 300,
    SemiLight = 350,
    Book = 380,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    UltraBold = 800,
    Heavy = 900,
    UltraHeavy = 1000,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

/// Whether to slant the glyphs in the font.
///
#[derive(Clone, Debug, Copy)]
pub enum FontStyle {
    Normal = 0,
    Oblique = 1,
    Italic = 2,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

/// Indicates the current baseline when drawing text. 
/// 
/// see <https://developer.mozilla.org/ru/docs/Web/API/Canvas_API/Tutorial/Drawing_text>
#[derive(Clone, Debug, Copy)]
pub enum BaseLine {
    Top,
    Hanging,
    Middle,
    Alphabetic,
    Ideographic,
    Bottom,
}

impl Default for BaseLine {
    fn default() -> Self {
        BaseLine::Middle
    }
}
