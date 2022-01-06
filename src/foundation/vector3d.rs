use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
// use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{Float, NumCast, Signed};

/// A 3d Vector tagged with a unit.
#[repr(C)]
pub struct Vector3D<T> {
    /// The `x` (traditionally, horizontal) coordinate.
    pub x: T,
    /// The `y` (traditionally, vertical) coordinate.
    pub y: T,
    /// The `z` (traditionally, depth) coordinate.
    pub z: T,
}

// mint_vec!(Vector3D[x, y, z] = Vector3);

impl<T: Copy> Copy for Vector3D<T> {}

impl<T: Clone> Clone for Vector3D<T> {
    fn clone(&self) -> Self {
        Vector3D {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Vector3D<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y, z) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Vector3D {
            x,
            y,
            z,
        })
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Vector3D<T>
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

impl<T: Eq> Eq for Vector3D<T> {}

impl<T: PartialEq> PartialEq for Vector3D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Hash> Hash for Vector3D<T> {
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
        self.z.hash(h);
    }
}

// impl<T: Zero> Zero for Vector3D<T> {
//     /// Constructor, setting all components to zero.
//     #[inline]
//     fn zero() -> Self {
//         Vector3D::new(Zero::zero(), Zero::zero(), Zero::zero())
//     }
// }

impl<T: fmt::Debug> fmt::Debug for Vector3D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl<T: Default> Default for Vector3D<T> {
    fn default() -> Self {
        Vector3D::new(Default::default(), Default::default(), Default::default())
    }
}

impl<T> Vector3D<T> {
    /// Constructor, setting all components to zero.
    // #[inline]
    // pub fn zero() -> Self
    // where
    //     T: Zero,
    // {
    //     Vector3D::new(Zero::zero(), Zero::zero(), Zero::zero())
    // }

    // /// Constructor, setting all components to one.
    // #[inline]
    // pub fn one() -> Self
    // where
    //     T: One,
    // {
    //     Vector3D::new(One::one(), One::one(), One::one())
    // }

