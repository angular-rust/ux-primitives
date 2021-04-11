#![cfg(feature = "canvas")]

mod canvas;
pub use canvas::*;

mod style;
pub use style::*;

mod image;
pub use image::*;

mod direction;
pub use direction::*;

mod text_metrics;
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