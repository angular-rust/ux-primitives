/// Representing the control buttons on the `Overlay`.
///
pub enum OverlayButton {
    Back,
    Mute,
    Unmute,
    Pause,
    Unpause,
    /// Allows [OverlayButton] to be extended (e.g. for using project specific overlay buttons - settings, restart, context help).
    Custom {
        value: u32,
    },
}
