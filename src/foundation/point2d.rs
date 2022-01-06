#![allow(unused_imports)]
use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use num_traits::{Float, NumCast};

/// A 2d Point tagged with a unit.
#[repr(C)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Copy for Point2D<T> {}

impl<T: Clone> Clone for Point2D<T> {
    fn clone(&self) -> Self {
        Point2D {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Point2D<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Point2D {
            x,
            y,
        })
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Point2D<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.x, &self.y).serialize(serializer)
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T> arbitrary::Arbitrary<'a> for Point2D<T>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self>
    {
        let (x, y) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(Point2D {
            x,
            y,
        })
    }
}
impl<T> Eq for Point2D<T> where T: Eq {}

impl<T> PartialEq for Point2D<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Hash for Point2D<T>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
    }
}

// mint_vec!(Point2D[x, y] = Point2);

impl<T: fmt::Debug> fmt::Debug for Point2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl<T: Default> Default for Point2D<T> {
    fn default() -> Self {
        Point2D::new(Default::default(), Default::default())
    }
}

impl<T> Point2D<T> {
    // /// Constructor, setting all components to zero.
    // #[inline]
    // pub fn origin() -> Self
    // where
    //     T: Zero,
    // {
    //     Point2D::new(Zero::zero(), Zero::zero())
    // }

    // /// The same as [`origin()`](#method.origin).
    // #[inline]
    // pub fn zero() -> Self
    // where
    //     T: Zero,
    // {
    //     Self::origin()
    // }

    /// Constructor taking scalar values directly.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Point2D {
            x,
            y,
        }
    }

    // /// Constructor taking properly Lengths instead of scalar values.
    // #[inline]
    // pub fn from_lengths(x: Length<T>, y: Length<T>) -> Self {
    //     Point2D::new(x.0, y.0)
    // }

    /// Constructor setting all components to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Point2D {
            x: v.clone(),
            y: v,
        }
    }

    // /// Tag a unitless value with units.
    // #[inline]
    // pub fn from_untyped(p: Point2D<T>) -> Self {
    //     Point2D::new(p.x, p.y)
    // }
}

impl<T: Copy> Point2D<T> {
    // /// Create a 3d point from this one, using the specified z value.
    // #[inline]
    // pub fn extend(self, z: T) -> Point3D<T> {
    //     point3(self.x, self.y, z)
    // }

    // /// Cast this point into a vector.
    // ///
    // /// Equivalent to subtracting the origin from this point.
    // #[inline]
    // pub fn to_vector(self) -> Vector2D<T> {
    //     Vector2D {
    //         x: self.x,
    //         y: self.y,
    //     }
    // }

    /// Swap x and y.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use primitives::Point2D;
    ///
    /// let point: Point2D<_> = Point2D::new(1, -8);
    ///
    /// assert_eq!(point.yx(), Point2D::new(-8, 1));
    /// ```
    #[inline]
    pub fn yx(self) -> Self {
        Point2D::new(self.y, self.x)
    }

    // /// Drop the units, preserving only the numeric value.
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// # use primitives::Point2D;
    // ///
    // /// let point: Point2D<_> = Point2D::new(1, -8);
    // ///
    // /// assert_eq!(point.x, point.to_untyped().x);
    // /// assert_eq!(point.y, point.to_untyped().y);
    // /// ```
    // #[inline]
    // pub fn to_untyped(self) -> Point2D<T> {
    //     Point2D::new(self.x, self.y)
    // }

    // /// Cast the unit, preserving the numeric value.
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// # use primitives::Point2D;
    // ///
    // /// let point: Point2D<_> = Point2D::new(1, -8);
    // ///
    // /// assert_eq!(point.x, point.cast_unit::<Cm>().x);
    // /// assert_eq!(point.y, point.cast_unit::<Cm>().y);
    // /// ```
    // #[inline]
    // pub fn cast_unit<V>(self) -> Point2D<T, V> {
    //     Point2D::new(self.x, self.y)
    // }

