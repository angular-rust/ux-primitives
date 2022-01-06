use crate::prelude::Key;

/// The InputKeyboard should be implemented by an object wishing to be used as a virtual keyboard input device.
///
pub trait InputKeyboard {
    /// Determine if a specific key is currently down.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - They key.
    ///
    /// Return: Returns true is the key is currently down, false otherwise.
    /// 
    fn is_key_down(&self, kind: Key) -> bool;

    /// Determine if a specific key was pressed in the current update frame.
    /// A press is defined as a new down - i.e. was up previous frame, and is down this frame.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The key.
    ///
    /// Return: Returns true is the key was pressed in the current update, false otherwise.
    /// 
    fn is_key_press(&self, kind: Key) -> bool;

    /// Determine if a specific key was released in the current update.
    /// A release is defined as a new up - i.e. was down previous frame, and is up this frame.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The key.
    ///
    /// Return: Returns true is the key was released in the current update, false otherwise.
    /// 
    fn is_key_release(&self, kind: Key) -> bool;

    /// Determine how long a specific key has been down.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The key.
    /// * `as_time` - If true then returns duration as milliseconds, else returns duration as frame updates. (optional, default: true)
    /// * `is_previous` - If true then returns the previous duration down (the time held prior to the most recent release). (optional, default: false)
    ///
    /// Return: Returns the duration the key has been down.
    /// 
    fn get_key_down_duration(&self, kind: Key, as_time: Option<bool>, is_previous: Option<bool>) -> i32;

    /// Determine how long a specific key has been up.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The key.
    /// * `as_time` - If true then returns duration as milliseconds, else returns duration as frame updates. (optional, default: true)
    /// * `is_previous` - If true then returns the previous duration up (the time unused prior to the most recent press). (optional, default: false)
    ///
    /// Return: Returns the duration the key has been up.
    /// 
    fn get_key_up_duration(&self, kind: Key, as_time: Option<bool>, is_previous: Option<bool>) -> i32;

    /// Translate a specific key to a keyboard keyCode.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The key.
    ///
    /// Return: Returns the keyboard keyCode of the corresponding key.
    /// 
    fn get_key_code(&self, kind: Key) -> i32;

    /// Translate a keyCode to a specific key.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The keyCode.
    ///
    /// Return: Returns the key of the corresponding keyboard keyCode.
    /// 
    fn get_key(&self, key_code: i32) -> Key;
}
