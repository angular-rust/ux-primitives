#![allow(unused_imports)]
use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 3d Point tagged with a unit.
#[repr(C)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// mint_vec!(Point3D[x, y, z] = Point3);

impl<T: Copy> Copy for Point3D<T> {}

impl<T: Clone> Clone for Point3D<T> {
    fn clone(&self) -> Self {
        Point3D {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Point3D<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y, z) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Point3D {
            x,
            y,
            z,
        })
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Point3D<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.x, &self.y, &self.z).serialize(serializer)
    }
}

impl<T> Eq for Point3D<T> where T: Eq {}

impl<T> PartialEq for Point3D<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T> Hash for Point3D<T>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
        self.z.hash(h);
    }
}

impl<T: fmt::Debug> fmt::Debug for Point3D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl<T: Default> Default for Point3D<T> {
    fn default() -> Self {
        Point3D::new(Default::default(), Default::default(), Default::default())
    }
}

impl<T> Point3D<T> {
    // /// Constructor, setting all components to zero.
    // #[inline]
    // pub fn origin() -> Self
    // where
    //     T: Zero,
    // {
    //     point3(Zero::zero(), Zero::zero(), Zero::zero())
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
    pub const fn new(x: T, y: T, z: T) -> Self {
        Point3D {
            x,
            y,
            z,
        }
    }

    // /// Constructor taking properly Lengths instead of scalar values.
    // #[inline]
    // pub fn from_lengths(x: Length<T>, y: Length<T>, z: Length<T>) -> Self {
    //     point3(x.0, y.0, z.0)
    // }

    /// Constructor setting all components to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Point3D {
            x: v.clone(),
            y: v.clone(),
            z: v,
        }
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(p: Point3D<T>) -> Self {
        point3(p.x, p.y, p.z)
    }
}

impl<T: Copy> Point3D<T> {
    // /// Cast this point into a vector.
    // ///
    // /// Equivalent to subtracting the origin to this point.
    // #[inline]
    // pub fn to_vector(self) -> Vector3D<T> {
    //     Vector3D {
    //         x: self.x,
    //         y: self.y,
    //         z: self.z,
    //     }
    // }

    // /// Returns a 2d point using this point's x and y coordinates
    // #[inline]
    // pub fn xy(self) -> Point2D<T> {
    //     point2(self.x, self.y)
    // }

    // /// Returns a 2d point using this point's x and z coordinates
    // #[inline]
    // pub fn xz(self) -> Point2D<T> {
    //     point2(self.x, self.z)
    // }

    // /// Returns a 2d point using this point's x and z coordinates
    // #[inline]
    // pub fn yz(self) -> Point2D<T> {
    //     point2(self.y, self.z)
    // }

    /// Cast into an array with x, y and z.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use euclid::{Point3D, point3};
    /// enum Mm {}
    ///
    /// let point: Point3D<_, Mm> = point3(1, -8, 0);
    ///
    /// assert_eq!(point.to_array(), [1, -8, 0]);
    /// ```
    #[inline]
    pub fn to_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    // #[inline]
    // pub fn to_array_4d(self) -> [T; 4]
    // where
    //     T: One,
    // {
    //     [self.x, self.y, self.z, One::one()]
    // }

