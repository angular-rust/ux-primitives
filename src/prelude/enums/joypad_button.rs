/// Representing the buttons on a Joypad for use in the `Joypad` virtual controller.
/// 
/// Includes directional buttons and two fire buttons: Primary & Secondary. Fire is used as a shortcut for Primary or Secondary.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoypadButton {
    /// Fire Joypad button
    Fire,
    /// Up Joypad button
    Up,
    /// Right Joypad button
    Right,
    /// Down Joypad button
    Down,
    /// Left Joypad button
    Left,
    /// Primary Joypad button
    Primary,
    /// Secondary Joypad button
    Secondary,
}
