/// The font-weight property sets how thick or thin characters in text should be displayed
///
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum FontWeight {
    /// Thin text weight 
    Thin = 100,
    /// Ultra light text weight
    UltraLight = 200,
    /// Light text weight
    Light = 300,
    /// Semi light text weight
    SemiLight = 350,
    /// Book text weight
    Book = 380,
    /// Normal text weight
    Normal = 400,
    /// Medium text weight
    Medium = 500,
    /// Semi bold text weight
    SemiBold = 600,
    /// Bold text weight
    Bold = 700,
    /// Ultra bold text weight
    UltraBold = 800,
    /// Heavy text weight
    Heavy = 900,
    /// Ultra heavy text weight
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
    /// Normal font
    Normal = 0,
    /// Oblique font
    Oblique = 1,
    /// Italic font
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
    /// Top baseline
    Top,
    /// Hanging baseline
    Hanging,
    /// Middle baseline
    Middle,
    /// Alphabetic baseline
    Alphabetic,
    /// Ideographic baseline
    Ideographic,
    /// Bottom baseline
    Bottom,
}

impl Default for BaseLine {
    fn default() -> Self {
        BaseLine::Middle
    }
}