    /// Cast into a tuple with x, y and z.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use euclid::{Point3D, point3};
    /// enum Mm {}
    ///
    /// let point: Point3D<_, Mm> = point3(1, -8, 0);
    ///
    /// assert_eq!(point.to_tuple(), (1, -8, 0));
    /// ```
    #[inline]
    pub fn to_tuple(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    // #[inline]
    // pub fn to_tuple_4d(self) -> (T, T, T, T)
    // where
    //     T: One,
    // {
    //     (self.x, self.y, self.z, One::one())
    // }

    /// Drop the units, preserving only the numeric value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use euclid::{Point3D, point3};
    /// enum Mm {}
    ///
    /// let point: Point3D<_, Mm> = point3(1, -8, 0);
    ///
    /// assert_eq!(point.x, point.to_untyped().x);
    /// assert_eq!(point.y, point.to_untyped().y);
    /// assert_eq!(point.z, point.to_untyped().z);
    /// ```
    #[inline]
    pub fn to_untyped(self) -> Point3D<T> {
        point3(self.x, self.y, self.z)
    }

    // /// Cast the unit, preserving the numeric value.
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// # use euclid::{Point3D, point3};
    // /// enum Mm {}
    // /// enum Cm {}
    // ///
    // /// let point: Point3D<_, Mm> = point3(1, -8, 0);
    // ///
    // /// assert_eq!(point.x, point.cast_unit::<Cm>().x);
    // /// assert_eq!(point.y, point.cast_unit::<Cm>().y);
    // /// assert_eq!(point.z, point.cast_unit::<Cm>().z);
    // /// ```
    // #[inline]
    // pub fn cast_unit<V>(self) -> Point3D<T, V> {
    //     point3(self.x, self.y, self.z)
    // }

    // /// Convert into a 2d point.
    // #[inline]
    // pub fn to_2d(self) -> Point2D<T> {
    //     self.xy()
    // }

    // /// Rounds each component to the nearest integer value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::point3;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(point3::<_, Mm>(-0.1, -0.8, 0.4).round(), point3::<_, Mm>(0.0, -1.0, 0.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn round(self) -> Self
    // where
    //     T: Round,
    // {
    //     point3(self.x.round(), self.y.round(), self.z.round())
    // }

    // /// Rounds each component to the smallest integer equal or greater than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::point3;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(point3::<_, Mm>(-0.1, -0.8, 0.4).ceil(), point3::<_, Mm>(0.0, 0.0, 1.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn ceil(self) -> Self
    // where
    //     T: Ceil,
    // {
    //     point3(self.x.ceil(), self.y.ceil(), self.z.ceil())
    // }

    // /// Rounds each component to the biggest integer equal or lower than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::point3;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(point3::<_, Mm>(-0.1, -0.8, 0.4).floor(), point3::<_, Mm>(-1.0, -1.0, 0.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn floor(self) -> Self
    // where
    //     T: Floor,
    // {
    //     point3(self.x.floor(), self.y.floor(), self.z.floor())
    // }

    // /// Linearly interpolate between this point and another point.
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// use euclid::point3;
    // /// use euclid::default::Point3D;
    // ///
    // /// let from: Point3D<_> = point3(0.0, 10.0, -1.0);
    // /// let to:  Point3D<_> = point3(8.0, -4.0,  0.0);
    // ///
    // /// assert_eq!(from.lerp(to, -1.0), point3(-8.0,  24.0, -2.0));
    // /// assert_eq!(from.lerp(to,  0.0), point3( 0.0,  10.0, -1.0));
    // /// assert_eq!(from.lerp(to,  0.5), point3( 4.0,   3.0, -0.5));
    // /// assert_eq!(from.lerp(to,  1.0), point3( 8.0,  -4.0,  0.0));
    // /// assert_eq!(from.lerp(to,  2.0), point3(16.0, -18.0,  1.0));
    // /// ```
    // #[inline]
    // pub fn lerp(self, other: Self, t: T) -> Self
    // where
    //     T: One + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
    // {
    //     let one_t = T::one() - t;
    //     point3(
    //         one_t * self.x + t * other.x,
    //         one_t * self.y + t * other.y,
    //         one_t * self.z + t * other.z,
    //     )
    // }
}

impl<T: PartialOrd> Point3D<T> {
    // #[inline]
    // pub fn min(self, other: Self) -> Self {
    //     point3(
    //         min(self.x, other.x),
    //         min(self.y, other.y),
    //         min(self.z, other.z),
    //     )
    // }

