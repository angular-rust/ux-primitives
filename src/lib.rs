pub mod color;

mod colorspace;
pub use colorspace::*;

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

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

pub mod prelude {
    pub use super::color;
    pub use super::colorspace::*;
}