    /// Cast into an array with x and y.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use primitives::Point2D;
    ///
    /// let point: Point2D<_> = Point2D::new(1, -8);
    ///
    /// assert_eq!(point.to_array(), [1, -8]);
    /// ```
    #[inline]
    pub fn to_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    /// Cast into a tuple with x and y.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use primitives::Point2D;
    ///
    /// let point: Point2D<_> = Point2D::new(1, -8);
    ///
    /// assert_eq!(point.to_tuple(), (1, -8));
    /// ```
    #[inline]
    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    // /// Convert into a 3d point with z-coordinate equals to zero.
    // #[inline]
    // pub fn to_3d(self) -> Point3D<T>
    // where
    //     T: Zero,
    // {
    //     point3(self.x, self.y, Zero::zero())
    // }

    // /// Rounds each component to the nearest integer value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use primitives::Point2D;
    // ///
    // /// assert_eq!(Point2D::new::<_>(-0.1, -0.8).round(), Point2D::new::<_>(0.0, -1.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn round(self) -> Self
    // where
    //     T: Round,
    // {
    //     Point2D::new(self.x.round(), self.y.round())
    // }

    // /// Rounds each component to the smallest integer equal or greater than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use primitives::Point2D;
    // ///
    // /// assert_eq!(Point2D::new::<_>(-0.1, -0.8).ceil(), Point2D::new::<_>(0.0, 0.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn ceil(self) -> Self
    // where
    //     T: Ceil,
    // {
    //     Point2D::new(self.x.ceil(), self.y.ceil())
    // }

    // /// Rounds each component to the biggest integer equal or lower than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use primitives::Point2D;
    // ///
    // /// assert_eq!(Point2D::new::<_>(-0.1, -0.8).floor(), Point2D::new::<_>(-1.0, -1.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn floor(self) -> Self
    // where
    //     T: Floor,
    // {
    //     Point2D::new(self.x.floor(), self.y.floor())
    // }

    // /// Linearly interpolate between this point and another point.
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// use primitives::default::Point2D;
    // ///
    // /// let from: Point2D<_> = Point2D::new(0.0, 10.0);
    // /// let to:  Point2D<_> = Point2D::new(8.0, -4.0);
    // ///
    // /// assert_eq!(from.lerp(to, -1.0), Point2D::new(-8.0,  24.0));
    // /// assert_eq!(from.lerp(to,  0.0), Point2D::new( 0.0,  10.0));
    // /// assert_eq!(from.lerp(to,  0.5), Point2D::new( 4.0,   3.0));
    // /// assert_eq!(from.lerp(to,  1.0), Point2D::new( 8.0,  -4.0));
    // /// assert_eq!(from.lerp(to,  2.0), Point2D::new(16.0, -18.0));
    // /// ```
    // #[inline]
    // pub fn lerp(self, other: Self, t: T) -> Self
    // where
    //     T: One + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
    // {
    //     let one_t = T::one() - t;
    //     Point2D::new(one_t * self.x + t * other.x, one_t * self.y + t * other.y)
    // }
}

impl<T: PartialOrd> Point2D<T> {
    // #[inline]
    // pub fn min(self, other: Self) -> Self {
    //     Point2D::new(min(self.x, other.x), min(self.y, other.y))
    // }

    // #[inline]
    // pub fn max(self, other: Self) -> Self {
    //     Point2D::new(max(self.x, other.x), max(self.y, other.y))
    // }

    // /// Returns the point each component of which clamped by corresponding
    // /// components of `start` and `end`.
    // ///
    // /// Shortcut for `self.max(start).min(end)`.
    // #[inline]
    // pub fn clamp(self, start: Self, end: Self) -> Self
    // where
    //     T: Copy,
    // {
    //     self.max(start).min(end)
    // }
}

// impl<T: NumCast + Copy> Point2D<T> {
//     /// Cast from one numeric representation to another, preserving the units.
//     ///
//     /// When casting from floating point to integer coordinates, the decimals are truncated
//     /// as one would expect from a simple cast, but this behavior does not always make sense
//     /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
//     #[inline]
//     pub fn cast<NewT: NumCast>(self) -> Point2D<NewT> {
//         self.try_cast().unwrap()
//     }

