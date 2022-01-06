use super::{Disposable, Pauseable, Updateable};

/// The Process represents the smallest atom of the framework.
/// 
/// Many managers will implement this interface.
///
pub trait Process: Updateable + Disposable + Pauseable {}
