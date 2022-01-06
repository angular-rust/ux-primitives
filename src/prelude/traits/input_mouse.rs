use crate::prelude::{MouseButton, MouseCursor};

/// The InputMouse should be implemented by objects wishing to act as virtual mouse controllers.
/// 
/// Screen bounds are based on Factory::width & Factory::height.
///
pub trait InputMouse {
    /// The horizontal component of the mouse position.
    fn x(&self) -> i32;

    /// The vertical component of the mouse position.
    fn y(&self) -> i32;

    /// The horizontal position of the mouse relative to screen width.  Range 0...1.
    fn relative_x(&self) -> f32;

    /// The vertical position of the mouse relative to screen height.  Range 0...1.
    fn relative_y(&self) -> f32;

    /// The horizontal position of the mouse relative to screen width and offset to screen centre.  Range -1...1.
    fn relative_centralised_x(&self) -> f32;

    /// The vertical position of the mouse relative to screen height and offset to screen centre.  Range -1...1.
    fn relative_centralised_y(&self) -> f32;

    /// Returns true if the mouse position is within the bounding rectangle (factory width x factory height).
    fn is_within_bounds(&self) -> bool;

    /// Returns true if the mouse position is different to the previous update's position.
    fn is_moving(&self) -> bool;

    /// Get the visibility of the mouse cursor.
    fn is_visible(&self) -> bool;

    /// Specify the visibility of the mouse cursor.
    /// If true the cursor will be displayed, if false the cursor is hidden.
    fn set_visible(&self, val: bool);

    /// The current scroll position.  Starts at 0.  Range -infinity...infinity.
    fn scroll(&self) -> i32;

    /// The current cursor type.
    fn cursor(&self) -> MouseCursor;
    
    /// Set the cursor type.
    fn set_cursor(&self, val: MouseCursor) -> MouseCursor;

    /// The horizontal velocity of the mouse position.
    /// 
    /// # Arguments
    /// 
    /// * `as_time` - If true then returns the velocity as pixels per second (extrapolated from the previous update),
    /// else returns velocity as pixels moved in previous update. (optional, default: true)
    ///
    /// Return: The horizontal velocity of the mouse.
    /// 
    fn get_delta_x(&self, as_time: Option<bool>) -> i32;

    /// The vertical velocity of the mouse position.
    /// 
    /// # Arguments
    /// 
    /// * `as_time` - If true then returns the velocity as pixels per second (extrapolated from the previous update),
    /// else returns velocity as pixels moved in previous update. (optional, default: true)
    ///
    /// Return: The vertical velocity of the mouse.
    /// 
    fn get_delta_y(&self, as_time: Option<bool>) -> i32;

    /// The velocity of the mouse.
    /// 
    /// # Arguments
    /// 
    /// * `as_time` - If true then returns the velocity as pixels per second (extrapolated from the previous update),
    /// else returns velocity as pixels moved in previous update. (optional, default: true)
    ///
    /// Return: The velocity of the mouse.
    /// 
    fn get_speed(&self, as_time: Option<bool>) -> i32;

    /// The velocity of scrolling.
    /// 
    /// # Arguments
    /// 
    /// * `as_time` - If true then returns the velocity as pixels per second (extrapolated from the previous update),
    /// else returns velocity as scroll moved in previous update. (optional, default: true)
    ///
    /// Return: The scroll velocity of the mouse.
    /// 
    fn get_delta_scroll(&self, as_time: Option<bool>) -> i32;

    /// Determine how long the mouse has been still.
    /// 
    /// # Arguments
    /// 
    /// * `as_time` - If true then returns duration as milliseconds, else returns duration as frame updates. (optional, default: true)
    ///
    /// Return: Returns the duration the mouse has been still.
    /// 
    fn get_still_duration(&self, as_time: Option<bool>) -> i32;

    /// Determine if a specific mouse button was clicked twice (within the defined time).
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    /// * `delay` - The time within which the mouse button must be clicked twice. (optional, default: 100)
    ///
    /// Return: Returns true if the mouse button was clicked twice (within the defined time).
    /// 
    fn is_button_double_click(&self, kind: Option<MouseButton>, delay: Option<i32>) -> bool;

    /// Determine if the mouse is being dragged with a specific mouse button down (for at least the defined delay).
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    /// * `delay` - The time which, if exceeded, assumes the mouse is being dragged.  (optional, default: 100)
    ///
    /// Return: Returns true if the mouse button was down for a duration exceeding delay.
    /// 
    fn is_button_drag(&self, kind: Option<MouseButton>, delay: Option<i32>) -> bool;

    /// Determine if a specific mouse button is currently down.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    ///
    /// Return: Returns true is the mouse button is currently down, false otherwise.
    /// 
    fn is_button_down(&self, kind: Option<MouseButton>) -> bool;

    /// Determine if a specific mouse button was pressed in the current update frame.
    /// 
    /// # Arguments
    /// 
    /// A press is defined as a new down - i.e. was up previous frame, and is down this frame.
    /// * `kind` - The mouse button. (optional)
    ///
    /// Return: Returns true is the mouse button was pressed in the current update, false otherwise.
    /// 
    fn is_button_press(&self, kind: Option<MouseButton>) -> bool;

    /// Determine if a specific mouse button was released in the current update.
    /// A release is defined as a new up - i.e. was down previous frame, and is up this frame.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    ///
    /// Return: Returns true is the mouse button was released in the current update, false otherwise.
    /// 
    fn is_button_release(&self, kind: Option<MouseButton>) -> bool;

    /// Determine the duration a specific mouse button is down.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    /// * `as_time` - If true then returns duration as milliseconds, else returns duration as frame updates. (optional, default: true)
    /// * `is_previous	If true then returns the previous duration down (the time held prior to the most recent release). (optional, default: false)
    ///
    /// Return: The duration a specific mouse button is down.
    /// 
    fn get_button_down_duration(&self, kind: Option<MouseButton>, as_time: Option<bool>, is_previous: Option<bool>)
        -> i32;

    /// Determine the duration a specific mouse button is up.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    /// * `as_time` - If true then returns duration as milliseconds, else returns duration as frame updates. (optional, default: true)
    /// * `is_previous` - If true then returns the previous duration up (the time unused prior to the most recent press). (optional, default: false)
    ///
    /// Return: The duration a specific mouse button is up.
    /// 
    fn get_button_up_duration(&self, kind: Option<MouseButton>, as_time: Option<bool>, is_previous: Option<bool>) -> i32;

    /// Determine the horizontal movement of the mouse since a specific mouse button was pressed.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    ///
    /// Return: The horizontal movement of the mouse.
    /// 
    fn get_button_drag_width(&self, kind: Option<MouseButton>) -> i32;

    /// Determine the vertical movement of the mouse since a specific mouse button was pressed.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    ///
    /// Return: The vertical movement of the mouse.
    /// 
    fn get_button_drag_height(&self, kind: Option<MouseButton>) -> i32;

    /// Determine the horizontal position of the mouse when a specific mouse button was last clicked.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    ///
    /// Return: The horizontal position of the mouse.
    /// 
    fn get_button_last_clicked_x(&self, kind: Option<MouseButton>) -> i32;

    /// Determine the vertical position of the mouse when a specific mouse button was last clicked.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The mouse button. (optional)
    ///
    /// Return: The vertical position of the mouse.
    /// 
    fn get_button_last_clicked_y(&self, kind: Option<MouseButton>) -> i32;
}
