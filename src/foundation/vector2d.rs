use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
// use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};


use num_traits::{Float, NumCast, Signed};

/// A 2d Vector tagged with a unit.
#[repr(C)]
pub struct Vector2D<T> {
    /// The `x` (traditionally, horizontal) coordinate.
    pub x: T,
    /// The `y` (traditionally, vertical) coordinate.
    pub y: T,
}

// mint_vec!(Vector2D[x, y] = Vector2);

impl<T: Copy> Copy for Vector2D<T> {}

impl<T: Clone> Clone for Vector2D<T> {
    fn clone(&self) -> Self {
        Vector2D {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Vector2D<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Vector2D {
            x,
            y,
        })
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Vector2D<T>
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
impl<'a, T> arbitrary::Arbitrary<'a> for Vector2D<T>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self>
    {
        let (x, y) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(Vector2D {
            x,
            y,
        })
    }
}

impl<T: Eq> Eq for Vector2D<T> {}

impl<T: PartialEq> PartialEq for Vector2D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Hash> Hash for Vector2D<T> {
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
    }
}

// impl<T: Zero> Zero for Vector2D<T> {
//     /// Constructor, setting all components to zero.
//     #[inline]
//     fn zero() -> Self {
//         Vector2D::new(Zero::zero(), Zero::zero())
//     }
// }

impl<T: fmt::Debug> fmt::Debug for Vector2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl<T: Default> Default for Vector2D<T> {
    fn default() -> Self {
        Vector2D::new(Default::default(), Default::default())
    }
}

impl<T> Vector2D<T> {
    // /// Constructor, setting all components to zero.
    // #[inline]
    // pub fn zero() -> Self
    // where
    //     T: Zero,
    // {
    //     Vector2D::new(Zero::zero(), Zero::zero())
    // }

    // /// Constructor, setting all components to one.
    // #[inline]
    // pub fn one() -> Self
    // where
    //     T: One,
    // {
    //     Vector2D::new(One::one(), One::one())
    // }

    /// Constructor taking scalar values directly.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Vector2D {
            x,
            y,
        }
    }

    /// Constructor setting all components to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Vector2D {
            x: v.clone(),
            y: v,
        }
    }

    // /// Constructor taking angle and length
    // pub fn from_angle_and_length(angle: Angle<T>, length: T) -> Self
    // where
    //     T: Trig + Mul<Output = T> + Copy,
    // {
    //     Vector2D::new(length * angle.radians.cos(), length * angle.radians.sin())
    // }

    // /// Constructor taking properly  Lengths instead of scalar values.
    // #[inline]
    // pub fn from_lengths(x: Length<T>, y: Length<T>) -> Self {
    //     Vector2D::new(x.0, y.0)
    // }

    // /// Tag a unit-less value with units.
    // #[inline]
    // pub fn from_untyped(p: Vector2D<T>) -> Self {
    //     Vector2D::new(p.x, p.y)
    // }

    /// Computes the vector with absolute values of each component.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::{i32, f32};
    /// # use primitives::foundation::Vector2D;
    /// enum U {}
    ///
    /// assert_eq!(Vector2D::new(-1, 2).abs(), Vector2D::new(1, 2));
    ///
    /// let vec = Vector2D::new(f32::NAN, -f32::MAX).abs();
    /// assert!(vec.x.is_nan());
    /// assert_eq!(vec.y, f32::MAX);
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
        Vector2D::new(self.x.abs(), self.y.abs())
    }

    /// Dot product.
    #[inline]
    pub fn dot(self, other: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.x * other.x + self.y * other.y
    }

    /// Returns the norm of the cross product [self.x, self.y, 0] x [other.x, other.y, 0].
    #[inline]
    pub fn cross(self, other: Self) -> T
    where
        T: Sub<Output = T> + Mul<Output = T>,
    {
        self.x * other.y - self.y * other.x
    }

    /// Returns the component-wise multiplication of the two vectors.
    #[inline]
    pub fn component_mul(self, other: Self) -> Self
    where
        T: Mul<Output = T>,
    {
        Vector2D::new(self.x * other.x, self.y * other.y)
    }

    /// Returns the component-wise division of the two vectors.
    #[inline]
    pub fn component_div(self, other: Self) -> Self
    where
        T: Div<Output = T>,
    {
        Vector2D::new(self.x / other.x, self.y / other.y)
    }
}

