#![cfg(feature = "canvas")]

mod canvas;
mod style;
mod image;
mod direction;
mod text_metrics;
pub use canvas::*;
pub use style::*;
pub use image::*;
pub use direction::*;
pub use text_metrics::*;

#[derive(Clone, Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square
}

impl Default for LineCap {
    fn default() -> Self {
        LineCap::Butt
    }
}

#[derive(Clone, Debug)]
pub enum LineJoin {
    Miter,
    Bevel,
    Round
}

impl Default for LineJoin {
    fn default() -> Self {
        LineJoin::Miter
    }
}