#[derive(Clone, Debug)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

#[derive(Clone, Debug)]
pub enum TextWeight {
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

impl Default for TextWeight {
    fn default() -> Self {
        TextWeight::Normal
    }
}

#[derive(Clone, Debug)]
pub enum TextStyle {
    Normal = 0,
    Oblique = 1,
    Italic = 2,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle::Normal
    }
}

/// see https://developer.mozilla.org/ru/docs/Web/API/Canvas_API/Tutorial/Drawing_text
#[derive(Clone, Debug)]
pub enum BaseLine {
    Top, 
    Hanging, 
    Middle, 
    Alphabetic, 
    Ideographic, 
    Bottom
}

impl Default for BaseLine {
    fn default() -> Self {
        BaseLine::Middle
    }
}