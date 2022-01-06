/// Representing the mouse cursor for use in the `InputMouse` virtual controller.
///
pub enum MouseCursor {
    Arrow,
    Auto,
    Button,
    Hand,
    Beam,
    Custom { value: u32 },
}
