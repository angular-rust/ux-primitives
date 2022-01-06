/// The Updatable should be implemented by any object wishing to enter the broad phase update traversal stack.
///
pub trait Updateable {
    /// The age of the object.
    /// 
    /// # Arguments
    /// 
    /// * `as_time` - If true treats the time as milliseconds, otherwise as frame updates. (optional, default true)
    ///
    /// Return: The age of the object (as elapsed time, not time since birth).
    /// 
    fn age(&self, as_time: Option<bool>) -> i32;

    /// Used to modify the internal state according to object specific logic and the elapsed time.
    /// This method is called internally by the framework, it will rarely need to be called directly.
    /// 
    /// # Arguments
    /// 
    /// * `deltaTime` - The time elapsed between this update and the previous update.  
    /// Can be used to accurately influence rate of change - e.g. speed. (optional, default: 0)
    /// 
    fn update(&self, delta_time: Option<i32>);
}
