//! The `primitives` prelude.
//!
//! The purpose of this module is to alleviate imports of many common primitives
//! traits by adding a glob import to the top of primitives heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use primitives::prelude::*;
//! ```

mod enums;
pub use self::enums::*;

mod traits;
pub use self::traits::*;

mod canvas;
pub use canvas::*;

pub mod color;

/// This is the basic glue for the  UX Framework
///
pub trait Object: 'static {}

/// Marks structures as framework [Object]'s
///
pub trait Is<T: Object>: AsRef<T> + 'static {}

pub use crate::foundation::colorspace::{
    Color,
    prelude::*
};

/// Creates a structure with zero values.
///
/// This interface is obsolete.
///
pub trait Zero {
    /// Creates a structure with zero values.
    fn zero() -> Self;
}

/// Creates a structure with values equal to one.
///
/// This interface is obsolete.
///
pub trait One {
    /// Creates a structure with values equal to one.
    fn one() -> Self;
}