impl<T: Copy> Vector2D<T> {
    // /// Create a 3d vector from this one, using the specified z value.
    // #[inline]
    // pub fn extend(self, z: T) -> Vector3D<T> {
    //     vec3(self.x, self.y, z)
    // }

    // /// Cast this vector into a point.
    // ///
    // /// Equivalent to adding this vector to the origin.
    // #[inline]
    // pub fn to_point(self) -> Point2D<T> {
    //     Point2D {
    //         x: self.x,
    //         y: self.y,
    //     }
    // }

    /// Swap x and y.
    #[inline]
    pub fn yx(self) -> Self {
        Vector2D::new(self.y, self.x)
    }

    // /// Cast this vector into a size.
    // #[inline]
    // pub fn to_size(self) -> Size2D<T> {
    //     size2(self.x, self.y)
    // }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(self) -> Vector2D<T> {
        Vector2D::new(self.x, self.y)
    }

    // /// Cast the unit.
    // #[inline]
    // pub fn cast_unit<V>(self) -> Vector2D<T, V> {
    //     Vector2D::new(self.x, self.y)
    // }

    /// Cast into an array with x and y.
    #[inline]
    pub fn to_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    /// Cast into a tuple with x and y.
    #[inline]
    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    // /// Convert into a 3d vector with `z` coordinate equals to `T::zero()`.
    // #[inline]
    // pub fn to_3d(self) -> Vector3D<T>
    // where
    //     T: Zero,
    // {
    //     vec3(self.x, self.y, Zero::zero())
    // }

    // /// Rounds each component to the nearest integer value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::vec2;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(vec2::<_, Mm>(-0.1, -0.8).round(), vec2::<_, Mm>(0.0, -1.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn round(self) -> Self
    // where
    //     T: Round,
    // {
    //     Vector2D::new(self.x.round(), self.y.round())
    // }

    // /// Rounds each component to the smallest integer equal or greater than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::vec2;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(vec2::<_, Mm>(-0.1, -0.8).ceil(), vec2::<_, Mm>(0.0, 0.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn ceil(self) -> Self
    // where
    //     T: Ceil,
    // {
    //     Vector2D::new(self.x.ceil(), self.y.ceil())
    // }

    // /// Rounds each component to the biggest integer equal or lower than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::vec2;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(vec2::<_, Mm>(-0.1, -0.8).floor(), vec2::<_, Mm>(-1.0, -1.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn floor(self) -> Self
    // where
    //     T: Floor,
    // {
    //     Vector2D::new(self.x.floor(), self.y.floor())
    // }

    // /// Returns the signed angle between this vector and the x axis.
    // /// Positive values counted counterclockwise, where 0 is `+x` axis, `PI/2`
    // /// is `+y` axis.
    // ///
    // /// The returned angle is between -PI and PI.
    // pub fn angle_from_x_axis(self) -> Angle<T>
    // where
    //     T: Trig,
    // {
    //     Angle::radians(Trig::fast_atan2(self.y, self.x))
    // }

    // /// Creates translation by this vector in vector units.
    // #[inline]
    // pub fn to_transform(self) -> Transform2D<T>
    // where
    //     T: Zero + One,
    // {
    //     Transform2D::translation(self.x, self.y)
    // }
}

impl<T> Vector2D<T>
where
    T: Copy + Mul<T, Output = T> + Add<T, Output = T>,
{
    /// Returns the vector's length squared.
    #[inline]
    pub fn square_length(self) -> T {
        self.x * self.x + self.y * self.y
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

    // /// Returns the signed angle between this vector and another vector.
    // ///
    // /// The returned angle is between -PI and PI.
    // pub fn angle_to(self, other: Self) -> Angle<T>
    // where
    //     T: Sub<Output = T> + Trig,
    // {
    //     Angle::radians(Trig::fast_atan2(self.cross(other), self.dot(other)))
    // }
}

impl<T: Float> Vector2D<T> {
    /// Returns the vector length.
    #[inline]
    pub fn length(self) -> T {
        self.square_length().sqrt()
    }

    /// Returns the vector with length of one unit.
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
        self.x.is_finite() && self.y.is_finite()
    }
}

