/// These messages are dispatched internally.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageKind {
    /// Dispose message type
    Dispose,
    /// Init message type
    Init,
    /// Pause message type
    Pause,
    /// Resume message type
    Resume,
    /// Allows [MessageKind] to be extended (e.g. for using project specific mesages).
    Custom {
        /// Custom value
        value: u32,
    },
}
