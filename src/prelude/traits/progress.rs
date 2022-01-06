/// The Progress should be implemented by objects intended to progress from start to finish ( 0...1 ).
///
pub trait Progress {
    /// Range: 0...1.  0 represents just starting, 1 represents complete.
    fn progress(&self) -> f32;
}
