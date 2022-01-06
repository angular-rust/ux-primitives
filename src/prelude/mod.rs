//! oops
//!
//!
//! <img src="https://github.com/hecrj/iced/blob/9712b319bb7a32848001b96bd84977430f14b623/examples/resources/ferris.png?raw=true" width="300">
//!
//!
//! [![Pane grid - Iced](https://thumbs.gfycat.com/MixedFlatJellyfish-small.gif)](https://www.youtube.com/watch?v=QQ7MimTj2vg)
//!

mod enums;
pub use self::enums::*;

mod traits;
pub use self::traits::*;

mod canvas;
pub use canvas::*;

pub mod color;

/// This is the basic glue for the  UX Framework
///
pub trait Object: 'static {}

/// Marks structures as framework [Object]'s
///
pub trait Is<T: Object>: AsRef<T> + 'static {}

pub use crate::foundation::colorspace::{
    Color,
    prelude::*
};

/// Creates a structure with zero values.
///
/// This interface is obsolete.
///
pub trait Zero {
    fn zero() -> Self;
}

/// Creates a structure with values equal to one.
///
/// This interface is obsolete.
///
pub trait One {
    fn one() -> Self;
}
