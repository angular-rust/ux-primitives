/// The Session should be implemented by objects wishing to preserve interactive information interchange.
/// 
/// The session can be used as both memento and arbitrator between subsystems - including entities and scenes.
/// Sessions can be permanently stored and retrieved on future application ezecution.
///
pub trait Session {
    /// The unique identifier of this session.
    fn id(&self) -> String;

    /// Useful for testing and debug.
    fn is_tester(&self) -> bool;

    /// Creates a copy of the current session.
    /// 
    /// # Arguments
    ///
    /// * `newId` - The unique identifier of the copy.
    ///
    /// Return: A copy of the current session.
    /// 
    fn clone(&self, new_id: String) -> Box<dyn Session>;

    /// Reverts the session back to factory settings (as if created afresh).
    /// 
    /// # Arguments
    ///
    /// * `isSaved` - If true immediately writes the reset session to disk. (optional, default: false)
    /// 
    fn reset(&self, is_saved: Option<bool>);

    /// Writes the session to disk.
    fn save(&self);

    /// Removes the session from disk.
    fn delete(&self);

    /// Helper functio'n to calculate overall progress of a game / rewards acquired etc.
    ///
    /// Return: Range 0...1: with 1 representing complete.
    fn percentage_complete(&self) -> f32;

    /// Retrieve the collection of sessions identifiers currently saved to disk.
    /// 
    /// # Arguments
    ///
    /// * `suggestions` - Prepopulates results with the suggestions. (optional)
    ///
    /// Return: A collection of sessions identifiers currently save to disk.
    /// 
    fn get_session_ids(&self, suggestions: Option<Vec<String>>) -> Vec<String>;

    /// Retrieve the collection of sessions currently saved to disk.
    /// 
    /// # Arguments
    ///
    /// * `suggestions` - Prepopulates results with the suggestions. (optional)
    ///
    /// Return: A collection of sessions currently save to disk.
    /// 
    fn get_sessions(&self, suggestions: Option<Vec<String>>) -> Vec<String>;

    /// Removes all session data from disk.
    fn delete_all_sessions(&self);
}