    // #[inline]
    // pub fn max(self, other: Self) -> Self {
    //     point3(
    //         max(self.x, other.x),
    //         max(self.y, other.y),
    //         max(self.z, other.z),
    //     )
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

// impl<T: NumCast + Copy> Point3D<T> {
//     /// Cast from one numeric representation to another, preserving the units.
//     ///
//     /// When casting from floating point to integer coordinates, the decimals are truncated
//     /// as one would expect from a simple cast, but this behavior does not always make sense
//     /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
//     #[inline]
//     pub fn cast<NewT: NumCast>(self) -> Point3D<NewT> {
//         self.try_cast().unwrap()
//     }

//     /// Fallible cast from one numeric representation to another, preserving the units.
//     ///
//     /// When casting from floating point to integer coordinates, the decimals are truncated
//     /// as one would expect from a simple cast, but this behavior does not always make sense
//     /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
//     pub fn try_cast<NewT: NumCast>(self) -> Option<Point3D<NewT>> {
//         match (
//             NumCast::from(self.x),
//             NumCast::from(self.y),
//             NumCast::from(self.z),
//         ) {
//             (Some(x), Some(y), Some(z)) => Some(point3(x, y, z)),
//             _ => None,
//         }
//     }

//     // Convenience functions for common casts

//     /// Cast into an `f32` point.
//     #[inline]
//     pub fn to_f32(self) -> Point3D<f32> {
//         self.cast()
//     }

//     /// Cast into an `f64` point.
//     #[inline]
//     pub fn to_f64(self) -> Point3D<f64> {
//         self.cast()
//     }

//     /// Cast into an `usize` point, truncating decimals if any.
//     ///
//     /// When casting from floating point points, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_usize(self) -> Point3D<usize> {
//         self.cast()
//     }

//     /// Cast into an `u32` point, truncating decimals if any.
//     ///
//     /// When casting from floating point points, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_u32(self) -> Point3D<u32> {
//         self.cast()
//     }

//     /// Cast into an `i32` point, truncating decimals if any.
//     ///
//     /// When casting from floating point points, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_i32(self) -> Point3D<i32> {
//         self.cast()
//     }

//     /// Cast into an `i64` point, truncating decimals if any.
//     ///
//     /// When casting from floating point points, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_i64(self) -> Point3D<i64> {
//         self.cast()
//     }
// }

// impl<T: Float> Point3D<T> {
//     /// Returns true if all members are finite.
//     #[inline]
//     pub fn is_finite(self) -> bool {
//         self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
//     }
// }

// impl<T: Copy + Add<T, Output = T>> Point3D<T> {
//     #[inline]
//     pub fn add_size(self, other: Size3D<T>) -> Self {
//         point3(
//             self.x + other.width,
//             self.y + other.height,
//             self.z + other.depth,
//         )
//     }
// }

// impl<T: Float + Sub<T, Output = T>> Point3D<T> {
//     #[inline]
//     pub fn distance_to(self, other: Self) -> T {
//         (self - other).length()
//     }
// }

impl<T: Neg> Neg for Point3D<T> {
    type Output = Point3D<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        point3(-self.x, -self.y, -self.z)
    }
}

// impl<T: Add> Add<Size3D<T>> for Point3D<T> {
//     type Output = Point3D<T::Output>;

//     #[inline]
//     fn add(self, other: Size3D<T>) -> Self::Output {
//         point3(
//             self.x + other.width,
//             self.y + other.height,
//             self.z + other.depth,
//         )
//     }
// }

// impl<T: AddAssign> AddAssign<Size3D<T>> for Point3D<T> {
//     #[inline]
//     fn add_assign(&mut self, other: Size3D<T>) {
//         self.x += other.width;
//         self.y += other.height;
//         self.z += other.depth;
//     }
// }

// impl<T: Add> Add<Vector3D<T>> for Point3D<T> {
//     type Output = Point3D<T::Output>;

//     #[inline]
//     fn add(self, other: Vector3D<T>) -> Self::Output {
//         point3(self.x + other.x, self.y + other.y, self.z + other.z)
//     }
// }

// impl<T: Copy + Add<T, Output = T>> AddAssign<Vector3D<T>> for Point3D<T> {
//     #[inline]
//     fn add_assign(&mut self, other: Vector3D<T>) {
//         *self = *self + other
//     }
// }

// impl<T: Sub> Sub for Point3D<T> {
//     type Output = Vector3D<T::Output>;

//     #[inline]
//     fn sub(self, other: Self) -> Self::Output {
//         vec3(self.x - other.x, self.y - other.y, self.z - other.z)
//     }
// }

// impl<T: Sub> Sub<Size3D<T>> for Point3D<T> {
//     type Output = Point3D<T::Output>;

//     #[inline]
//     fn sub(self, other: Size3D<T>) -> Self::Output {
//         point3(
//             self.x - other.width,
//             self.y - other.height,
//             self.z - other.depth,
//         )
//     }
// }

// impl<T: SubAssign> SubAssign<Size3D<T>> for Point3D<T> {
//     #[inline]
//     fn sub_assign(&mut self, other: Size3D<T>) {
//         self.x -= other.width;
//         self.y -= other.height;
//         self.z -= other.depth;
//     }
// }

// impl<T: Sub> Sub<Vector3D<T>> for Point3D<T> {
//     type Output = Point3D<T::Output>;

//     #[inline]
//     fn sub(self, other: Vector3D<T>) -> Self::Output {
//         point3(self.x - other.x, self.y - other.y, self.z - other.z)
//     }
// }

// impl<T: Copy + Sub<T, Output = T>> SubAssign<Vector3D<T>> for Point3D<T> {
//     #[inline]
//     fn sub_assign(&mut self, other: Vector3D<T>) {
//         *self = *self - other
//     }
// }

impl<T: Copy + Mul> Mul<T> for Point3D<T> {
    type Output = Point3D<T::Output>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        point3(
            self.x * scale,
            self.y * scale,
            self.z * scale,
        )
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Point3D<T> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
}

// impl<T: Copy + Mul> Mul<Scale<T>> for Point3D<T> {
//     type Output = Point3D<T::Output2>;

//     #[inline]
//     fn mul(self, scale: Scale<T>) -> Self::Output {
//         point3(
//             self.x * scale.0,
//             self.y * scale.0,
//             self.z * scale.0,
//         )
//     }
// }

// impl<T: Copy + MulAssign> MulAssign<Scale<T>> for Point3D<T> {
//     #[inline]
//     fn mul_assign(&mut self, scale: Scale<T>) {
//         *self *= scale.0;
//     }
// }

impl<T: Copy + Div> Div<T> for Point3D<T> {
    type Output = Point3D<T::Output>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        point3(
            self.x / scale,
            self.y / scale,
            self.z / scale,
        )
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Point3D<T> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        self.x /= scale;
        self.y /= scale;
        self.z /= scale;
    }
}

