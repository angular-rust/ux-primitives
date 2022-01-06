use crate::prelude::JoypadButton;

/// The InputJoypad should be implemented by an object wishing to be used as a directional pad plus two fire button joypad.
/// 
/// A joypad is useful for a simple game input device.  The device is limited to 6 `JoypadButtons` to prevent keyboard lock and overly complex game controls.
/// A joypad can be configured to accept bespoke key configurations, or alternative user interface control devices (e.g. mouse or touchpad).
///
pub trait InputJoypad {
    /// Determine if a specific joypad button is currently down.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The joypad button.
    ///
    /// Return: Returns true is the joypad button is currently down, false otherwise.
    /// 
    fn is_button_down(&self, kind: JoypadButton) -> bool;

    /// Determine if a specific joypad button was pressed in the current update frame.
    /// A press is defined as a new down - i.e. was up previous frame, and is down this frame.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The joypad button.
    ///
    /// Return: Returns true is the joypad button was pressed in the current update, false otherwise.
    /// 
    fn is_button_press(&self, kind: JoypadButton) -> bool;

    /// Determine if a specific joypad button was released in the current update.
    /// A release is defined as a new up - i.e. was down previous frame, and is up this frame.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The joypad button.
    ///
    /// Return: Returns true is the joypad button was released in the current update, false otherwise.
    /// 
    fn is_button_release(&self, kind: JoypadButton) -> bool;

    /// Determine how long a specific joypad button has been down.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The joypad button.
    /// * `as_time` - If true then returns duration as milliseconds, else returns duration as frame updates. (optional, default: true)
    /// * `is_previous` - If true then returns the previous duration down (the time held prior to the most recent release). (optional, default: false)
    ///
    /// Return: Returns the duration the joypad button has been down.
    /// 
    fn get_button_down_duration(&self, kind: JoypadButton, as_time: Option<bool>, is_previous: Option<bool>) -> i32;

    /// Determine how long a specific joypad button has been up.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The joypad button.
    /// * `as_time` - If true then returns duration as milliseconds, else returns duration as frame updates. (optional, default: true)
    /// * `is_previous` - If true then returns the previous duration up (the time unused prior to the most recent press). (optional, default: false)
    ///
    /// Return: Returns the duration the joypad button has been up.
    /// 
    fn get_button_up_duration(&self, kind: JoypadButton, as_time: Option<bool>, is_previous: Option<bool>) -> i32;
}