// impl<T> Vector2D<T>
// where
//     T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
// {
//     /// Linearly interpolate each component between this vector and another vector.
//     ///
//     /// # Example
//     ///
//     /// ```rust
//     /// use euclid::vec2;
//     /// use euclid::default::Vector2D;
//     ///
//     /// let from: Vector2D<_> = Vector2D::new(0.0, 10.0);
//     /// let to:  Vector2D<_> = Vector2D::new(8.0, -4.0);
//     ///
//     /// assert_eq!(from.lerp(to, -1.0), Vector2D::new(-8.0,  24.0));
//     /// assert_eq!(from.lerp(to,  0.0), Vector2D::new( 0.0,  10.0));
//     /// assert_eq!(from.lerp(to,  0.5), Vector2D::new( 4.0,   3.0));
//     /// assert_eq!(from.lerp(to,  1.0), Vector2D::new( 8.0,  -4.0));
//     /// assert_eq!(from.lerp(to,  2.0), Vector2D::new(16.0, -18.0));
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

// impl<T: PartialOrd> Vector2D<T> {
//     /// Returns the vector each component of which are minimum of this vector and another.
//     #[inline]
//     pub fn min(self, other: Self) -> Self {
//         Vector2D::new(min(self.x, other.x), min(self.y, other.y))
//     }

//     /// Returns the vector each component of which are maximum of this vector and another.
//     #[inline]
//     pub fn max(self, other: Self) -> Self {
//         Vector2D::new(max(self.x, other.x), max(self.y, other.y))
//     }

//     /// Returns the vector each component of which is clamped by corresponding
//     /// components of `start` and `end`.
//     ///
//     /// Shortcut for `self.max(start).min(end)`.
//     #[inline]
//     pub fn clamp(self, start: Self, end: Self) -> Self
//     where
//         T: Copy,
//     {
//         self.max(start).min(end)
//     }

//     /// Returns vector with results of "greater than" operation on each component.
//     #[inline]
//     pub fn greater_than(self, other: Self) -> BoolVector2D {
//         BoolVector2D {
//             x: self.x > other.x,
//             y: self.y > other.y,
//         }
//     }

//     /// Returns vector with results of "lower than" operation on each component.
//     #[inline]
//     pub fn lower_than(self, other: Self) -> BoolVector2D {
//         BoolVector2D {
//             x: self.x < other.x,
//             y: self.y < other.y,
//         }
//     }
// }

// impl<T: PartialEq> Vector2D<T> {
//     /// Returns vector with results of "equal" operation on each component.
//     #[inline]
//     pub fn equal(self, other: Self) -> BoolVector2D {
//         BoolVector2D {
//             x: self.x == other.x,
//             y: self.y == other.y,
//         }
//     }

//     /// Returns vector with results of "not equal" operation on each component.
//     #[inline]
//     pub fn not_equal(self, other: Self) -> BoolVector2D {
//         BoolVector2D {
//             x: self.x != other.x,
//             y: self.y != other.y,
//         }
//     }
// }

impl<T: NumCast + Copy> Vector2D<T> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn cast<NewT: NumCast>(self) -> Vector2D<NewT> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    pub fn try_cast<NewT: NumCast>(self) -> Option<Vector2D<NewT>> {
        match (NumCast::from(self.x), NumCast::from(self.y)) {
            (Some(x), Some(y)) => Some(Vector2D::new(x, y)),
            _ => None,
        }
    }

    // Convenience functions for common casts.

    /// Cast into an `f32` vector.
    #[inline]
    pub fn to_f32(self) -> Vector2D<f32> {
        self.cast()
    }

    /// Cast into an `f64` vector.
    #[inline]
    pub fn to_f64(self) -> Vector2D<f64> {
        self.cast()
    }

    /// Cast into an `usize` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_usize(self) -> Vector2D<usize> {
        self.cast()
    }

    /// Cast into an `u32` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_u32(self) -> Vector2D<u32> {
        self.cast()
    }

    /// Cast into an i32 vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i32(self) -> Vector2D<i32> {
        self.cast()
    }

    /// Cast into an i64 vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i64(self) -> Vector2D<i64> {
        self.cast()
    }
}

impl<T: Neg> Neg for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector2D::new(-self.x, -self.y)
    }
}

