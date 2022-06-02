use super::Point;

/// A 2d axis aligned rectangle represented by its minimum and maximum coordinates.
///
/// # Representation
///
/// This struct is similar to [`Rect`], but stores rectangle as two endpoints
/// instead of origin point and size. Such representation has several advantages over
/// [`Rect`] representation:
/// - Several operations are more efficient with `Box2D`, including [`intersection`],
///   [`union`], and point-in-rect.
/// - The representation is less susceptible to overflow. With [`Rect`], computation
///   of second point can overflow for a large range of values of origin and size.
///   However, with `Box2D`, computation of [`size`] cannot overflow if the coordinates
///   are signed and the resulting size is unsigned.
///
/// A known disadvantage of `Box2D` is that translating the rectangle requires translating
/// both points, whereas translating [`Rect`] only requires translating one point.
///
/// # Empty box
///
/// A box is considered empty (see [`is_empty`]) if any of the following is true:
/// - it's area is empty,
/// - it's area is negative (`min.x > max.x` or `min.y > max.y`),
/// - it contains NaNs.
///
/// [`Rect`]: struct.Rect.html
/// [`intersection`]: #method.intersection
/// [`is_empty`]: #method.is_empty
/// [`union`]: #method.union
/// [`size`]: #method.size
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
)]
pub struct Box2D<T> {
    /// Represents min point
    pub min: Point<T>,
    /// Represents max point
    pub max: Point<T>,
}
