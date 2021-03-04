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