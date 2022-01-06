use super::{Overlay, Process, Viewable};

/// The OverlayProcess should be implemented by objects representing an operating [Overlay].
/// 
/// These extra interface requirements are required for internal workings, 
/// but are not exposed via the minimal [Overlay] interface.
///
pub trait OverlayProcess: Overlay + Process + Viewable {}
