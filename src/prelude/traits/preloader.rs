use super::{Process, Progress, Viewable};

/// The Preloader should be implemented by objects intended to act as preloaders.
///
pub trait Preloader: Process + Viewable + Progress {}
