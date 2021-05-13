mod style;
pub use style::*;

mod canvas;
pub use canvas::*;

pub mod colorspace;

mod text;
pub use text::*;

pub mod color;

pub mod prelude {
    pub use super::Object;
    pub use super::Is;
    pub use super::canvas::CanvasContext;
    pub use super::color;
    pub use crate::colorspace::prelude::*;
    pub use crate::text::*;
}

/// Important part of UX Framework
pub trait Object: 'static {}
pub trait Is<T: Object>: AsRef<T> + 'static {}

/// Alias for `euclid::default::Point2D`.
pub use euclid::default::Point2D as Point;

/// Alias for `euclid::default::Vector2D`.
pub use euclid::default::Vector2D as Vector;

/// Alias for `euclid::default::Size2D`.
pub use euclid::default::Size2D as Size;

/// Alias for `euclid::default::Rect`
pub use euclid::default::Rect;

/// Alias for `euclid::default::Transform2D`
pub type Transform<S> = euclid::default::Transform2D<S>;

/// Alias for `euclid::default::Rotation2D`
pub type Rotation<S> = euclid::default::Rotation2D<S>;

/// Alias for `euclid::default::Translation2D`
pub type Translation<S> = euclid::Translation2D<S, euclid::UnknownUnit, euclid::UnknownUnit>;