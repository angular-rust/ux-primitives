/// The Pauseable should be implemented by objects intended to be temporarily disabled 
/// from the broad phase update traversal.
///
pub trait Pauseable {
    /// Determines if the object is updating or not.
    fn is_active(&self) -> bool;

    fn set_active(&self, val: bool);

    /// Sets active to false.
    fn pause(&self);

    /// Sets active to true.
    fn resume(&self);
}
