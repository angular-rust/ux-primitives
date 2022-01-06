use crate::prelude::SceneKind;

use super::EngineScene;

/// The SceneManager should be implemented by objects intended to manage the [Scene] state machine.
/// 
/// Only a single scene is active at any given update.  Which scene is configured by this manager.
///
pub trait SceneManager {
    /// The currently active scene.
    /// Use as a runtime property and not as an initialization property.
    fn scene(&self) -> Box<dyn EngineScene>;

    /// Sets the current scene to a new scene.
    /// 
    /// # Arguments
    ///
    /// * `kind` - The new scene.
    /// 
    fn set_scene(&self, kind: SceneKind);

    /// Sets the current scene to the scene returned by Factory::get_back_scene_type().
    /// The new scene should be representative of retreat.
    /// @see Factory::get_back_scene_type
    fn back(&self);

    /// Sets the current scene to the scene returned by Factory::get_next_Scene_type().
    /// The new scene should be representative of progress.
    /// @see Factory::get_next_scene_type
    fn next(&self);

    /// Restarts the current scene.
    /// Equivalent of disposing current scene and then setScene to current scene again.
    fn restart(&self);
}
