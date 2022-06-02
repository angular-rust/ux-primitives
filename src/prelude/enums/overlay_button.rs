/// Representing the control buttons on the `Overlay`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayButton {
    /// Back overlay button
    Back,
    /// Mute overlay button
    Mute,
    /// UnMute overlay button
    Unmute,
    /// Pause overlay button
    Pause,
    /// Unpause overlay button
    Unpause,
    /// Allows [OverlayButton] to be extended (e.g. for using project specific overlay buttons - settings, restart, context help).
    Custom {
        /// Custom value
        value: u32,
    },
}