    /// Constructor taking scalar values directly.
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Vector3D {
            x,
            y,
            z,
        }
    }
    /// Constructor setting all components to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Vector3D {
            x: v.clone(),
            y: v.clone(),
            z: v,
        }
    }

    // /// Constructor taking properly  Lengths instead of scalar values.
    // #[inline]
    // pub fn from_lengths(x: Length<T>, y: Length<T>, z: Length<T>) -> Vector3D<T> {
    //     Vector3D::new(x.0, y.0, z.0)
    // }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(p: Vector3D<T>) -> Self {
        Vector3D::new(p.x, p.y, p.z)
    }

    /// Computes the vector with absolute values of each component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::{i32, f32};
    /// # use primitives::foundation::Vector3D;
    ///
    /// assert_eq!(Vector3D::new(-1, 0, 2).abs(), Vector3D::new(1, 0, 2));
    ///
    /// let vec = Vector3D::new(f32::NAN, 0.0, -f32::MAX).abs();
    /// assert!(vec.x.is_nan());
    /// assert_eq!(vec.y, 0.0);
    /// assert_eq!(vec.z, f32::MAX);
    /// ```
    ///
    /// # Panics
    ///
    /// The behavior for each component follows the scalar type's implementation of
    /// `num_traits::Signed::abs`.
    pub fn abs(self) -> Self
    where
        T: Signed,
    {
        Vector3D::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// Dot product.
    #[inline]
    pub fn dot(self, other: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Copy> Vector3D<T> {
    /// Cross product.
    #[inline]
    pub fn cross(self, other: Self) -> Self
    where
        T: Sub<Output = T> + Mul<Output = T>,
    {
        Vector3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Returns the component-wise multiplication of the two vectors.
    #[inline]
    pub fn component_mul(self, other: Self) -> Self
    where
        T: Mul<Output = T>,
    {
        Vector3D::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    /// Returns the component-wise division of the two vectors.
    #[inline]
    pub fn component_div(self, other: Self) -> Self
    where
        T: Div<Output = T>,
    {
        Vector3D::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }

    // /// Cast this vector into a point.
    // ///
    // /// Equivalent to adding this vector to the origin.
    // #[inline]
    // pub fn to_point(self) -> Point3D<T> {
    //     point3(self.x, self.y, self.z)
    // }

    // /// Returns a 2d vector using this vector's x and y coordinates
    // #[inline]
    // pub fn xy(self) -> Vector2D<T> {
    //     vec2(self.x, self.y)
    // }

    // /// Returns a 2d vector using this vector's x and z coordinates
    // #[inline]
    // pub fn xz(self) -> Vector2D<T> {
    //     vec2(self.x, self.z)
    // }

    // /// Returns a 2d vector using this vector's x and z coordinates
    // #[inline]
    // pub fn yz(self) -> Vector2D<T> {
    //     vec2(self.y, self.z)
    // }

    /// Cast into an array with x, y and z.
    #[inline]
    pub fn to_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    // /// Cast into an array with x, y, z and 0.
    // #[inline]
    // pub fn to_array_4d(self) -> [T; 4]
    // where
    //     T: Zero,
    // {
    //     [self.x, self.y, self.z, Zero::zero()]
    // }

    /// Cast into a tuple with x, y and z.
    #[inline]
    pub fn to_tuple(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    // /// Cast into a tuple with x, y, z and 0.
    // #[inline]
    // pub fn to_tuple_4d(self) -> (T, T, T, T)
    // where
    //     T: Zero,
    // {
    //     (self.x, self.y, self.z, Zero::zero())
    // }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(self) -> Vector3D<T> {
        Vector3D::new(self.x, self.y, self.z)
    }

    // /// Cast the unit.
    // #[inline]
    // pub fn cast_unit<V>(self) -> Vector3D<T, V> {
    //     Vector3D::new(self.x, self.y, self.z)
    // }

    // /// Convert into a 2d vector.
    // #[inline]
    // pub fn to_2d(self) -> Vector2D<T> {
    //     self.xy()
    // }

    // /// Rounds each component to the nearest integer value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::vec3;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(vec3::<_, Mm>(-0.1, -0.8, 0.4).round(), vec3::<_, Mm>(0.0, -1.0, 0.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn round(self) -> Self
    // where
    //     T: Round,
    // {
    //     Vector3D::new(self.x.round(), self.y.round(), self.z.round())
    // }

    // /// Rounds each component to the smallest integer equal or greater than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::vec3;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(vec3::<_, Mm>(-0.1, -0.8, 0.4).ceil(), vec3::<_, Mm>(0.0, 0.0, 1.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn ceil(self) -> Self
    // where
    //     T: Ceil,
    // {
    //     Vector3D::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    // }

    // /// Rounds each component to the biggest integer equal or lower than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::vec3;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(vec3::<_, Mm>(-0.1, -0.8, 0.4).floor(), vec3::<_, Mm>(-1.0, -1.0, 0.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn floor(self) -> Self
    // where
    //     T: Floor,
    // {
    //     Vector3D::new(self.x.floor(), self.y.floor(), self.z.floor())
    // }

    // /// Creates translation by this vector in vector units
    // #[inline]
    // pub fn to_transform(self) -> Transform3D<T>
    // where
    //     T: Zero + One,
    // {
    //     Transform3D::translation(self.x, self.y, self.z)
    // }
}

impl<T> Vector3D<T>
where
    T: Copy + Mul<T, Output = T> + Add<T, Output = T>,
{
    /// Returns the vector's length squared.
    #[inline]
    pub fn square_length(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns this vector projected onto another one.
    ///
    /// Projecting onto a nil vector will cause a division by zero.
    #[inline]
    pub fn project_onto_vector(self, onto: Self) -> Self
    where
        T: Sub<T, Output = T> + Div<T, Output = T>,
    {
        onto * (self.dot(onto) / onto.square_length())
    }
}

impl<T: Float> Vector3D<T> {
    // /// Returns the positive angle between this vector and another vector.
    // ///
    // /// The returned angle is between 0 and PI.
    // pub fn angle_to(self, other: Self) -> Angle<T>
    // where
    //     T: Trig,
    // {
    //     Angle::radians(Trig::fast_atan2(
    //         self.cross(other).length(),
    //         self.dot(other),
    //     ))
    // }

    /// Returns the vector length.
    #[inline]
    pub fn length(self) -> T {
        self.square_length().sqrt()
    }

    /// Returns the vector with length of one unit
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    /// Returns the vector with length of one unit.
    ///
    /// Unlike [`Vector2D::normalize`](#method.normalize), this returns None in the case that the
    /// length of the vector is zero.
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let len = self.length();
        if len == T::zero() {
            None
        } else {
            Some(self / len)
        }
    }

    /// Return the normalized vector even if the length is larger than the max value of Float.
    #[inline]
    #[must_use]
    pub fn robust_normalize(self) -> Self {
        let length = self.length();
        if length.is_infinite() {
            let scaled = self / T::max_value();
            scaled / scaled.length()
        } else {
            self / length
        }
    }

    /// Return this vector capped to a maximum length.
    #[inline]
    pub fn with_max_length(self, max_length: T) -> Self {
        let square_length = self.square_length();
        if square_length > max_length * max_length {
            return self * (max_length / square_length.sqrt());
        }

        self
    }

    /// Return this vector with a minimum length applied.
    #[inline]
    pub fn with_min_length(self, min_length: T) -> Self {
        let square_length = self.square_length();
        if square_length < min_length * min_length {
            return self * (min_length / square_length.sqrt());
        }

        self
    }

    /// Return this vector with minimum and maximum lengths applied.
    #[inline]
    pub fn clamp_length(self, min: T, max: T) -> Self {
        debug_assert!(min <= max);
        self.with_min_length(min).with_max_length(max)
    }

    /// Returns true if all members are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

// impl<T> Vector3D<T>
// where
//     T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
// {
//     /// Linearly interpolate each component between this vector and another vector.
//     ///
//     /// # Example
//     ///
//     /// ```rust
//     /// use euclid::vec3;
//     /// use euclid::default::Vector3D;
//     ///
//     /// let from: Vector3D<_> = Vector3D::new(0.0, 10.0, -1.0);
//     /// let to:  Vector3D<_> = Vector3D::new(8.0, -4.0,  0.0);
//     ///
//     /// assert_eq!(from.lerp(to, -1.0), Vector3D::new(-8.0,  24.0, -2.0));
//     /// assert_eq!(from.lerp(to,  0.0), Vector3D::new( 0.0,  10.0, -1.0));
//     /// assert_eq!(from.lerp(to,  0.5), Vector3D::new( 4.0,   3.0, -0.5));
//     /// assert_eq!(from.lerp(to,  1.0), Vector3D::new( 8.0,  -4.0,  0.0));
//     /// assert_eq!(from.lerp(to,  2.0), Vector3D::new(16.0, -18.0,  1.0));
//     /// ```
//     #[inline]
//     pub fn lerp(self, other: Self, t: T) -> Self {
//         let one_t = T::one() - t;
//         self * one_t + other * t
//     }

//     /// Returns a reflection vector using an incident ray and a surface normal.
//     #[inline]
//     pub fn reflect(self, normal: Self) -> Self {
//         let two = T::one() + T::one();
//         self - normal * two * self.dot(normal)
//     }
// }

impl<T: PartialOrd> Vector3D<T> {
    // /// Returns the vector each component of which are minimum of this vector and another.
    // #[inline]
    // pub fn min(self, other: Self) -> Self {
    //     Vector3D::new(
    //         min(self.x, other.x),
    //         min(self.y, other.y),
    //         min(self.z, other.z),
    //     )
    // }

    // /// Returns the vector each component of which are maximum of this vector and another.
    // #[inline]
    // pub fn max(self, other: Self) -> Self {
    //     Vector3D::new(
    //         max(self.x, other.x),
    //         max(self.y, other.y),
    //         max(self.z, other.z),
    //     )
    // }

    // /// Returns the vector each component of which is clamped by corresponding
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

    // /// Returns vector with results of "greater than" operation on each component.
    // #[inline]
    // pub fn greater_than(self, other: Self) -> BoolVector3D {
    //     BoolVector3D {
    //         x: self.x > other.x,
    //         y: self.y > other.y,
    //         z: self.z > other.z,
    //     }
    // }

    // /// Returns vector with results of "lower than" operation on each component.
    // #[inline]
    // pub fn lower_than(self, other: Self) -> BoolVector3D {
    //     BoolVector3D {
    //         x: self.x < other.x,
    //         y: self.y < other.y,
    //         z: self.z < other.z,
    //     }
    // }
}

// impl<T: PartialEq> Vector3D<T> {
//     /// Returns vector with results of "equal" operation on each component.
//     #[inline]
//     pub fn equal(self, other: Self) -> BoolVector3D {
//         BoolVector3D {
//             x: self.x == other.x,
//             y: self.y == other.y,
//             z: self.z == other.z,
//         }
//     }

//     /// Returns vector with results of "not equal" operation on each component.
//     #[inline]
//     pub fn not_equal(self, other: Self) -> BoolVector3D {
//         BoolVector3D {
//             x: self.x != other.x,
//             y: self.y != other.y,
//             z: self.z != other.z,
//         }
//     }
// }

impl<T: NumCast + Copy> Vector3D<T> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn cast<NewT: NumCast>(self) -> Vector3D<NewT> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    pub fn try_cast<NewT: NumCast>(self) -> Option<Vector3D<NewT>> {
        match (
            NumCast::from(self.x),
            NumCast::from(self.y),
            NumCast::from(self.z),
        ) {
            (Some(x), Some(y), Some(z)) => Some(Vector3D::new(x, y, z)),
            _ => None,
        }
    }

    // Convenience functions for common casts.

    /// Cast into an `f32` vector.
    #[inline]
    pub fn to_f32(self) -> Vector3D<f32> {
        self.cast()
    }

    /// Cast into an `f64` vector.
    #[inline]
    pub fn to_f64(self) -> Vector3D<f64> {
        self.cast()
    }

    /// Cast into an `usize` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_usize(self) -> Vector3D<usize> {
        self.cast()
    }

    /// Cast into an `u32` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_u32(self) -> Vector3D<u32> {
        self.cast()
    }

    /// Cast into an `i32` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i32(self) -> Vector3D<i32> {
        self.cast()
    }

    /// Cast into an `i64` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i64(self) -> Vector3D<i64> {
        self.cast()
    }
}

