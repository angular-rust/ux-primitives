pub mod color;
pub mod text;
#[cfg(feature = "canvas")]
pub mod canvas;
#[cfg(any(feature = "geom", feature = "canvas"))]
pub mod geom;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use prelude::*;

pub mod prelude {
    pub use crate::color;
    pub use crate::color::prelude::*;
    pub use crate::color::palette;
    pub use crate::text::*;
    #[cfg(feature = "canvas")]
    pub use crate::canvas;
    #[cfg(feature = "canvas")]
    pub use crate::canvas::*;
    #[cfg(any(feature = "geom", feature = "canvas"))]
    pub use crate::geom;
    #[cfg(any(feature = "geom", feature = "canvas"))]
    pub use crate::geom::*;
}
