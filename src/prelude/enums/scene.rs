/// Default Scene types. 
///  
/// A basic game can be made using these defaults.
/// Can be extended with Custom by using concrete project values.
///
pub enum SceneKind {
    Splash,
    Intro,
    SelectSession,
    SelectStory,
    SelectLevel,
    Instructions,
    Settings,
    Menu,
    Avatar,
    Shop,
    Rewards,
    LeaderBoard,
    Game,
    Interstitial,
    Cinematic,
    Results,
    Exit,

    /// Recommended to be used as a testing sandbox to test new entities etc.
    Test,

    /// Allows [SceneKind] to be extended (e.g. for using project specific enumerated scene types).
    Custom {
        value: u32,
    },
}
