pub mod color;
pub mod datatable;
pub mod text;

#[cfg(feature = "canvas")]
pub mod canvas;

#[cfg(any(feature = "geom", feature = "canvas"))]
pub mod geom;