impl<T: Add> Add for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Add + Copy> Add<&Self> for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    #[inline]
    fn add(self, other: &Self) -> Self::Output {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

// impl<T: Add<Output = T> + Zero> Sum for Vector2D<T> {
//     fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
//         iter.fold(Self::zero(), Add::add)
//     }
// }

// impl<'a, T: 'a + Add<Output = T> + Copy + Zero: 'a> Sum<&'a Self> for Vector2D<T> {
//     fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
//         iter.fold(Self::zero(), Add::add)
//     }
// }

impl<T: Copy + Add<T, Output = T>> AddAssign for Vector2D<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl<T: Sub> Sub for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Copy + Sub<T, Output = T>> SubAssign<Vector2D<T>> for Vector2D<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl<T: Copy + Mul> Mul<T> for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Vector2D::new(self.x * scale, self.y * scale)
    }
}

impl<T: Copy + Mul<T, Output = T>> MulAssign<T> for Vector2D<T> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        *self = *self * scale
    }
}

// impl<T: Copy + Mul> Mul<Scale<T>> for Vector2D<T> {
//     type Output = Vector2D<T::Output>;

//     #[inline]
//     fn mul(self, scale: Scale<T>) -> Self::Output {
//         Vector2D::new(self.x * scale.0, self.y * scale.0)
//     }
// }

// impl<T: Copy + MulAssign> MulAssign<Scale<T>> for Vector2D<T> {
//     #[inline]
//     fn mul_assign(&mut self, scale: Scale<T>) {
//         self.x *= scale.0;
//         self.y *= scale.0;
//     }
// }

impl<T: Copy + Div> Div<T> for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Vector2D::new(self.x / scale, self.y / scale)
    }
}

impl<T: Copy + Div<T, Output = T>> DivAssign<T> for Vector2D<T> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        *self = *self / scale
    }
}

// impl<T: Copy + Div> Div<Scale<T>> for Vector2D<T> {
//     type Output = Vector2D<T::Output>;

//     #[inline]
//     fn div(self, scale: Scale<T>) -> Self::Output {
//         Vector2D::new(self.x / scale.0, self.y / scale.0)
//     }
// }

// impl<T: Copy + DivAssign> DivAssign<Scale<T>> for Vector2D<T> {
//     #[inline]
//     fn div_assign(&mut self, scale: Scale<T>) {
//         self.x /= scale.0;
//         self.y /= scale.0;
//     }
// }

// impl<T: Round> Round for Vector2D<T> {
//     /// See [`Vector2D::round()`](#method.round)
//     #[inline]
//     fn round(self) -> Self {
//         self.round()
//     }
// }

// impl<T: Ceil> Ceil for Vector2D<T> {
//     /// See [`Vector2D::ceil()`](#method.ceil)
//     #[inline]
//     fn ceil(self) -> Self {
//         self.ceil()
//     }
// }

// impl<T: Floor> Floor for Vector2D<T> {
//     /// See [`Vector2D::floor()`](#method.floor)
//     #[inline]
//     fn floor(self) -> Self {
//         self.floor()
//     }
// }

// impl<T: ApproxEq<T>> ApproxEq<Vector2D<T>> for Vector2D<T> {
//     #[inline]
//     fn approx_epsilon() -> Self {
//         Vector2D::new(T::approx_epsilon(), T::approx_epsilon())
//     }

//     #[inline]
//     fn approx_eq_eps(&self, other: &Self, eps: &Self) -> bool {
//         self.x.approx_eq_eps(&other.x, &eps.x) && self.y.approx_eq_eps(&other.y, &eps.y)
//     }
// }

impl<T> Into<[T; 2]> for Vector2D<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T> From<[T; 2]> for Vector2D<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Vector2D::new(x, y)
    }
}

impl<T> Into<(T, T)> for Vector2D<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> From<(T, T)> for Vector2D<T> {
    fn from(tuple: (T, T)) -> Self {
        Vector2D::new(tuple.0, tuple.1)
    }
}

// impl<T> From<Size2D<T>> for Vector2D<T> {
//     fn from(size: Size2D<T>) -> Self {
//         Vector2D::new(size.width, size.height)
//     }
// }

#[cfg(test)]
mod vector2d {
    use crate::foundation::Vector2D;
    // use crate::scale::Scale;
    // use crate::{default};

    #[cfg(feature = "mint")]
    use mint;

    type Vec2 = Vector2D<f32>;

    #[test]
    pub fn test_scalar_mul() {
        let p1: Vec2 = Vector2D::new(3.0, 5.0);

        let result = p1 * 5.0;

        assert_eq!(result, Vec2::new(15.0, 25.0));
    }

