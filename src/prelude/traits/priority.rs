/// The Priority should be implemented by objects intended to be ranked or sorted.
///
pub trait Priority {
    /// The rank score of this item.
    /// Higher numbers should be considered on top of the list, therefore of higher priority.
    fn priority(&self) -> i32;

    fn set_priority(&self, val: i32);
}
