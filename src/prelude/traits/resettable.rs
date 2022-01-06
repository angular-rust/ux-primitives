/// The Resettable should be implemented by objects intended to be reset (returned to initial state).
///
pub trait Resettable {
    /// Call method to return object to it's initial state.
    ///
    /// Return: True if reset was successful, false otherwise.
    fn reset(&self) -> bool;
}
