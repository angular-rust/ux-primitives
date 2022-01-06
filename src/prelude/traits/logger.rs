/// The Logger should be implemented by objects wishing to act as logging services.
/// 
/// Logging services are useful for debugging or analytics.
/// The interface is intentionally vague to allow a multitude of implementations.
///
pub trait Logger {
    /// Pass a value to the logger to log.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value is string representation of logging message.
    /// 
    fn log(&self, value: String);
}
