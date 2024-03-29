//! The library provides the basic building blocks for creating games and regular applications. 
//! It contains behavior definitions for game applications and basic structures.
//! 
//! The basic fundamental entity for graphical applications such as games is color. 
//! This library contains sufficient functionality for working with color, including storage, 
//! processing and conversion between different color spaces. 
//! 
//! It also provides support for multiple color schemes and contains the 
//! OpenColor palette for the convenience and higher quality of your applications.


#![doc(html_logo_url = "https://dudochkin-victor.github.io/assets/ux-primitives/logo.svg")]

#![warn(missing_docs)]

pub mod foundation;

pub mod prelude;


// /// A one-dimensional distance, with value represented by `T` and unit of measurement `Unit`.
// ///
// /// `T` can be any numeric type, for example a primitive type like `u64` or `f32`.
// ///
// /// `Unit` is not used in the representation of a `Length` value. It is used only at compile time
// /// to ensure that a `Length` stored with one unit is converted explicitly before being used in an
// /// expression that requires a different unit.  It may be a type without values, such as an empty
// /// enum.
// ///
// /// You can multiply a `Length` by a `scale::Scale` to convert it from one unit to
// /// another. See the [`Scale`] docs for an example.
// ///
// /// [`Scale`]: struct.Scale.html
// #[repr(C)]
// pub struct Length<T, Unit>(pub T, #[doc(hidden)] pub PhantomData<Unit>);

// /// A 2d size tagged with a unit.
// #[repr(C)]
// pub struct Size2D<T, U> {
//     /// The extent of the element in the `U` units along the `x` axis (usually horizontal).
//     pub width: T,
//     /// The extent of the element in the `U` units along the `y` axis (usually vertical).
//     pub height: T,
//     #[doc(hidden)]
//     pub _unit: PhantomData<U>,
// }

// /// A 3d size tagged with a unit.
// #[repr(C)]
// pub struct Size3D<T, U> {
//     /// The extent of the element in the `U` units along the `x` axis.
//     pub width: T,
//     /// The extent of the element in the `U` units along the `y` axis.
//     pub height: T,
//     /// The extent #![warn(missing_docs)]of the element in the `U` units along the `z` axis.
//     pub depth: T,
//     #[doc(hidden)]
//     pub _unit: PhantomData<U>,
// }

// /// A 2d Rectangle optionally tagged with a unit.
// ///
// /// # Representation
// ///
// /// `Rect` is represented by an origin point and a size.
// ///
// /// See [`Box2D`] for a rectangle represented by two endpoints.
// ///
// /// # Empty rectangle
// ///
// /// A rectangle is considered empty (see [`is_empty`]) if any of the following is true:
// /// - it's area is empty,
// /// - it's area is negative (`size.x < 0` or `size.y < 0`),
// /// - it contains NaNs.
// ///
// /// [`is_empty`]: #method.is_empty
// /// [`Box2D`]: struct.Box2D.html
// #[repr(C)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(
//     feature = "serde",
//     serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
// )]
// pub struct Rect<T, U> {
//     pub origin: Point2D<T, U>,
//     pub size: Size2D<T, U>,
// }

// /// A 2d axis aligned rectangle represented by its minimum and maximum coordinates.
// ///
// /// # Representation
// ///
// /// This struct is similar to [`Rect`], but stores rectangle as two endpoints
// /// instead of origin point and size. Such representation has several advantages over
// /// [`Rect`] representation:
// /// - Several operations are more efficient with `Box2D`, including [`intersection`],
// ///   [`union`], and point-in-rect.
// /// - The representation is less susceptible to overflow. With [`Rect`], computation
// ///   of second point can overflow for a large range of values of origin and size.
// ///   However, with `Box2D`, computation of [`size`] cannot overflow if the coordinates
// ///   are signed and the resulting size is unsigned.
// ///
// /// A known disadvantage of `Box2D` is that translating the rectangle requires translating
// /// both points, whereas translating [`Rect`] only requires translating one point.
// ///
// /// # Empty box
// ///
// /// A box is considered empty (see [`is_empty`]) if any of the following is true:
// /// - it's area is empty,
// /// - it's area is negative (`min.x > max.x` or `min.y > max.y`),
// /// - it contains NaNs.
// ///
// /// [`Rect`]: struct.Rect.html
// /// [`intersection`]: #method.intersection
// /// [`is_empty`]: #method.is_empty
// /// [`union`]: #method.union
// /// [`size`]: #method.size
// #[repr(C)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(
//     feature = "serde",
//     serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
// )]
// pub struct Box2D<T, U> {
//     pub min: Point2D<T, U>,
//     pub max: Point2D<T, U>,
// }

// /// An axis aligned 3D box represented by its minimum and maximum coordinates.
// #[repr(C)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(
//     feature = "serde",
//     serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
// )]
// pub struct Box3D<T, U> {
//     pub min: Point3D<T, U>,
//     pub max: Point3D<T, U>,
// }

// /// A group of 2D side offsets, which correspond to top/right/bottom/left for borders, padding,
// /// and margins in CSS, optionally tagged with a unit.
// #[repr(C)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(
//     feature = "serde",
//     serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
// )]
// pub struct SideOffsets2D<T, U> {
//     pub top: T,
//     pub right: T,
//     pub bottom: T,
//     pub left: T,
//     #[doc(hidden)]
//     pub _unit: PhantomData<U>,
// }