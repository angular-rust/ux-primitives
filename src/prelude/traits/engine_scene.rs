use crate::prelude::SceneKind;

use super::{EntityCollection, Process, Viewable};

/// The Scene should be implemented by objects intending to Represents scene states in the `SceneManager`.
/// 
/// Scenes Represents the larger building blocks of the concept, and contain Entities which do the work.
///
pub trait EngineScene: Process + EntityCollection + Viewable {
    /// The kind of this scene.
    fn kind(&self) -> SceneKind;

    /// Sets whether the scene is disposed when no longer the active scene.  In most cases this should be true.
    fn is_disposable(&self) -> bool;

    /// Sets whether the pause button is displayed / active in the overlay.
    fn is_pauseable(&self) -> bool;

    /// Sets whether the mute button is displayed / active in the overlay.
    fn is_muteable(&self) -> bool;

    /// Sets whether the session is automatically saved when this scene is advanced.  In most cases this should be true.
    fn is_session_saved_on_next(&self) -> bool;
}