impl<T: Neg> Neg for Vector3D<T> {
    type Output = Vector3D<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Add> Add for Vector3D<T> {
    type Output = Vector3D<T::Output>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Vector3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

// impl<'a, T: 'a + Add + Copy: 'a> Add<&Self> for Vector3D<T> {
//     type Output = Vector3D<T::Output>;

//     #[inline]
//     fn add(self, other: &Self) -> Self::Output {
//         Vector3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
//     }
// }

// impl<T: Add<Output = T> + Zero> Sum for Vector3D<T> {
//     fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
//         iter.fold(Self::zero(), Add::add)
//     }
// }

// impl<'a, T: 'a + Add<Output = T> + Copy + Zero: 'a> Sum<&'a Self> for Vector3D<T> {
//     fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
//         iter.fold(Self::zero(), Add::add)
//     }
// }

impl<T: Copy + Add<T, Output = T>> AddAssign for Vector3D<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl<T: Sub> Sub for Vector3D<T> {
    type Output = Vector3D<T::Output>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Vector3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: Copy + Sub<T, Output = T>> SubAssign<Vector3D<T>> for Vector3D<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl<T: Copy + Mul> Mul<T> for Vector3D<T> {
    type Output = Vector3D<T::Output>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Vector3D::new(
            self.x * scale,
            self.y * scale,
            self.z * scale,
        )
    }
}

impl<T: Copy + Mul<T, Output = T>> MulAssign<T> for Vector3D<T> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        *self = *self * scale
    }
}