//     /// Fallible cast from one numeric representation to another, preserving the units.
//     ///
//     /// When casting from floating point to integer coordinates, the decimals are truncated
//     /// as one would expect from a simple cast, but this behavior does not always make sense
//     /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
//     pub fn try_cast<NewT: NumCast>(self) -> Option<Point2D<NewT>> {
//         match (NumCast::from(self.x), NumCast::from(self.y)) {
//             (Some(x), Some(y)) => Some(Point2D::new(x, y)),
//             _ => None,
//         }
//     }

//     // Convenience functions for common casts

//     /// Cast into an `f32` point.
//     #[inline]
//     pub fn to_f32(self) -> Point2D<f32> {
//         self.cast()
//     }

//     /// Cast into an `f64` point.
//     #[inline]
//     pub fn to_f64(self) -> Point2D<f64> {
//         self.cast()
//     }

//     /// Cast into an `usize` point, truncating decimals if any.
//     ///
//     /// When casting from floating point points, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_usize(self) -> Point2D<usize> {
//         self.cast()
//     }

//     /// Cast into an `u32` point, truncating decimals if any.
//     ///
//     /// When casting from floating point points, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_u32(self) -> Point2D<u32> {
//         self.cast()
//     }

//     /// Cast into an i32 point, truncating decimals if any.
//     ///
//     /// When casting from floating point points, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_i32(self) -> Point2D<i32> {
//         self.cast()
//     }

//     /// Cast into an i64 point, truncating decimals if any.
//     ///
//     /// When casting from floating point points, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_i64(self) -> Point2D<i64> {
//         self.cast()
//     }
// }

// impl<T: Float> Point2D<T> {
//     /// Returns true if all members are finite.
//     #[inline]
//     pub fn is_finite(self) -> bool {
//         self.x.is_finite() && self.y.is_finite()
//     }
// }

// impl<T: Copy + Add<T, Output = T>> Point2D<T> {
//     #[inline]
//     pub fn add_size(self, other: &Size2D<T>) -> Self {
//         Point2D::new(self.x + other.width, self.y + other.height)
//     }
// }

// impl<T: Float + Sub<T, Output = T>> Point2D<T> {
impl<T: Float + Sub<T, Output = T>> Point2D<T> {
    #[inline]
    pub fn distance_to(self, other: Self) -> T {
        // (self - other).length()
        let point = self - other;
        (point.x * point.x + point.y * point.y).sqrt()
    }
}

impl<T: Neg> Neg for Point2D<T> {
    type Output = Point2D<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Point2D::new(-self.x, -self.y)
    }
}

impl<T: Add> Add for Point2D<T> {
    type Output = Point2D<T::Output>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Point2D::new(self.x + other.x, self.y + other.y)
    }
}

// impl<T: Add> Add<Size2D<T>> for Point2D<T> {
//     type Output = Point2D<T::Output>;

//     #[inline]
//     fn add(self, other: Size2D<T>) -> Self::Output {
//         Point2D::new(self.x + other.width, self.y + other.height)
//     }
// }

// impl<T: AddAssign> AddAssign<Size2D<T>> for Point2D<T> {
//     #[inline]
//     fn add_assign(&mut self, other: Size2D<T>) {
//         self.x += other.width;
//         self.y += other.height;
//     }
// }

// impl<T: Add> Add<Vector2D<T>> for Point2D<T> {
//     type Output = Point2D<T::Output>;

//     #[inline]
//     fn add(self, other: Vector2D<T>) -> Self::Output {
//         Point2D::new(self.x + other.x, self.y + other.y)
//     }
// }

// impl<T: Copy + Add<T, Output = T>> AddAssign<Vector2D<T>> for Point2D<T> {
//     #[inline]
//     fn add_assign(&mut self, other: Vector2D<T>) {
//         *self = *self + other
//     }
// }

impl<T: Sub> Sub for Point2D<T> {
    type Output = Point2D<T::Output>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Point2D::new(self.x - other.x, self.y - other.y)
    }
}

// impl<T: Sub> Sub for Point2D<T> {
//     type Output = Vector2D<T::Output>;

//     #[inline]
//     fn sub(self, other: Self) -> Self::Output {
//         vec2(self.x - other.x, self.y - other.y)
//     }
// }

// impl<T: Sub> Sub<Size2D<T>> for Point2D<T> {
//     type Output = Point2D<T::Output>;

