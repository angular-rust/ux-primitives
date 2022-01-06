use super::{Disposable, Positionable, Priority, Updateable};

/// The View should be implemented by all objects in the view broad phase traversal stack.
///
pub trait ViewObject: Priority + Positionable + Disposable + Updateable {
    // /// Optional: the object who this view represents.
    // fn owner(&self) -> T;

    /// The parent view of this view.
    /// The reference is null if this view has no parent (for exemple a view not in the view traversal stack).
    fn parent(&self) -> Box<dyn ViewObject>;

    /// Get the visibility of this view.
    fn is_visible(&self) -> bool;

    /// Specify the visibility of this view.
    /// If true the view will be displayed, if false the view is hidden.
    fn set_visible(&self, val: bool);

    /// Determined by whether this view is visible and included in a visible branch of the view stack (i.e. actually has the potential to be drawn within the overlay).
    /// If true the view is potentially visible, if false the view is impossible to be seen.
    fn is_in_view_stack(&self) -> bool;

    /// The horizontal position considering all parent's positions / scene graph.
    fn global_x(&self) -> f32;

    /// The vertical position considering all parent's positions / scene graph.
    fn global_y(&self) -> f32;

    /// Adds a new view child to this view.
    /// A view can have multiple children, and when you add a child to a view, it is automatically connected to the parent node through its parent property.
    /// 
    /// # Arguments
    /// 
    /// * `child` - The child view to add.
    /// * `priority` - The sorting priority of the child view to add.  Higher numbers will appear towards the top of the view stack.  Default value is 0.
    ///
    /// Return: Added view (to allow decoration).  Or null if addition was unsuccessful.
    /// 
    fn add_child(&self, child: Box<dyn ViewObject>, priority: Option<i32>) -> Box<dyn ViewObject>;

    /// Remove the specified view.
    /// The removed view will no longer be included in the view traversal stack so will no longer be visible.
    /// The view itself is still in memory, if you want to free them completely call child.dispose().
    /// 
    /// # Arguments
    /// 
    /// * `child` - The view to remove.
    /// 
    fn remove_child(&self, child: Box<dyn ViewObject>);

    /// Removes all child views.
    /// The children are still in memory, if you want to free them completely call view.dispose() from their owner object.
    fn clear(&self);

    /// Removes this view from the view traversal stack and subsequently all of its child views.
    /// The view itself is still in memory, if you want to free it completely call dispose().
    fn remove(&self);
}
