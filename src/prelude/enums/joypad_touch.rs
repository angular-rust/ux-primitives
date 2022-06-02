/// Representing the touch input modes for a Touchscreen Joypad for use in the `Joypad` virtual controller.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoypadTouch {
    /// Touch input is disabled for joypad.  The default.
    /// It is advised that [JoypadTouch] be set in Factory based on device interrogation.
    Disabled,

    /// The screen is split into five fixed position regions corresponding to directions, only one can be down at a time.
    /// A single fire button (primary) is located in the center.
    Dpad,

    /// A self centering joystick represented by a constant drag / touch.  A drag can begin anywhere.
    /// Distance sets the minimum vector length that determines movement.  20 pixels is the default.
    /// If the horizontal drag vector is > distance then left or right is down.
    /// If the vertical drag vector is > distance then up or down is down.
    /// Any tap of less than 200ms triggers primary down.
    Joystick { 
        /// Distance value
        distance: i32 
    }, // is optional

    /// A swipe can begin anywhere.  Speed sets the minimum vector length (pixels per second) that determines movement.  100 pixels is the default.
    /// If the horizontal swipe vector is > speed then left or right is down.
    /// If the vertical swipe vector is > speed then up or down is down.
    /// Any tap of less than 200ms triggers primary down.
    Swipe { 
        /// Swipe speed value
        speed: i32 
    }, // is optional
}