    #[test]
    pub fn test_dot() {
        let p1: Vec2 = Vector2D::new(2.0, 7.0);
        let p2: Vec2 = Vector2D::new(13.0, 11.0);
        assert_eq!(p1.dot(p2), 103.0);
    }

    #[test]
    pub fn test_cross() {
        let p1: Vec2 = Vector2D::new(4.0, 7.0);
        let p2: Vec2 = Vector2D::new(13.0, 8.0);
        let r = p1.cross(p2);
        assert_eq!(r, -59.0);
    }

    // #[test]
    // pub fn test_normalize() {
    //     use std::f32;

    //     let p0: Vec2 = Vec2::zero();
    //     let p1: Vec2 = Vector2D::new(4.0, 0.0);
    //     let p2: Vec2 = Vector2D::new(3.0, -4.0);
    //     assert!(p0.normalize().x.is_nan() && p0.normalize().y.is_nan());
    //     assert_eq!(p1.normalize(), Vector2D::new(1.0, 0.0));
    //     assert_eq!(p2.normalize(), Vector2D::new(0.6, -0.8));

    //     let p3: Vec2 = Vector2D::new(::std::f32::MAX, ::std::f32::MAX);
    //     assert_ne!(
    //         p3.normalize(),
    //         Vector2D::new(1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt())
    //     );
    //     assert_eq!(
    //         p3.robust_normalize(),
    //         Vector2D::new(1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt())
    //     );

    //     let p4: Vec2 = Vec2::zero();
    //     assert!(p4.try_normalize().is_none());
    //     let p5: Vec2 = Vec2::new(f32::MIN_POSITIVE, f32::MIN_POSITIVE);
    //     assert!(p5.try_normalize().is_none());

    //     let p6: Vec2 = Vector2D::new(4.0, 0.0);
    //     let p7: Vec2 = Vector2D::new(3.0, -4.0);
    //     assert_eq!(p6.try_normalize().unwrap(), Vector2D::new(1.0, 0.0));
    //     assert_eq!(p7.try_normalize().unwrap(), Vector2D::new(0.6, -0.8));
    // }

    // #[test]
    // pub fn test_min() {
    //     let p1: Vec2 = Vector2D::new(1.0, 3.0);
    //     let p2: Vec2 = Vector2D::new(2.0, 2.0);

    //     let result = p1.min(p2);

    //     assert_eq!(result, Vector2D::new(1.0, 2.0));
    // }

    // #[test]
    // pub fn test_max() {
    //     let p1: Vec2 = Vector2D::new(1.0, 3.0);
    //     let p2: Vec2 = Vector2D::new(2.0, 2.0);

    //     let result = p1.max(p2);

    //     assert_eq!(result, Vector2D::new(2.0, 3.0));
    // }

    // #[test]
    // pub fn test_angle_from_x_axis() {
    //     // use crate::approxeq::ApproxEq;
    //     use core::f32::consts::FRAC_PI_2;

    //     let right: Vec2 = Vector2D::new(10.0, 0.0);
    //     let down: Vec2 = Vector2D::new(0.0, 4.0);
    //     let up: Vec2 = Vector2D::new(0.0, -1.0);

    //     assert!(right.angle_from_x_axis().get().approx_eq(&0.0));
    //     assert!(down.angle_from_x_axis().get().approx_eq(&FRAC_PI_2));
    //     assert!(up.angle_from_x_axis().get().approx_eq(&-FRAC_PI_2));
    // }

    // #[test]
    // pub fn test_angle_to() {
    //     // use crate::approxeq::ApproxEq;
    //     use core::f32::consts::FRAC_PI_2;

    //     let right: Vec2 = Vector2D::new(10.0, 0.0);
    //     let right2: Vec2 = Vector2D::new(1.0, 0.0);
    //     let up: Vec2 = Vector2D::new(0.0, -1.0);
    //     let up_left: Vec2 = Vector2D::new(-1.0, -1.0);

    //     assert!(right.angle_to(right2).get().approx_eq(&0.0));
    //     assert!(right.angle_to(up).get().approx_eq(&-FRAC_PI_2));
    //     assert!(up.angle_to(right).get().approx_eq(&FRAC_PI_2));
    //     assert!(up_left
    //         .angle_to(up)
    //         .get()
    //         .approx_eq_eps(&(0.5 * FRAC_PI_2), &0.0005));
    // }

