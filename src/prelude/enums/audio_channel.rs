/// Can be used to separate audio space to allow different transforms to apply to different groups of playing sounds.
/// 
/// Can be extended with Custom by using concrete project values.
///
pub enum AudioChannel {
    Default,
    Effects,
    Interface,
    Music,

    /// Allows AudioChannel to be extended (e.g. for using project specific enumerated audio channels or entity specific channels).
    Custom {
        value: u32,
    },
}