// impl<T: Copy + Mul> Mul<Scale<T>> for Vector3D<T> {
//     type Output = Vector3D<T::Output>;

//     #[inline]
//     fn mul(self, scale: Scale<T>) -> Self::Output {
//         Vector3D::new(
//             self.x * scale.0,
//             self.y * scale.0,
//             self.z * scale.0,
//         )
//     }
// }

// impl<T: Copy + MulAssign> MulAssign<Scale<T>> for Vector3D<T> {
//     #[inline]
//     fn mul_assign(&mut self, scale: Scale<T>) {
//         self.x *= scale.0;
//         self.y *= scale.0;
//         self.z *= scale.0;
//     }
// }

impl<T: Copy + Div> Div<T> for Vector3D<T> {
    type Output = Vector3D<T::Output>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Vector3D::new(
            self.x / scale,
            self.y / scale,
            self.z / scale,
        )
    }
}

impl<T: Copy + Div<T, Output = T>> DivAssign<T> for Vector3D<T> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        *self = *self / scale
    }
}

// impl<T: Copy + Div> Div<Scale<T>> for Vector3D<T> {
//     type Output = Vector3D<T::Output>;

//     #[inline]
//     fn div(self, scale: Scale<T>) -> Self::Output {
//         Vector3D::new(
//             self.x / scale.0,
//             self.y / scale.0,
//             self.z / scale.0,
//         )
//     }
// }

