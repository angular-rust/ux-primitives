use super::{
    AssetManager, AudioManager, EngineFactory, InputManager, Logger, Overlay, Pauseable, Preloader, SceneManager,
    Session, Tools,
};

/// Handles main updates and provides global locators for all managers
///
pub trait Kernel: Pauseable + Logger {
    /// Defined by the [Factory], can be used for conditional logic relating to build modes and debug.
    fn is_debug(&self) -> bool;

    /// Identifies a non network location, can be used for conditional logic relating to build modes and debug.
    fn is_local(&self) -> bool;

    /// Toggleable by the user, intended to be used as a switch to disable intensive, but non essential, content (performance vs wow).
    fn is_eyecandy(&self) -> bool;

    fn set_eyecandy(&self, val: bool);

    /// Toggleable by the user, enables or disables full screen mode.
    fn is_fullscreen(&self) -> bool;

    fn set_fullscreen(&self, val: bool);

    /// The topmost visual element, used for chrome & global controls.
    fn overlay(&self) -> Box<dyn Overlay>;

    /// Assets manager.
    fn assets(&self) -> Box<dyn AssetManager>;

    /// Audio manager.
    fn audio(&self) -> Box<dyn AudioManager>;

    /// Inputs manager.
    fn inputs(&self) -> Box<dyn InputManager>;

    /// Scene manager.  State machine containing Entities.
    fn scenes(&self) -> Box<dyn SceneManager>;

    // /// Messenger manager.  Arbitrator for observer pattern across EntityCollections.
    // fn messenger(&self) -> Box<dyn MessageManager>;

    /// Helper methods.
    fn tools(&self) -> Box<dyn Tools>;

    /// Build properties and factory methods to create the application.
    fn factory(&self) -> Box<dyn EngineFactory>;

    /// Read and write globally accessible variables.
    fn session(&self) -> Box<dyn Session>;

    fn set_session(&self, val: Box<dyn Session>);

    // /// Used for read only application settings and localisation text.
    // /// 
    // /// # Arguments
    // /// 
    // /// * `id` - The unique identifier for the config setting (e.g. XML node name).
    // /// @return	Value of the corresponding config setting.
    // fn get_config(&self, id: String) -> T;

    /// Request the framerate of the application.
    /// 
    /// # Arguments
    /// 
    /// * `asActual` - Use actual framerate (potentially laggy), or the desired framerate (from [Factory]). (optional: default: true)
    ///
    /// Return: Frames per second.
    /// 
    fn get_framerate(&self, as_actual: Option<bool>) -> f32;

    /// Internal method called when preloader completes; launches the starting scene as defined by Factory::startingSceneType.
    /// 
    /// # Arguments
    /// 
    /// * `preloader` - Corresponding [Preloader].
    /// 
    fn on_preloader_complete(&self, preloader: Box<dyn Preloader>);
}
