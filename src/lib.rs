#[cfg(feature = "canvas")]
pub mod canvas;
pub mod color;
#[cfg(any(feature = "geom", feature = "canvas"))]
pub mod geom;
pub mod text;

//#[macro_use]
#[cfg(any(feature = "color_quantization", test))]
extern crate lazy_static;

pub use prelude::*;

pub mod prelude {
    #[cfg(feature = "canvas")]
    pub use crate::canvas;
    #[cfg(feature = "canvas")]
    pub use crate::canvas::*;
    pub use crate::color;
    pub use crate::color::palette;
    pub use crate::color::prelude::*;
    #[cfg(any(feature = "geom", feature = "canvas"))]
    pub use crate::geom;
    #[cfg(any(feature = "geom", feature = "canvas"))]
    pub use crate::geom::*;
    pub use crate::text::*;
}
