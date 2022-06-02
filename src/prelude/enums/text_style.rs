/// Representing default identifiers of `TextStyle`s.
/// 
/// Can be extended with Custom by using concrete project values.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextStyleKind {
    /// Button text style
    Button,
    /// Body text style
    Body,
    /// Headline text style
    Headline,
    /// Subhead text style
    Subhead,
    /// Smallprint text style
    Smallprint,
    /// Oversized text style
    Oversized,

    /// Allows [TextStyleKind] to be extended (e.g. for using project specific text styles).
    Custom {
        /// Custom value
        value: u32,
    },
}
