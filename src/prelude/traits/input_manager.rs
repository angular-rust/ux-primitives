use crate::prelude::{JoypadTouch, Key};

use super::{InputJoypad, InputKeyboard, InputMouse, Resettable};

/// The InputManager should be implemented by an object wishing to provide user input states to the kernel.
/// 
/// The state machine represents the configuration of the input devices at any specific update frame.
/// State based input is useful for many types of game mechanics, including: momentum, instant replays and special move combos.
///
pub trait InputManager: Resettable {
    /// The default virtual joypad user input: simple 4 directional controller with 2 fire buttons.  Listens to cursor keys and WASD keys.
    fn joypad(&self) -> Box<dyn InputJoypad>;

    /// The virtual keyboard user input: every key on the keyboard.
    fn keyboard(&self) -> Box<dyn InputKeyboard>;

    /// The virtual mouse user input: 3 button mouse and scroll wheel.
    fn mouse(&self) -> Box<dyn InputMouse>;

    /// Factory method to create a virtual joypad with custom key controls.
    /// 
    /// # Arguments
    /// 
    /// * `up` - The key for up directional movement.  Defaults to up cursor. (optional)
    /// * `right` - The key for right directional movement.  Defaults to right cursor. (optional)
    /// * `down` - The key for down directional movement.  Defaults to down cursor. (optional)
    /// * `left` - The key for left directional movement.  Defaults to left cursor. (optional)
    /// * `primary` - The key for primary fire.  Defaults to space. (optional)
    /// * `secondary` - The key for secondary fire.  Defaults to Z key. (optional)
    /// * `upAlt` - Optional alternative key for up directional movement. (optional)
    /// * `rightAlt` - Optional alternative key for right directional movement. (optional)
    /// * `downAlt` - Optional alternative key for down directional movement. (optional)
    /// * `leftAlt` - Optional alternative key for left directional movement. (optional)
    /// * `primaryAlt` - Optional alternative key for primary fire. (optional)
    /// * `secondaryAlt` - Optional alternative key for secondary fire. (optional)
    /// * `joypadTouchType` - Optional touch enabled mode (for devices without keys), defaults to Factory::joypadTouchType (optional)
    ///
    /// Return: A virtual joypad with custom key controls.
    /// 
    fn create_joypad(
        &self,
        up: Option<Key>,
        right: Option<Key>,
        down: Option<Key>,
        left: Option<Key>,
        primary: Option<Key>,
        secondary: Option<Key>,
        up_alt: Option<Key>,
        right_alt: Option<Key>,
        down_alt: Option<Key>,
        left_alt: Option<Key>,
        primary_alt: Option<Key>,
        secondary_alt: Option<Key>,
        joypad_touch_type: Option<JoypadTouch>,
    ) -> Box<dyn InputJoypad>;
}
