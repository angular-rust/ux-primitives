//! The foundation for UX-Framework of AngularRust

pub mod colorschemes;
pub mod colorspace;

mod angle;
pub use angle::*;

mod box2d;
pub use box2d::*;

// mod box3d;
// pub use box3d::*;

mod image;
pub use image::*;

mod length;
pub use length::*;

mod offset;
pub use offset::*;

// mod point2d;
// pub use point2d::*;

// mod point3d;
// pub use point3d::*;

mod rect;
pub use rect::*;

mod size2d;
pub use size2d::*;

// mod size3d;
// pub use size3d::*;

mod style;
pub use style::*;

mod text;
pub use text::*;

mod vector2d;
pub use vector2d::*;

mod vector3d;
pub use vector3d::*;

/// Alias for `crate::Size2D`.
pub use Size2D as Size;
/// Alias for `crate::Point2`.
// pub use crate::Point2D as Point;
pub use Vector2D as Point;
pub use Vector2D as Point2D;
pub use Vector3D as Point3D;
