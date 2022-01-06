use super::ViewObject;

/// The Viewable should be implemented by all objects that compose a view.
///
pub trait Viewable {
    /// The view bound to this object.
    fn view(&self) -> Box<dyn ViewObject>;
}
