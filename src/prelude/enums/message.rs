/// These messages are dispatched internally.
///
pub enum MessageKind {
    Dispose,
    Init,
    Pause,
    Resume,
    /// Allows [MessageKind] to be extended (e.g. for using project specific mesages).
    Custom {
        value: u32,
    },
}