// impl<T: Copy + Div> Div<Scale<T>> for Point3D<T2> {
//     type Output = Point3D<T::Output>;

//     #[inline]
//     fn div(self, scale: Scale<T>) -> Self::Output {
//         point3(
//             self.x / scale.0,
//             self.y / scale.0,
//             self.z / scale.0,
//         )
//     }
// }

// impl<T: Copy + DivAssign> DivAssign<Scale<T>> for Point3D<T> {
//     #[inline]
//     fn div_assign(&mut self, scale: Scale<T>) {
//         *self /= scale.0;
//     }
// }

// impl<T: Zero> Zero for Point3D<T> {
//     #[inline]
//     fn zero() -> Self {
//         Self::origin()
//     }
// }

// impl<T: Round> Round for Point3D<T> {
//     /// See [Point3D::round()](#method.round)
//     #[inline]
//     fn round(self) -> Self {
//         self.round()
//     }
// }

// impl<T: Ceil> Ceil for Point3D<T> {
//     /// See [Point3D::ceil()](#method.ceil)
//     #[inline]
//     fn ceil(self) -> Self {
//         self.ceil()
//     }
// }

// impl<T: Floor> Floor for Point3D<T> {
//     /// See [Point3D::floor()](#method.floor)
//     #[inline]
//     fn floor(self) -> Self {
//         self.floor()
//     }
// }

// impl<T: ApproxEq<T>> ApproxEq<Point3D<T>> for Point3D<T> {
//     #[inline]
//     fn approx_epsilon() -> Self {
//         point3(
//             T::approx_epsilon(),
//             T::approx_epsilon(),
//             T::approx_epsilon(),
//         )
//     }

//     #[inline]
//     fn approx_eq_eps(&self, other: &Self, eps: &Self) -> bool {
//         self.x.approx_eq_eps(&other.x, &eps.x)
//             && self.y.approx_eq_eps(&other.y, &eps.y)
//             && self.z.approx_eq_eps(&other.z, &eps.z)
//     }
// }

impl<T> Into<[T; 3]> for Point3D<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T> From<[T; 3]> for Point3D<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        point3(x, y, z)
    }
}