// impl<T: Copy + DivAssign> DivAssign<Scale<T>> for Vector3D<T> {
//     #[inline]
//     fn div_assign(&mut self, scale: Scale<T>) {
//         self.x /= scale.0;
//         self.y /= scale.0;
//         self.z /= scale.0;
//     }
// }

// impl<T: Round> Round for Vector3D<T> {
//     /// See [`Vector3D::round()`](#method.round)
//     #[inline]
//     fn round(self) -> Self {
//         self.round()
//     }
// }

// impl<T: Ceil> Ceil for Vector3D<T> {
//     /// See [`Vector3D::ceil()`](#method.ceil)
//     #[inline]
//     fn ceil(self) -> Self {
//         self.ceil()
//     }
// }

// impl<T: Floor> Floor for Vector3D<T> {
//     /// See [`Vector3D::floor()`](#method.floor)
//     #[inline]
//     fn floor(self) -> Self {
//         self.floor()
//     }
// }

// impl<T: ApproxEq<T>> ApproxEq<Vector3D<T>> for Vector3D<T> {
//     #[inline]
//     fn approx_epsilon() -> Self {
//         Vector3D::new(
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

impl<T> Into<[T; 3]> for Vector3D<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T> From<[T; 3]> for Vector3D<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Vector3D::new(x, y, z)
    }
}

