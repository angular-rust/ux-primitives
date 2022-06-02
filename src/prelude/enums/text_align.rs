/// Representing alternative horizontal text alignments for 
/// `TextStyle` implementations and layout of multiple lines of text
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlign {
    /// Justified text
    Justify,
    /// Left aligned text
    Left,
    /// Center aligned text
    Center,
    /// Right aligned text
    Right,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}