//     #[inline]
//     fn sub(self, other: Size2D<T>) -> Self::Output {
//         Point2D::new(self.x - other.width, self.y - other.height)
//     }
// }

// impl<T: SubAssign> SubAssign<Size2D<T>> for Point2D<T> {
//     #[inline]
//     fn sub_assign(&mut self, other: Size2D<T>) {
//         self.x -= other.width;
//         self.y -= other.height;
//     }
// }

// impl<T: Sub> Sub<Vector2D<T>> for Point2D<T> {
//     type Output = Point2D<T::Output>;

//     #[inline]
//     fn sub(self, other: Vector2D<T>) -> Self::Output {
//         Point2D::new(self.x - other.x, self.y - other.y)
//     }
// }

// impl<T: Copy + Sub<T, Output = T>> SubAssign<Vector2D<T>> for Point2D<T> {
//     #[inline]
//     fn sub_assign(&mut self, other: Vector2D<T>) {
//         *self = *self - other
//     }
// }

impl<T: Copy + Mul> Mul<T> for Point2D<T> {
    type Output = Point2D<T::Output>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Point2D::new(self.x * scale, self.y * scale)
    }
}

impl<T: Copy + Mul<T, Output = T>> MulAssign<T> for Point2D<T> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        *self = *self * scale
    }
}

// impl<T: Copy + Mul> Mul<Scale<T>> for Point2D<T> {
//     type Output = Point2D<T::Output2>;

//     #[inline]
//     fn mul(self, scale: Scale<T>) -> Self::Output {
//         Point2D::new(self.x * scale.0, self.y * scale.0)
//     }
// }

// impl<T: Copy + MulAssign> MulAssign<Scale<T>> for Point2D<T> {
//     #[inline]
//     fn mul_assign(&mut self, scale: Scale<T>) {
//         self.x *= scale.0;
//         self.y *= scale.0;
//     }
// }

impl<T: Copy + Div> Div<T> for Point2D<T> {
    type Output = Point2D<T::Output>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Point2D::new(self.x / scale, self.y / scale)
    }
}

impl<T: Copy + Div<T, Output = T>> DivAssign<T> for Point2D<T> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        *self = *self / scale
    }
}

// impl<T: Copy + Div> Div<Scale<T>> for Point2D<T2> {
//     type Output = Point2D<T::Output>;

//     #[inline]
//     fn div(self, scale: Scale<T>) -> Self::Output {
//         Point2D::new(self.x / scale.0, self.y / scale.0)
//     }
// }

// impl<T: Copy + DivAssign> DivAssign<Scale<T>> for Point2D<T> {
//     #[inline]
//     fn div_assign(&mut self, scale: Scale<T>) {
//         self.x /= scale.0;
//         self.y /= scale.0;
//     }
// }

// impl<T: Zero> Zero for Point2D<T> {
//     #[inline]
//     fn zero() -> Self {
//         Self::origin()
//     }
// }

// impl<T: Round> Round for Point2D<T> {
//     /// See [Point2D::round()](#method.round)
//     #[inline]
//     fn round(self) -> Self {
//         self.round()
//     }
// }

// impl<T: Ceil> Ceil for Point2D<T> {
//     /// See [Point2D::ceil()](#method.ceil)
//     #[inline]
//     fn ceil(self) -> Self {
//         self.ceil()
//     }
// }

// impl<T: Floor> Floor for Point2D<T> {
//     /// See [Point2D::floor()](#method.floor)
//     #[inline]
//     fn floor(self) -> Self {
//         self.floor()
//     }
// }

// impl<T: ApproxEq<T>> ApproxEq<Point2D<T>> for Point2D<T> {
//     #[inline]
//     fn approx_epsilon() -> Self {
//         Point2D::new(T::approx_epsilon(), T::approx_epsilon())
//     }

//     #[inline]
//     fn approx_eq_eps(&self, other: &Self, eps: &Self) -> bool {
//         self.x.approx_eq_eps(&other.x, &eps.x) && self.y.approx_eq_eps(&other.y, &eps.y)
//     }
// }

impl<T> Into<[T; 2]> for Point2D<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}

