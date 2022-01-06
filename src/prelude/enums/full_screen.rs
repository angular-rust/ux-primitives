/// Representing possible full screen modes.
/// 
/// Can be extended with Custom by using concrete project values.
///
pub enum FullScreen {
    /// FullScreen mode is not available.
    Disabled,

    /// Preserves original pixel size.
    NoScale,

    /// Scale without preserving aspect ratio - some distortion may occur, including non linear pixel sizes.
    ScaleAspectRatioIgnore,

    /// Scale without aspect ratio distortion.  Non linear pixel sizes may occur.
    ScaleAspectRatioPreserve,

    /// Scale without distortion, and doubling pixels (2x2, 4x4 etc) to nearest multiple to preserve pixel sizes.
    ScaleNearestMultiple,

    /// Allows Scale to be extended (e.g. for using project specific scale modes).
    Custom { value: f32 },
}
