/// Can be used to separate audio space to allow different transforms to apply to different groups of playing sounds.
/// 
/// Can be extended with Custom by using concrete project values.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioChannel {
    /// Default audio channel
    Default,
    /// Effects audio channel
    Effects,
    /// Interface audio channel
    Interface,
    /// Music audio channel
    Music,

    /// Allows AudioChannel to be extended (e.g. for using project specific enumerated audio channels or entity specific channels).
    Custom {
        /// Custom value
        value: u32,
    },
}
