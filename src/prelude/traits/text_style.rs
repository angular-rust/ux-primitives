use crate::prelude::TextAlign;

/// The TextStyle should be implemented by objects created by the createTextStyle method of `Factory`.
///
pub trait TextStyle {
    /// The name of the font, as a string.
    fn font(&self) -> String;

    /// Set the name of the font, as a string.
    fn set_font(&self, val: String);

    /// The point size of text.
    fn size(&self) -> f32;

    /// Set the point size of text.
    fn set_size(&self, val: f32);

    /// The color of the text.
    fn color(&self) -> i32;

    /// Set the color of the text.
    fn set_color(&self, val: i32);

    /// Font horizontal alignment.
    fn align(&self) -> TextAlign;

    /// Set the font horizontal alignment.
    fn set_align(&self, val: TextAlign);

    /// Space in pixels added between each character.
    fn spacing_horizontal(&self) -> f32;

    /// Set the space in pixels added between each character.
    fn set_spacing_horizontal(&self, val: f32);

    /// Space in pixels added between each new line (often called leading).
    fn spacing_vertical(&self) -> f32;

    /// Set the space in pixels added between each new line (often called leading).
    fn set_spacing_vertical(&self, val: f32);

    /// Font weight.
    fn is_bold(&self) -> bool;

    /// Set the font weight.
    fn set_bold(&self, val: bool);

    /// Font emphasis.
    fn is_italic(&self) -> bool;

    /// Set the font emphasis.
    fn set_italic(&self, val: bool);

    /// Thickness of the glyph edges of this font.  Range: -1...1.  Default is 0.
    fn thickness(&self) -> f32;

    /// Set the thickness of the glyph edges of this font.  Range: -1...1.  Default is 0.
    fn set_thickness(&self, val: f32);

    // /// Collection of visual filters appled to font.
    // fn filters(&self) -> Vec<T>;

    // /// Set the collection of visual filters appled to font.
    // fn set_filters(&self, val: Vec<T>);

    /// String representation of this object.
    ///
    /// Return: Representation of this object.
    fn to_string(&self) -> String;

    /// Duplicates this TextStyle.
    ///
    /// Return: A duplicate.
    fn clone(&self) -> Box<dyn TextStyle>;
}
