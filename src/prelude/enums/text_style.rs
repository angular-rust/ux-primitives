/// Representing default identifiers of `TextStyle`s.
/// 
/// Can be extended with Custom by using concrete project values.
///
pub enum TextStyleKind {
    Button,
    Body,
    Headline,
    Subhead,
    Smallprint,
    Oversized,

    /// Allows [TextStyleKind] to be extended (e.g. for using project specific text styles).
    Custom {
        value: u32,
    },
}
