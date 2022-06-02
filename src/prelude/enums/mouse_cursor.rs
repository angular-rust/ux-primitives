/// Representing the mouse cursor for use in the `InputMouse` virtual controller.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseCursor {
    /// Arrow mouse cursor
    Arrow,
    /// Auto mouse cursor
    Auto,
    /// Button mouse cursor
    Button,
    /// Hand mouse cursor
    Hand,
    /// Beam mouse cursor
    Beam,
    /// Custom mouse cursor
    Custom { 
        /// Custom value
        value: u32 
    },
}