// impl<T> From<[T; 2]> for Point2D<T> {
//     fn from([x, y]: [T; 2]) -> Self {
//         Point2D::new(x, y)
//     }
// }

impl<T> Into<(T, T)> for Point2D<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

// impl<T> From<(T, T)> for Point2D<T> {
//     fn from(tuple: (T, T)) -> Self {
//         Point2D::new(tuple.0, tuple.1)
//     }
// }

#[cfg(test)]
mod point2d {
    // use crate::Point2D;

    #[cfg(feature = "mint")]
    use mint;

    // #[test]
    // pub fn test_min() {
    //     let p1 = Point2D::new(1.0, 3.0);
    //     let p2 = Point2D::new(2.0, 2.0);

    //     let result = p1.min(p2);

    //     assert_eq!(result, Point2D::new(1.0, 2.0));
    // }

    // #[test]
    // pub fn test_max() {
    //     let p1 = Point2D::new(1.0, 3.0);
    //     let p2 = Point2D::new(2.0, 2.0);

    //     let result = p1.max(p2);

    //     assert_eq!(result, Point2D::new(2.0, 3.0));
    // }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let p1 = Point2D::new(1.0, 3.0);
        let pm: mint::Point2<_> = p1.into();
        let p2 = Point2D::from(pm);