impl<T> Into<(T, T, T)> for Point3D<T> {
    fn into(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T> From<(T, T, T)> for Point3D<T> {
    fn from(tuple: (T, T, T)) -> Self {
        point3(tuple.0, tuple.1, tuple.2)
    }
}

/// Shorthand for `Point3D::new(x, y)`.
#[inline]
pub const fn point3<T>(x: T, y: T, z: T) -> Point3D<T> {
    Point3D {
        x,
        y,
        z,
    }
}

#[cfg(test)]
mod point3d {
    // use crate::default;
    // use crate::Point3D;
    // use crate::{point2, point3};
    #[cfg(feature = "mint")]
    use mint;

    // #[test]
    // pub fn test_min() {
    //     let p1 = Point3D::new(1.0, 3.0, 5.0);
    //     let p2 = Point3D::new(2.0, 2.0, -1.0);

    //     let result = p1.min(p2);

    //     assert_eq!(result, Point3D::new(1.0, 2.0, -1.0));
    // }

    // #[test]
    // pub fn test_max() {
    //     let p1 = Point3D::new(1.0, 3.0, 5.0);
    //     let p2 = Point3D::new(2.0, 2.0, -1.0);

    //     let result = p1.max(p2);

    //     assert_eq!(result, Point3D::new(2.0, 3.0, 5.0));
    // }

    // #[test]
    // pub fn test_conv_vector() {
    //     use crate::point3;
    //     for i in 0..100 {
    //         // We don't care about these values as long as they are not the same.
    //         let x = i as f32 * 0.012345;
    //         let y = i as f32 * 0.987654;
    //         let z = x * y;
    //         let p: Point3D<f32> = point3(x, y, z);
    //         assert_eq!(p.to_vector().to_point(), p);
    //     }
    // }

    // #[test]
    // pub fn test_swizzling() {
    //     let p: default::Point3D<i32> = point3(1, 2, 3);
    //     assert_eq!(p.xy(), point2(1, 2));
    //     assert_eq!(p.xz(), point2(1, 3));
    //     assert_eq!(p.yz(), point2(2, 3));
    // }

    // #[test]
    // pub fn test_distance_to() {
    //     let p1 = Point3D::new(1.0, 2.0, 3.0);
    //     let p2 = Point3D::new(2.0, 2.0, 3.0);

    //     assert_eq!(p1.distance_to(p2), 1.0);

    //     let p1 = Point3D::new(1.0, 2.0, 3.0);
    //     let p2 = Point3D::new(1.0, 4.0, 3.0);

    //     assert_eq!(p1.distance_to(p2), 2.0);

    //     let p1 = Point3D::new(1.0, 2.0, 3.0);
    //     let p2 = Point3D::new(1.0, 2.0, 6.0);

    //     assert_eq!(p1.distance_to(p2), 3.0);
    // }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let p1 = Point3D::new(1.0, 3.0, 5.0);
        let pm: mint::Point3<_> = p1.into();
        let p2 = Point3D::from(pm);

        assert_eq!(p1, p2);
    }

    mod ops {
        use crate::Point3D;
        // use crate::scale::Scale;
        // use crate::{size3, vec3, Vector3D};

        // pub enum Mm {}
        // pub enum Cm {}

        // pub type Point3DMm<T> = crate::Point3D<T, Mm>;
        // pub type Point3DCm<T> = crate::Point3D<T, Cm>;

        #[test]
        pub fn test_neg() {
            assert_eq!(-Point3D::new(1.0, 2.0, 3.0), Point3D::new(-1.0, -2.0, -3.0));
            assert_eq!(-Point3D::new(0.0, 0.0, 0.0), Point3D::new(-0.0, -0.0, -0.0));
            assert_eq!(-Point3D::new(-1.0, -2.0, -3.0), Point3D::new(1.0, 2.0, 3.0));
        }

        // #[test]
        // pub fn test_add_size() {
        //     let p1 = Point3DMm::new(1.0, 2.0, 3.0);
        //     let p2 = size3(4.0, 5.0, 6.0);

        //     let result = p1 + p2;

        //     assert_eq!(result, Point3DMm::new(5.0, 7.0, 9.0));
        // }

        // #[test]
        // pub fn test_add_assign_size() {
        //     let mut p1 = Point3DMm::new(1.0, 2.0, 3.0);

        //     p1 += size3(4.0, 5.0, 6.0);

        //     assert_eq!(p1, Point3DMm::new(5.0, 7.0, 9.0));
        // }

        // #[test]
        // pub fn test_add_vec() {
        //     let p1 = Point3DMm::new(1.0, 2.0, 3.0);
        //     let p2 = vec3(4.0, 5.0, 6.0);

        //     let result = p1 + p2;

        //     assert_eq!(result, Point3DMm::new(5.0, 7.0, 9.0));
        // }

        // #[test]
        // pub fn test_add_assign_vec() {
        //     let mut p1 = Point3DMm::new(1.0, 2.0, 3.0);

        //     p1 += vec3(4.0, 5.0, 6.0);

        //     assert_eq!(p1, Point3DMm::new(5.0, 7.0, 9.0));
        // }

        // #[test]
        // pub fn test_sub() {
        //     let p1 = Point3DMm::new(1.0, 2.0, 3.0);
        //     let p2 = Point3DMm::new(4.0, 5.0, 6.0);

        //     let result = p1 - p2;

        //     assert_eq!(result, Vector3D::<_, Mm>::new(-3.0, -3.0, -3.0));
        // }

        // #[test]
        // pub fn test_sub_size() {
        //     let p1 = Point3DMm::new(1.0, 2.0, 3.0);
        //     let p2 = size3(4.0, 5.0, 6.0);

        //     let result = p1 - p2;

        //     assert_eq!(result, Point3DMm::new(-3.0, -3.0, -3.0));
        // }

        // #[test]
        // pub fn test_sub_assign_size() {
        //     let mut p1 = Point3DMm::new(1.0, 2.0, 3.0);

        //     p1 -= size3(4.0, 5.0, 6.0);

        //     assert_eq!(p1, Point3DMm::new(-3.0, -3.0, -3.0));
        // }

        // #[test]
        // pub fn test_sub_vec() {
        //     let p1 = Point3DMm::new(1.0, 2.0, 3.0);
        //     let p2 = vec3(4.0, 5.0, 6.0);

        //     let result = p1 - p2;

        //     assert_eq!(result, Point3DMm::new(-3.0, -3.0, -3.0));
        // }

        // #[test]
        // pub fn test_sub_assign_vec() {
        //     let mut p1 = Point3DMm::new(1.0, 2.0, 3.0);

        //     p1 -= vec3(4.0, 5.0, 6.0);

        //     assert_eq!(p1, Point3DMm::new(-3.0, -3.0, -3.0));
        // }

        #[test]
        pub fn test_mul_scalar() {
            let p1: Point3D<f32> = Point3D::new(3.0, 5.0, 7.0);

            let result = p1 * 5.0;

            assert_eq!(result, Point3D::new(15.0, 25.0, 35.0));
        }

        #[test]
        pub fn test_mul_assign_scalar() {
            let mut p1: Point3D<f32> = Point3D::new(3.0, 5.0, 7.0);

            p1 *= 5.0;

            assert_eq!(p1, Point3D::new(15.0, 25.0, 35.0));
        }

        // #[test]
        // pub fn test_mul_scale() {
        //     let p1 = Point3DMm::new(1.0, 2.0, 3.0);
        //     let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

        //     let result = p1 * cm_per_mm;

        //     assert_eq!(result, Point3DCm::new(0.1, 0.2, 0.3));
        // }

        // #[test]
        // pub fn test_mul_assign_scale() {
        //     let mut p1 = Point3DMm::new(1.0, 2.0, 3.0);
        //     let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

        //     p1 *= scale;

        //     assert_eq!(p1, Point3DMm::new(0.1, 0.2, 0.3));
        // }

        #[test]
        pub fn test_div_scalar() {
            let p1: Point3D<f32> = Point3D::new(15.0, 25.0, 35.0);

            let result = p1 / 5.0;

            assert_eq!(result, Point3D::new(3.0, 5.0, 7.0));
        }

        #[test]
        pub fn test_div_assign_scalar() {
            let mut p1: Point3D<f32> = Point3D::new(15.0, 25.0, 35.0);

            p1 /= 5.0;

            assert_eq!(p1, Point3D::new(3.0, 5.0, 7.0));
        }

        // #[test]
        // pub fn test_div_scale() {
        //     let p1 = Point3DCm::new(0.1, 0.2, 0.3);
        //     let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

        //     let result = p1 / cm_per_mm;

        //     assert_eq!(result, Point3DMm::new(1.0, 2.0, 3.0));
        // }

        // #[test]
        // pub fn test_div_assign_scale() {
        //     let mut p1 = Point3DMm::new(0.1, 0.2, 0.3);
        //     let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

        //     p1 /= scale;

        //     assert_eq!(p1, Point3DMm::new(1.0, 2.0, 3.0));
        // }
    }
}