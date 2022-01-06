/// The Disposable should be implemented by objects that require
/// specialised garbage collection or memory deallocation.
/// 
/// Once an object is disposed it should be automatically removed from parent heirachies.
///
pub trait Disposable {
    /// Returns true if the object has been disposed (or is being disposed).
    fn is_disposed(&self) -> bool;

    /// Disposes this object by deallocating all used resources and breaking all
    /// (non-weak) references to other objects. This method must be the final
    /// call in the object's life cycle. No methods except this method should be
    /// called on the object and no properties of the object should be read or
    /// written after a call to this object; otherwise the behaviour is
    /// unreliable. The object may call the method on itself, directly or
    /// indirectly.
    fn dispose(&self);
}
