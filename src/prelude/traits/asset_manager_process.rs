use super::{AssetManager, Process};

/// The AssetManagerProcess should be implemented by objects representing an operating [AssetManager].
/// 
/// These extra interface requirements are required for internal workings, but are not exposed via the minimal [AssetManager] interface.
///
pub trait AssetManagerProcess: AssetManager + Process {}
