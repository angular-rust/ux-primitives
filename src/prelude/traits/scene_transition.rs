use super::{Process, Progress, Viewable};

/// The SceneTransition should be implemented by objects intended to display visual transitions during scene changes.
///
pub trait SceneTransition: Process + Progress + Viewable {
    /// The time over which the transition occurs.
    /// 
    /// # Arguments
    ///
    /// * `as_time` - If true returns duration in milliseconds, else updates. (optional, default: true)
    ///
    /// Return: The time over which the transition occurs.
    /// 
    fn get_duration(&self, as_time: Option<bool>) -> i32;
}
