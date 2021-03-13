mod color;
pub use color::*;

mod text;
pub use text::*;

#[cfg(feature = "canvas")]
mod canvas;
#[cfg(feature = "canvas")]
pub use canvas::*;

#[cfg(any(feature = "geom", feature = "canvas"))]
mod geom;
#[cfg(any(feature = "geom", feature = "canvas"))]
pub use geom::*;

#[macro_use]
extern crate lazy_static;