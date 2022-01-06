use crate::prelude::{FullScreen, JoypadTouch, Key, SceneKind, TextStyleKind};

use super::{
    AssetManagerProcess, Encrypter, Kernel, Logger, OverlayProcess, Preloader, EngineScene, SceneTransition,
    Session, TextStyle,
};

/// The Factory should be implemented by objects designed to populate an implementation.
/// 
/// The Factory represents the blueprint and builder for all project specific classes.
///
pub trait EngineFactory {
    /// The unique identifier for this specific project.  <=16 chars, no spaces.
    fn id(&self) -> String;

    /// The current version of this specific project.  Suggestion: major.minor.revision - e.g. 1.2.345
    fn version(&self) -> String;

    /// The author or this specific project.
    fn author(&self) -> String;

    /// A convenient switch to allow debug modes or verbose output in your code.  Adjust as needed.
    fn is_debug(&self) -> bool;

    /// A convenient switch to force all loaded content to be freshly loaded each request (rather than caching).
    fn is_decached(&self) -> bool;

    /// Disable to hide any eye candy options.
    fn is_eyecandy_option_enabled(&self) -> bool;

    /// Disable to hide any full screen options.
    fn is_fullscreen_option_enabled(&self) -> bool;

    /// Disable to hide any session resetting options.
    fn is_reset_sessions_option_enabled(&self) -> bool;

    /// The horizontal width of this application's bounding rectangle.
    fn width(&self) -> i32;

    /// The vertical height of this application's bounding rectangle.
    fn height(&self) -> i32;

    /// The default background color of the application's bounding rectangle.
    fn bg_color(&self) -> i32;

    /// The default scaling used for fullScreen mode.
    fn fullscreen_type(&self) -> FullScreen;

    /// The default handler for joypadTouch mode.
    fn joypad_touch_type(&self) -> JoypadTouch;

    /// The default secret key used to encrypt data.  Set it to something specific for your project, and conceal it's value.
    fn secret(&self) -> String;

    /// The intended frequency of the update broad phase traversal stack.  Technical limitations may prevent desired framerate from occurring.
    fn target_framerate(&self) -> i32;

    /// If true will send the time between each update as if the targetFramerate was hit perfectly.  If false will send the actual time between each update (which will vary from update to update).
    fn is_fixed_updates(&self) -> bool;

    // /// Dictionary of values.  Can be used to load initial configuration settings or store global variables.
    // fn config(&self) -> HashMap<String, T>;

    /// The scene which is displayed first.  The application starts here.
    fn starting_scene_type(&self) -> SceneKind;

    /// The default key used in this application to pause updates.
    fn key_pause(&self) -> Key;

    /// The default key used in this application to mute the audio.
    fn key_mute(&self) -> Key;

    /// The default key used in this application to back out of the current scene.
    fn key_back(&self) -> Key;

    /// The default key used in this application to advance to the next scene.
    fn key_next(&self) -> Key;

    /// The default key used in this application to do a special action (determined by the specific application).
    fn key_special(&self) -> Key;

    /// Called by the kernel to complete initialization (due to both requiring an initialized instance of each).
    /// 
    /// # Arguments
    /// 
    /// * `kernel` - An intialized kernel offering services to the factory.
    /// 
    fn on_init_complete(&self, kernel: Box<dyn Kernel>);

    /// Builds the application's asset manager which store images, sounds etc.
    /// Return Asset manager.
    fn create_asset_manager(&self) -> Box<dyn AssetManagerProcess>;

    /// Builds the application's encrypter to encrypt sensitive data / assets.
    /// Return Encrypter to encrypt sensitive data / assets.
    fn create_encrypter(&self) -> Box<dyn Encrypter>;

    // /// Builds an empty Entity for injection.
    // /// Return An empty Entity.
    // /// * `id` - The unique identifier of this entity. (optional)
    // fn create_entity(&self, id: Option<T>) -> Box<dyn Entity>;

    /// Builds the application's logger to log events / analytics.
    /// Return Logger to log events / analytics.
    fn create_logger(&self) -> Box<dyn Logger>;

    /// Builds the application's overlay to decorate and provide top level functionality.
    /// Return Overlay to decorate and provide top level functionality.
    fn create_overlay(&self) -> Box<dyn OverlayProcess>;

    /// Builds the application's preloader to load initial media assets.
    /// Return Preloader to load initial media assets.
    fn create_preloader(&self) -> Box<dyn Preloader>;

    /// Builds the application's scenes which contain specific functionality.
    /// Return Scene which contain specific functionality.
    /// 
    /// * `kind` - The type of scene.
    ///     
    fn create_scene(&self, kind: SceneKind) -> Box<dyn EngineScene>;

    /// Builds the application's transition between scenes.  Can be individually tailored for any combination of incoming and outgoing scene.
    /// Return Transition between scenes.
    /// * `kind_incoming` - The type of the incoming scene. (optional)
    /// * `kind_outgoing` - The type of the outgoing scene. (optional)
    ///     
    fn create_scene_transition(
        &self,
        kind_incoming: Option<SceneKind>,
        kind_outgoing: Option<SceneKind>,
    ) -> Box<dyn SceneTransition>;

    /// Builds the application's session to store user progress.
    /// Return Session to store user progress
    /// # Arguments
    /// 
    /// * `id` - The unique identifier of the session.  If session already exists will load existing. (optional)
    /// 
    fn create_session(&self, id: Option<String>) -> Box<dyn Session>;

    /// Builds the application's textStyle to configure font formatting.
    /// Return TextStyle to configure font formatting.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - The type of textStyle. (optional)
    /// 
    fn create_text_style(&self, kind: Option<TextStyleKind>) -> Box<dyn TextStyle>;

    /// When a scene is backed out of it will be replaced by the scene returned here.
    /// Return Scene type to back out to.
    /// 
    /// # Arguments
    /// 
    /// * `kind` - Type of scene to back out from.
    /// 
    fn get_back_scene_type(&self, kind: SceneKind) -> SceneKind;

    /// When a scene requests the next scene it will be replaced by the scene returned here.
    /// Return Scene type to advance to next.
    /// # Arguments
    /// 
    /// * `kind` - Type of scene to advance from.
    /// 
    fn get_next_scene_type(&self, kind: SceneKind) -> SceneKind;
}
