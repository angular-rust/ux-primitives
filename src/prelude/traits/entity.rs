use super::{EntityCollection, Process, Viewable};

/// The Entity should be implemented by all objects in the entity broad phase traversal stack.
/// 
/// The Entity represents the fundamental building block and provides sufficient functionality to build most game elements.
/// Project specific entities can be created as custom classes, or by injecting functionality through the Entity interface.
///
pub trait Entity: Process + Viewable + EntityCollection {
    /// The unique identifier of this entity.
    /// This value is very useful for retrieving a specific entity.
    fn id(&self) -> String;

    /// Set the unique identifier of this entity.
    fn set_id(&self, val: String);

    /// The parent of this entity
    /// The reference is null if this entity has no parent (for example an entity not in the entity traversal stack).
    /// Consider this a runtime only property, rather than calling it during constructor or initialization phases.
    fn parent(&self) -> Box<dyn Entity>;

    /// Used to easily remove this entity from its parent.
    /// 
    /// # Arguments
    /// 
    /// * `isRemovedFromView` - Determines whether this object's view is removed from the view stack at the same time. (optional, default: false)
    /// 
    fn remove(&self, is_removed_from_view: Option<bool>);
}