impl<T> Into<(T, T, T)> for Vector3D<T> {
    fn into(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T> From<(T, T, T)> for Vector3D<T> {
    fn from(tuple: (T, T, T)) -> Self {
        Vector3D::new(tuple.0, tuple.1, tuple.2)
    }
}

#[cfg(test)]
mod vector3d {
    use crate::foundation::Vector3D;
    // use crate::scale::Scale;
    // use crate::{default, vec2, vec3};

    #[cfg(feature = "mint")]
    use mint;

    type Vec3 = Vector3D<f32>;

    // #[test]
    // pub fn test_add() {
    //     let p1 = Vec3::new(1.0, 2.0, 3.0);
    //     let p2 = Vec3::new(4.0, 5.0, 6.0);

    //     assert_eq!(p1 + p2, Vector3D::new(5.0, 7.0, 9.0));
    //     assert_eq!(p1 + &p2, Vector3D::new(5.0, 7.0, 9.0));
    // }

    // #[test]
    // pub fn test_sum() {
    //     let vecs = [
    //         Vec3::new(1.0, 2.0, 3.0),
    //         Vec3::new(4.0, 5.0, 6.0),
    //         Vec3::new(7.0, 8.0, 9.0)
    //     ];
    //     let sum = Vec3::new(12.0, 15.0, 18.0);
    //     assert_eq!(vecs.iter().sum::<Vec3>(), sum);
    //     assert_eq!(vecs.into_iter().sum::<Vec3>(), sum);
    // }

    #[test]
    pub fn test_dot() {
        let p1: Vec3 = Vector3D::new(7.0, 21.0, 32.0);
        let p2: Vec3 = Vector3D::new(43.0, 5.0, 16.0);
        assert_eq!(p1.dot(p2), 918.0);
    }

    #[test]
    pub fn test_cross() {
        let p1: Vec3 = Vector3D::new(4.0, 7.0, 9.0);
        let p2: Vec3 = Vector3D::new(13.0, 8.0, 3.0);
        let p3 = p1.cross(p2);
        assert_eq!(p3, Vector3D::new(-51.0, 105.0, -59.0));
    }

    // #[test]
    // pub fn test_normalize() {
    //     use std::f32;

    //     let p0: Vec3 = Vec3::zero();
    //     let p1: Vec3 = Vector3D::new(0.0, -6.0, 0.0);
    //     let p2: Vec3 = Vector3D::new(1.0, 2.0, -2.0);
    //     assert!(
    //         p0.normalize().x.is_nan() && p0.normalize().y.is_nan() && p0.normalize().z.is_nan()
    //     );
    //     assert_eq!(p1.normalize(), Vector3D::new(0.0, -1.0, 0.0));
    //     assert_eq!(p2.normalize(), Vector3D::new(1.0 / 3.0, 2.0 / 3.0, -2.0 / 3.0));

    //     let p3: Vec3 = Vector3D::new(::std::f32::MAX, ::std::f32::MAX, 0.0);
    //     assert_ne!(
    //         p3.normalize(),
    //         Vector3D::new(1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt(), 0.0)
    //     );
    //     assert_eq!(
    //         p3.robust_normalize(),
    //         Vector3D::new(1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt(), 0.0)
    //     );

    //     let p4: Vec3 = Vec3::zero();
    //     assert!(p4.try_normalize().is_none());
    //     let p5: Vec3 = Vec3::new(f32::MIN_POSITIVE, f32::MIN_POSITIVE, f32::MIN_POSITIVE);
    //     assert!(p5.try_normalize().is_none());

    //     let p6: Vec3 = Vector3D::new(4.0, 0.0, 3.0);
    //     let p7: Vec3 = Vector3D::new(3.0, -4.0, 0.0);
    //     assert_eq!(p6.try_normalize().unwrap(), Vector3D::new(0.8, 0.0, 0.6));
    //     assert_eq!(p7.try_normalize().unwrap(), Vector3D::new(0.6, -0.8, 0.0));
    // }

    // #[test]
    // pub fn test_min() {
    //     let p1: Vec3 = Vector3D::new(1.0, 3.0, 5.0);
    //     let p2: Vec3 = Vector3D::new(2.0, 2.0, -1.0);

    //     let result = p1.min(p2);

    //     assert_eq!(result, Vector3D::new(1.0, 2.0, -1.0));
    // }

    // #[test]
    // pub fn test_max() {
    //     let p1: Vec3 = Vector3D::new(1.0, 3.0, 5.0);
    //     let p2: Vec3 = Vector3D::new(2.0, 2.0, -1.0);

    //     let result = p1.max(p2);

    //     assert_eq!(result, Vector3D::new(2.0, 3.0, 5.0));
    // }

    // #[test]
    // pub fn test_clamp() {
    //     let p1: Vec3 = Vector3D::new(1.0, -1.0, 5.0);
    //     let p2: Vec3 = Vector3D::new(2.0, 5.0, 10.0);
    //     let p3: Vec3 = Vector3D::new(-1.0, 2.0, 20.0);

    //     let result = p3.clamp(p1, p2);

    //     assert_eq!(result, Vector3D::new(1.0, 2.0, 10.0));
    // }

    // #[test]
    // pub fn test_typed_scalar_mul() {
    //     enum Mm {}
    //     enum Cm {}

    //     let p1 = super::Vector3D::<f32, Mm>::new(1.0, 2.0, 3.0);
    //     let cm_per_mm = Scale::<f32, Mm, Cm>::new(0.1);

    //     let result: super::Vector3D<f32, Cm> = p1 * cm_per_mm;

    //     assert_eq!(result, Vector3D::new(0.1, 0.2, 0.3));
    // }

    // #[test]
    // pub fn test_swizzling() {
    //     let p: Vec3 = Vector3D::new(1.0, 2.0, 3.0);
    //     assert_eq!(p.xy(), vec2(1.0, 2.0));
    //     assert_eq!(p.xz(), vec2(1.0, 3.0));
    //     assert_eq!(p.yz(), vec2(2.0, 3.0));
    // }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let v1 = Vec3::new(1.0, 3.0, 5.0);
        let vm: mint::Vector3<_> = v1.into();
        let v2 = Vec3::from(vm);

        assert_eq!(v1, v2);
    }

    // #[test]
    // pub fn test_reflect() {
    //     // use crate::approxeq::ApproxEq;
    //     let a: Vec3 = Vector3D::new(1.0, 3.0, 2.0);
    //     let n1: Vec3 = Vector3D::new(0.0, -1.0, 0.0);
    //     let n2: Vec3 = Vector3D::new(0.0, 1.0, 1.0).normalize();

    //     assert!(a.reflect(n1).approx_eq(&Vector3D::new(1.0, -3.0, 2.0)));
    //     assert!(a.reflect(n2).approx_eq(&Vector3D::new(1.0, -2.0, -3.0)));
    // }

    // #[test]
    // pub fn test_angle_to() {
    //     // use crate::approxeq::ApproxEq;
    //     use core::f32::consts::FRAC_PI_2;

    //     let right: Vec3 = Vector3D::new(10.0, 0.0, 0.0);
    //     let right2: Vec3 = Vector3D::new(1.0, 0.0, 0.0);
    //     let up: Vec3 = Vector3D::new(0.0, -1.0, 0.0);
    //     let up_left: Vec3 = Vector3D::new(-1.0, -1.0, 0.0);

    //     assert!(right.angle_to(right2).get().approx_eq(&0.0));
    //     assert!(right.angle_to(up).get().approx_eq(&FRAC_PI_2));
    //     assert!(up.angle_to(right).get().approx_eq(&FRAC_PI_2));
    //     assert!(up_left
    //         .angle_to(up)
    //         .get()
    //         .approx_eq_eps(&(0.5 * FRAC_PI_2), &0.0005));
    // }

    // #[test]
    // pub fn test_with_max_length() {
    //     // use crate::approxeq::ApproxEq;

    //     let v1: Vec3 = Vector3D::new(0.5, 0.5, 0.0);
    //     let v2: Vec3 = Vector3D::new(1.0, 0.0, 0.0);
    //     let v3: Vec3 = Vector3D::new(0.1, 0.2, 0.3);
    //     let v4: Vec3 = Vector3D::new(2.0, -2.0, 2.0);
    //     let v5: Vec3 = Vector3D::new(1.0, 2.0, -3.0);
    //     let v6: Vec3 = Vector3D::new(-1.0, 3.0, 2.0);

    //     assert_eq!(v1.with_max_length(1.0), v1);
    //     assert_eq!(v2.with_max_length(1.0), v2);
    //     assert_eq!(v3.with_max_length(1.0), v3);
    //     assert_eq!(v4.with_max_length(10.0), v4);
    //     assert_eq!(v5.with_max_length(10.0), v5);
    //     assert_eq!(v6.with_max_length(10.0), v6);

    //     let v4_clamped = v4.with_max_length(1.0);
    //     assert!(v4_clamped.length().approx_eq(&1.0));
    //     assert!(v4_clamped.normalize().approx_eq(&v4.normalize()));

    //     let v5_clamped = v5.with_max_length(1.5);
    //     assert!(v5_clamped.length().approx_eq(&1.5));
    //     assert!(v5_clamped.normalize().approx_eq(&v5.normalize()));

    //     let v6_clamped = v6.with_max_length(2.5);
    //     assert!(v6_clamped.length().approx_eq(&2.5));
    //     assert!(v6_clamped.normalize().approx_eq(&v6.normalize()));
    // }

    // #[test]
    // pub fn test_project_onto_vector() {
    //     // use crate::approxeq::ApproxEq;

    //     let v1: Vec3 = Vector3D::new(1.0, 2.0, 3.0);
    //     let x: Vec3 = Vector3D::new(1.0, 0.0, 0.0);
    //     let y: Vec3 = Vector3D::new(0.0, 1.0, 0.0);
    //     let z: Vec3 = Vector3D::new(0.0, 0.0, 1.0);

    //     assert!(v1.project_onto_vector(x).approx_eq(&Vector3D::new(1.0, 0.0, 0.0)));
    //     assert!(v1.project_onto_vector(y).approx_eq(&Vector3D::new(0.0, 2.0, 0.0)));
    //     assert!(v1.project_onto_vector(z).approx_eq(&Vector3D::new(0.0, 0.0, 3.0)));
    //     assert!(v1.project_onto_vector(-x).approx_eq(&Vector3D::new(1.0, 0.0, 0.0)));
    //     assert!(v1
    //         .project_onto_vector(x * 10.0)
    //         .approx_eq(&Vector3D::new(1.0, 0.0, 0.0)));
    //     assert!(v1.project_onto_vector(v1 * 2.0).approx_eq(&v1));
    //     assert!(v1.project_onto_vector(-v1).approx_eq(&v1));
    // }
}