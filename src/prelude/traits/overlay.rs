use crate::prelude::OverlayButton;

use super::Entity;

/// The Overlay should be implemented by objects representing the top most visual element of the application.
/// 
/// The overlay is intended to provide application wide border / chrome with controls such as back, mute, pause etc.
/// The overlay also provides flashing which is a useful cheap effect across many game scenarios.
///
pub trait Overlay {
    /// An optional [Entity] which is displayed and updated when the game is paused.
    fn pause_entity(&self) -> Box<dyn Entity>;

    fn set_pause_entity(&self, val: Box<dyn Entity>);

    /// Sets the visibility of a specific overlay button.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The overlay button.
    /// * `isVisible` - If true shows the button, if false hides it. (optional, default: true)
    /// 
    fn show_button(&self, kind: OverlayButton, is_visible: Option<bool>);

    /// Set the position of a specific overlay button.
    /// 
    /// # Arguments
    ///
    /// * `kind` - The overlay button.
    /// * `x` - The horizontal position.
    /// * `y` - The vertical position.
    /// * `width` - The width of the button hitArea, if null will not redefine. (optional)
    /// * `height` - The height of the button hitArea, if null will not redefine. (optional)
    /// 
    fn position_button(&self, kind: OverlayButton, x: f32, y: f32, width: Option<f32>, height: Option<f32>);

    /// Triggers an overlay button (as if it was clicked on).
    /// 
    /// # Arguments
    ///
    /// * `kind` - The overlay button.
    /// 
    fn activate_button(&self, kind: OverlayButton);

    /// Simple representation of progress.
    /// 
    /// # Arguments
    ///
    /// * `progress` - Range: 0...1.
    /// * `message` - An optional string to display. (optional)
    /// 
    fn show_progress(&self, progress: f32, message: Option<String>);

    /// Hides all overlay buttons.
    fn hide_buttons(&self);

    /// Creates a flash over the top of everything under the overlay.  Fades to invisible over a period of time.
    /// 
    /// # Arguments
    ///
    /// * `duration` - The period of time over which the flash should fade to 0. (optional)
    /// * `as_time` - If true treats the time as milliseconds, otherwise as frame updates. (optional, default: true)
    /// * `startingAlpha` - The alpha at which the flash starts.  Range: 0...1. (optional, default: 1.0)
    /// * `color` - The color of the flash.  Defaults to white. (optional, default: white)
    fn flash(&self, duration: Option<f32>, as_time: Option<bool>, starting_alpha: Option<f32>, color: Option<i32>);
}
