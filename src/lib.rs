pub mod color;
pub mod text;

#[cfg(feature = "canvas")]
pub mod canvas;

#[cfg(any(feature = "geom", feature = "canvas"))]
pub mod geom;

#[macro_use]
extern crate lazy_static;