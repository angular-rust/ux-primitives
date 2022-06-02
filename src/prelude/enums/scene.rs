/// Default Scene types. 
///  
/// A basic game can be made using these defaults.
/// Can be extended with Custom by using concrete project values.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneKind {
    /// Splash screen scene
    Splash,
    /// Intro screen scene
    Intro,
    /// SelectSession screen scene
    SelectSession,
    /// SelectStory screen scene
    SelectStory,
    /// SelectLevel screen scene
    SelectLevel,
    /// Instructions screen scene
    Instructions,
    /// Settings screen scene
    Settings,
    /// Menu screen scene
    Menu,
    /// Avatar screen scene
    Avatar,
    /// Shop screen scene
    Shop,
    /// Rewards screen scene
    Rewards,
    /// LeaderBoard screen scene
    LeaderBoard,
    /// Game screen scene
    Game,
    /// Interstitial screen scene
    Interstitial,
    /// Cinematic screen scene
    Cinematic,
    /// Results screen scene
    Results,
    /// Exit screen scene
    Exit,

    /// Recommended to be used as a testing sandbox to test new entities etc.
    Test,

    /// Allows [SceneKind] to be extended (e.g. for using project specific enumerated scene types).
    Custom {
        /// Custom value
        value: u32,
    },
}