    // #[test]
    // pub fn test_with_max_length() {
    //     // use crate::approxeq::ApproxEq;

    //     let v1: Vec2 = Vector2D::new(0.5, 0.5);
    //     let v2: Vec2 = Vector2D::new(1.0, 0.0);
    //     let v3: Vec2 = Vector2D::new(0.1, 0.2);
    //     let v4: Vec2 = Vector2D::new(2.0, -2.0);
    //     let v5: Vec2 = Vector2D::new(1.0, 2.0);
    //     let v6: Vec2 = Vector2D::new(-1.0, 3.0);

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

    //     let v1: Vec2 = Vector2D::new(1.0, 2.0);
    //     let x: Vec2 = Vector2D::new(1.0, 0.0);
    //     let y: Vec2 = Vector2D::new(0.0, 1.0);

    //     assert!(v1.project_onto_vector(x).approx_eq(&Vector2D::new(1.0, 0.0)));
    //     assert!(v1.project_onto_vector(y).approx_eq(&Vector2D::new(0.0, 2.0)));
    //     assert!(v1.project_onto_vector(-x).approx_eq(&Vector2D::new(1.0, 0.0)));
    //     assert!(v1.project_onto_vector(x * 10.0).approx_eq(&Vector2D::new(1.0, 0.0)));
    //     assert!(v1.project_onto_vector(v1 * 2.0).approx_eq(&v1));
    //     assert!(v1.project_onto_vector(-v1).approx_eq(&v1));
    // }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let v1 = Vec2::new(1.0, 3.0);
        let vm: mint::Vector2<_> = v1.into();
        let v2 = Vec2::from(vm);

        assert_eq!(v1, v2);
    }

    // pub enum Mm {}
    // pub enum Cm {}

    // pub type Vector2DMm<T> = super::Vector2D<T, Mm>;
    // pub type Vector2DCm<T> = super::Vector2D<T, Cm>;

    // #[test]
    // pub fn test_add() {
    //     let p1 = Vector2DMm::new(1.0, 2.0);
    //     let p2 = Vector2DMm::new(3.0, 4.0);

    //     assert_eq!(p1 + p2, Vector2D::new(4.0, 6.0));
    //     assert_eq!(p1 + &p2, Vector2D::new(4.0, 6.0));
    // }

    // #[test]
    // pub fn test_sum() {
    //     let vecs = [
    //         Vector2DMm::new(1.0, 2.0),
    //         Vector2DMm::new(3.0, 4.0),
    //         Vector2DMm::new(5.0, 6.0)
    //     ];
    //     let sum = Vector2DMm::new(9.0, 12.0);
    //     assert_eq!(vecs.iter().sum::<Vector2DMm<_>>(), sum);
    //     assert_eq!(vecs.into_iter().sum::<Vector2DMm<_>>(), sum);
    // }

    // #[test]
    // pub fn test_add_assign() {
    //     let mut p1 = Vector2DMm::new(1.0, 2.0);
    //     p1 += Vector2D::new(3.0, 4.0);

    //     assert_eq!(p1, Vector2D::new(4.0, 6.0));
    // }

    // #[test]
    // pub fn test_tpyed_scalar_mul() {
    //     let p1 = Vector2DMm::new(1.0, 2.0);
    //     let cm_per_mm = Scale::<f32, Mm, Cm>::new(0.1);

    //     let result: Vector2DCm<f32> = p1 * cm_per_mm;

    //     assert_eq!(result, Vector2D::new(0.1, 0.2));
    // }

    #[test]
    pub fn test_swizzling() {
        let p: Vector2D<i32> = Vector2D::new(1, 2);
        assert_eq!(p.yx(), Vector2D::new(2, 1));
    }

    // #[test]
    // pub fn test_reflect() {
    //     // use crate::approxeq::ApproxEq;
    //     let a: Vec2 = Vector2D::new(1.0, 3.0);
    //     let n1: Vec2 = Vector2D::new(0.0, -1.0);
    //     let n2: Vec2 = Vector2D::new(1.0, -1.0).normalize();

    //     assert!(a.reflect(n1).approx_eq(&Vector2D::new(1.0, -3.0)));
    //     assert!(a.reflect(n2).approx_eq(&Vector2D::new(3.0, 1.0)));
    // }
}