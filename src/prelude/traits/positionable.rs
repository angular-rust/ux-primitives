/// The Positionable should be implemented by objects intended to have 2D spatial position.
///
pub trait Positionable {
    /// The horizontal position.
    fn x(&self) -> f32;

    fn set_x(&self, val: f32);

    /// The vertical position.
    fn y(&self) -> f32;

    fn set_y(&self, val: f32);

    /// Sets both the horizontal and vertical position
    /// 
    /// # Arguments
    ///
    /// * `x` - The horizontal position.
    /// * `y` - The vertical position.
    /// 
    fn set_position(&self, x: f32, y: f32);
}