        assert_eq!(p1, p2);
    }

    // #[test]
    // pub fn test_conv_vector() {
    //     for i in 0..100 {
    //         // We don't care about these values as long as they are not the same.
    //         let x = i as f32 * 0.012345;
    //         let y = i as f32 * 0.987654;
    //         let p: Point2D<f32> = Point2D::new(x, y);
    //         assert_eq!(p.to_vector().to_point(), p);
    //     }
    // }

    // #[test]
    // pub fn test_swizzling() {
    //     let p: Point2D<i32> = Point2D::new(1, 2);
    //     assert_eq!(p.yx(), Point2D::new(2, 1));
    // }

    // #[test]
    // pub fn test_distance_to() {
    //     let p1 = Point2D::new(1.0, 2.0);
    //     let p2 = Point2D::new(2.0, 2.0);

    //     assert_eq!(p1.distance_to(p2), 1.0);

    //     let p1 = Point2D::new(1.0, 2.0);
    //     let p2 = Point2D::new(1.0, 4.0);

    //     assert_eq!(p1.distance_to(p2), 2.0);
    // }

    mod ops {
        use crate::Point2D;
        // use crate::scale::Scale;
        // use crate::{size2, vec2, Vector2D};

        // pub type Point2DMm<T> = crate::Point2D<T>;
        // pub type Point2DCm<T> = crate::Point2D<T, Cm>;

        // #[test]
        // pub fn test_neg() {
        //     assert_eq!(-Point2D::new(1.0, 2.0), Point2D::new(-1.0, -2.0));
        //     assert_eq!(-Point2D::new(0.0, 0.0), Point2D::new(-0.0, -0.0));
        //     assert_eq!(-Point2D::new(-1.0, -2.0), Point2D::new(1.0, 2.0));
        // }

        // #[test]
        // pub fn test_add_size() {
        //     let p1 = Point2DMm::new(1.0, 2.0);
        //     let p2 = size2(3.0, 4.0);

        //     let result = p1 + p2;

        //     assert_eq!(result, Point2DMm::new(4.0, 6.0));
        // }

        // #[test]
        // pub fn test_add_assign_size() {
        //     let mut p1 = Point2DMm::new(1.0, 2.0);

        //     p1 += size2(3.0, 4.0);

        //     assert_eq!(p1, Point2DMm::new(4.0, 6.0));
        // }

        // #[test]
        // pub fn test_add_vec() {
        //     let p1 = Point2DMm::new(1.0, 2.0);
        //     let p2 = vec2(3.0, 4.0);

        //     let result = p1 + p2;

        //     assert_eq!(result, Point2DMm::new(4.0, 6.0));
        // }

        // #[test]
        // pub fn test_add_assign_vec() {
        //     let mut p1 = Point2DMm::new(1.0, 2.0);

        //     p1 += vec2(3.0, 4.0);

        //     assert_eq!(p1, Point2DMm::new(4.0, 6.0));
        // }

        // #[test]
        // pub fn test_sub() {
        //     let p1 = Point2DMm::new(1.0, 2.0);
        //     let p2 = Point2DMm::new(3.0, 4.0);

        //     let result = p1 - p2;

        //     assert_eq!(result, Vector2D::<_>::new(-2.0, -2.0));
        // }

        // #[test]
        // pub fn test_sub_size() {
        //     let p1 = Point2DMm::new(1.0, 2.0);
        //     let p2 = size2(3.0, 4.0);

        //     let result = p1 - p2;

        //     assert_eq!(result, Point2DMm::new(-2.0, -2.0));
        // }

        // #[test]
        // pub fn test_sub_assign_size() {
        //     let mut p1 = Point2DMm::new(1.0, 2.0);

        //     p1 -= size2(3.0, 4.0);

        //     assert_eq!(p1, Point2DMm::new(-2.0, -2.0));
        // }

        // #[test]
        // pub fn test_sub_vec() {
        //     let p1 = Point2DMm::new(1.0, 2.0);
        //     let p2 = vec2(3.0, 4.0);

        //     let result = p1 - p2;

        //     assert_eq!(result, Point2DMm::new(-2.0, -2.0));
        // }

        // #[test]
        // pub fn test_sub_assign_vec() {
        //     let mut p1 = Point2DMm::new(1.0, 2.0);

        //     p1 -= vec2(3.0, 4.0);

        //     assert_eq!(p1, Point2DMm::new(-2.0, -2.0));
        // }

        // #[test]
        // pub fn test_mul_scalar() {
        //     let p1: Point2D<f32> = Point2D::new(3.0, 5.0);

        //     let result = p1 * 5.0;

        //     assert_eq!(result, Point2D::new(15.0, 25.0));
        // }

        // #[test]
        // pub fn test_mul_assign_scalar() {
        //     let mut p1 = Point2D::new(3.0, 5.0);

        //     p1 *= 5.0;

        //     assert_eq!(p1, Point2D::new(15.0, 25.0));
        // }

        // #[test]
        // pub fn test_mul_scale() {
        //     let p1 = Point2DMm::new(1.0, 2.0);
        //     let cm_per_mm: Scale<f32, Cm> = Scale::new(0.1);

        //     let result = p1 * cm_per_mm;

        //     assert_eq!(result, Point2DCm::new(0.1, 0.2));
        // }

        // #[test]
        // pub fn test_mul_assign_scale() {
        //     let mut p1 = Point2DMm::new(1.0, 2.0);
        //     let scale: Scale<f32> = Scale::new(0.1);

        //     p1 *= scale;

        //     assert_eq!(p1, Point2DMm::new(0.1, 0.2));
        // }

        // #[test]
        // pub fn test_div_scalar() {
        //     let p1: Point2D<f32> = Point2D::new(15.0, 25.0);

        //     let result = p1 / 5.0;

        //     assert_eq!(result, Point2D::new(3.0, 5.0));
        // }

        // #[test]
        // pub fn test_div_assign_scalar() {
        //     let mut p1: Point2D<f32> = Point2D::new(15.0, 25.0);

        //     p1 /= 5.0;

        //     assert_eq!(p1, Point2D::new(3.0, 5.0));
        // }

        // #[test]
        // pub fn test_div_scale() {
        //     let p1 = Point2DCm::new(0.1, 0.2);
        //     let cm_per_mm: Scale<f32, Cm> = Scale::new(0.1);

        //     let result = p1 / cm_per_mm;

        //     assert_eq!(result, Point2DMm::new(1.0, 2.0));
        // }

        // #[test]
        // pub fn test_div_assign_scale() {
        //     let mut p1 = Point2DMm::new(0.1, 0.2);
        //     let scale: Scale<f32> = Scale::new(0.1);

        //     p1 /= scale;

        //     assert_eq!(p1, Point2DMm::new(1.0, 2.0));
        // }

        #[test]
        pub fn test_point_debug_formatting() {
            let n = 1.23456789;
            let p1 = Point2D::new(n, -n);
            let should_be = format!("({:.4}, {:.4})", n, -n);

            let got = format!("{:.4?}", p1);

            assert_eq!(got, should_be);
        }
    }
}