use super::{One, Zero};
use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::Hash;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 3d size tagged with a unit.
#[repr(C)]
pub struct Size3D<T> {
    /// The extent of the element in the `U` units along the `x` axis.
    pub width: T,
    /// The extent of the element in the `U` units along the `y` axis.
    pub height: T,
    /// The extent of the element in the `U` units along the `z` axis.
    pub depth: T,
}

impl<T: Copy> Copy for Size3D<T> {}

impl<T: Clone> Clone for Size3D<T> {
    fn clone(&self) -> Self {
        Size3D {
            width: self.width.clone(),
            height: self.height.clone(),
            depth: self.depth.clone(),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Size3D<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (width, height, depth) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Size3D {
            width,
            height,
            depth,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Size3D<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.width, &self.height, &self.depth).serialize(serializer)
    }
}

impl<T> Eq for Size3D<T> where T: Eq {}

impl<T> PartialEq for Size3D<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.depth == other.depth
    }
}

impl<T> Hash for Size3D<T>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.width.hash(h);
        self.height.hash(h);
        self.depth.hash(h);
    }
}

impl<T: fmt::Debug> fmt::Debug for Size3D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.width, f)?;
        write!(f, "x")?;
        fmt::Debug::fmt(&self.height, f)?;
        write!(f, "x")?;
        fmt::Debug::fmt(&self.depth, f)
    }
}

impl<T: Default> Default for Size3D<T> {
    fn default() -> Self {
        Size3D::new(Default::default(), Default::default(), Default::default())
    }
}

impl<T> Size3D<T> {
    /// The same as [`Zero::zero()`] but available without importing trait.
    ///
    /// [`Zero::zero()`]: ./num/trait.Zero.html#tymethod.zero
    pub fn zero() -> Self
    where
        T: Zero,
    {
        Size3D::new(Zero::zero(), Zero::zero(), Zero::zero())
    }

    /// Constructor taking scalar values.
    #[inline]
    pub const fn new(width: T, height: T, depth: T) -> Self {
        Size3D {
            width,
            height,
            depth,
        }
    }

    // TODO:
    // /// Constructor taking scalar strongly typed lengths.
    // #[inline]
    // pub fn from_lengths(width: Length<T>, height: Length<T>, depth: Length<T>) -> Self {
    //     Size3D::new(width.0, height.0, depth.0)
    // }

    /// Constructor setting all components to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Size3D {
            width: v.clone(),
            height: v.clone(),
            depth: v,
        }
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(p: Size3D<T>) -> Self {
        Size3D::new(p.width, p.height, p.depth)
    }
}

impl<T: Copy> Size3D<T> {
    /// Return this size as an array of three elements (width, then height, then depth).
    #[inline]
    pub fn to_array(self) -> [T; 3] {
        [self.width, self.height, self.depth]
    }

    /// Return this size as an array of three elements (width, then height, then depth).
    #[inline]
    pub fn to_tuple(self) -> (T, T, T) {
        (self.width, self.height, self.depth)
    }

    // TODO:
    // /// Return this size as a vector with width, height and depth.
    // #[inline]
    // pub fn to_vector(self) -> Vector3D<T> {
    //     vec3(self.width, self.height, self.depth)
    // }

    // /// Drop the units, preserving only the numeric value.
    // #[inline]
    // pub fn to_untyped(self) -> Size3D<T> {
    //     self.cast_unit()
    // }

    // /// Cast the unit
    // #[inline]
    // pub fn cast_unit<V>(self) -> Size3D<T, V> {
    //     Size3D::new(self.width, self.height, self.depth)
    // }

    // TODO:
    // /// Rounds each component to the nearest integer value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::size3;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(size3::<_, Mm>(-0.1, -0.8, 0.4).round(), size3::<_, Mm>(0.0, -1.0, 0.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn round(self) -> Self
    // where
    //     T: Round,
    // {
    //     Size3D::new(self.width.round(), self.height.round(), self.depth.round())
    // }

    // TODO:
    // /// Rounds each component to the smallest integer equal or greater than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::size3;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(size3::<_, Mm>(-0.1, -0.8, 0.4).ceil(), size3::<_, Mm>(0.0, 0.0, 1.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn ceil(self) -> Self
    // where
    //     T: Ceil,
    // {
    //     Size3D::new(self.width.ceil(), self.height.ceil(), self.depth.ceil())
    // }

    // TODO:
    // /// Rounds each component to the biggest integer equal or lower than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::size3;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(size3::<_, Mm>(-0.1, -0.8, 0.4).floor(), size3::<_, Mm>(-1.0, -1.0, 0.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn floor(self) -> Self
    // where
    //     T: Floor,
    // {
    //     Size3D::new(self.width.floor(), self.height.floor(), self.depth.floor())
    // }

    /// Returns result of multiplication of all components
    pub fn volume(self) -> T
    where
        T: Mul<Output = T>,
    {
        self.width * self.height * self.depth
    }

    // /// Linearly interpolate between this size and another size.
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// use euclid::size3;
    // /// use euclid::default::Size3D;
    // ///
    // /// let from: Size3D<_> = size3(0.0, 10.0, -1.0);
    // /// let to:  Size3D<_> = size3(8.0, -4.0,  0.0);
    // ///
    // /// assert_eq!(from.lerp(to, -1.0), size3(-8.0,  24.0, -2.0));
    // /// assert_eq!(from.lerp(to,  0.0), size3( 0.0,  10.0, -1.0));
    // /// assert_eq!(from.lerp(to,  0.5), size3( 4.0,   3.0, -0.5));
    // /// assert_eq!(from.lerp(to,  1.0), size3( 8.0,  -4.0,  0.0));
    // /// assert_eq!(from.lerp(to,  2.0), size3(16.0, -18.0,  1.0));
    // /// ```
    #[inline]
    pub fn lerp(self, other: Self, t: T) -> Self
    where
        T: One + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
    {
        let one_t = T::one() - t;
        self * one_t + other * t
    }
}

// TODO:
// impl<T: NumCast + Copy> Size3D<T> {
//     /// Cast from one numeric representation to another, preserving the units.
//     ///
//     /// When casting from floating point to integer coordinates, the decimals are truncated
//     /// as one would expect from a simple cast, but this behavior does not always make sense
//     /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
//     #[inline]
//     pub fn cast<NewT: NumCast>(self) -> Size3D<NewT> {
//         self.try_cast().unwrap()
//     }

//     /// Fallible cast from one numeric representation to another, preserving the units.
//     ///
//     /// When casting from floating point to integer coordinates, the decimals are truncated
//     /// as one would expect from a simple cast, but this behavior does not always make sense
//     /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
//     pub fn try_cast<NewT: NumCast>(self) -> Option<Size3D<NewT>> {
//         match (
//             NumCast::from(self.width),
//             NumCast::from(self.height),
//             NumCast::from(self.depth),
//         ) {
//             (Some(w), Some(h), Some(d)) => Some(Size3D::new(w, h, d)),
//             _ => None,
//         }
//     }

//     // Convenience functions for common casts

//     /// Cast into an `f32` size.
//     #[inline]
//     pub fn to_f32(self) -> Size3D<f32> {
//         self.cast()
//     }

//     /// Cast into an `f64` size.
//     #[inline]
//     pub fn to_f64(self) -> Size3D<f64> {
//         self.cast()
//     }

//     /// Cast into an `uint` size, truncating decimals if any.
//     ///
//     /// When casting from floating point sizes, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_usize(self) -> Size3D<usize> {
//         self.cast()
//     }

//     /// Cast into an `u32` size, truncating decimals if any.
//     ///
//     /// When casting from floating point sizes, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_u32(self) -> Size3D<u32> {
//         self.cast()
//     }

//     /// Cast into an `i32` size, truncating decimals if any.
//     ///
//     /// When casting from floating point sizes, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_i32(self) -> Size3D<i32> {
//         self.cast()
//     }

//     /// Cast into an `i64` size, truncating decimals if any.
//     ///
//     /// When casting from floating point sizes, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_i64(self) -> Size3D<i64> {
//         self.cast()
//     }
// }

// TODO:
// impl<T: Float> Size3D<T> {
//     /// Returns true if all members are finite.
//     #[inline]
//     pub fn is_finite(self) -> bool {
//         self.width.is_finite() && self.height.is_finite() && self.depth.is_finite()
//     }
// }

// TODO:
// impl<T: Signed> Size3D<T> {
//     /// Computes the absolute value of each component.
//     ///
//     /// For `f32` and `f64`, `NaN` will be returned for component if the component is `NaN`.
//     ///
//     /// For signed integers, `::MIN` will be returned for component if the component is `::MIN`.
//     pub fn abs(self) -> Self {
//         size3(self.width.abs(), self.height.abs(), self.depth.abs())
//     }

//     /// Returns `true` if all components is positive and `false` any component is zero or negative.
//     pub fn is_positive(self) -> bool {
//         self.width.is_positive() && self.height.is_positive() && self.depth.is_positive()
//     }
// }

// impl<T: PartialOrd> Size3D<T> {
//     /// Returns the size each component of which are minimum of this size and another.
//     #[inline]
//     pub fn min(self, other: Self) -> Self {
//         size3(
//             min(self.width, other.width),
//             min(self.height, other.height),
//             min(self.depth, other.depth),
//         )
//     }

//     /// Returns the size each component of which are maximum of this size and another.
//     #[inline]
//     pub fn max(self, other: Self) -> Self {
//         size3(
//             max(self.width, other.width),
//             max(self.height, other.height),
//             max(self.depth, other.depth),
//         )
//     }

//     /// Returns the size each component of which clamped by corresponding
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

//     // Returns true if this size is larger or equal to the other size in all dimensions.
//     #[inline]
//     pub fn contains(self, other: Self) -> bool {
//         self.width >= other.width && self.height >= other.height && self.depth >= other.depth
//     }


//     /// Returns vector with results of "greater than" operation on each component.
//     pub fn greater_than(self, other: Self) -> BoolVector3D {
//         BoolVector3D {
//             x: self.width > other.width,
//             y: self.height > other.height,
//             z: self.depth > other.depth,
//         }
//     }

//     /// Returns vector with results of "lower than" operation on each component.
//     pub fn lower_than(self, other: Self) -> BoolVector3D {
//         BoolVector3D {
//             x: self.width < other.width,
//             y: self.height < other.height,
//             z: self.depth < other.depth,
//         }
//     }

//     /// Returns `true` if any component of size is zero, negative or NaN.
//     pub fn is_empty(self) -> bool
//     where
//         T: Zero,
//     {
//         let zero = T::zero();
//         !(self.width > zero && self.height > zero && self.depth <= zero)
//     }
// }

// impl<T: PartialEq> Size3D<T> {
//     /// Returns vector with results of "equal" operation on each component.
//     pub fn equal(self, other: Self) -> BoolVector3D {
//         BoolVector3D {
//             x: self.width == other.width,
//             y: self.height == other.height,
//             z: self.depth == other.depth,
//         }
//     }

//     /// Returns vector with results of "not equal" operation on each component.
//     pub fn not_equal(self, other: Self) -> BoolVector3D {
//         BoolVector3D {
//             x: self.width != other.width,
//             y: self.height != other.height,
//             z: self.depth != other.depth,
//         }
//     }
// }

// impl<T: Round> Round for Size3D<T> {
//     /// See [`Size3D::round()`](#method.round).
//     #[inline]
//     fn round(self) -> Self {
//         self.round()
//     }
// }

// impl<T: Ceil> Ceil for Size3D<T> {
//     /// See [`Size3D::ceil()`](#method.ceil).
//     #[inline]
//     fn ceil(self) -> Self {
//         self.ceil()
//     }
// }

// impl<T: Floor> Floor for Size3D<T> {
//     /// See [`Size3D::floor()`](#method.floor).
//     #[inline]
//     fn floor(self) -> Self {
//         self.floor()
//     }
// }

impl<T: Zero> Zero for Size3D<T> {
    #[inline]
    fn zero() -> Self {
        Size3D::new(Zero::zero(), Zero::zero(), Zero::zero())
    }
}

impl<T: Neg> Neg for Size3D<T> {
    type Output = Size3D<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Size3D::new(-self.width, -self.height, -self.depth)
    }
}

impl<T: Add> Add for Size3D<T> {
    type Output = Size3D<T::Output>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Size3D::new(
            self.width + other.width,
            self.height + other.height,
            self.depth + other.depth,
        )
    }
}

impl<T: Copy + Add<T, Output = T>> Add<&Self> for Size3D<T> {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        Size3D::new(
            self.width + other.width,
            self.height + other.height,
            self.depth + other.depth,
        )
    }
}

impl<T: Add<Output = T> + Zero> Sum for Size3D<T> {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

// TODO:
// impl<'a, T: 'a + Add<Output = T> + Copy + Zero: 'a> Sum<&'a Self> for Size3D<T> {
//     fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
//         iter.fold(Self::zero(), Add::add)
//     }
// }

impl<T: AddAssign> AddAssign for Size3D<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.width += other.width;
        self.height += other.height;
        self.depth += other.depth;
    }
}

impl<T: Sub> Sub for Size3D<T> {
    type Output = Size3D<T::Output>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Size3D::new(
            self.width - other.width,
            self.height - other.height,
            self.depth - other.depth,
        )
    }
}

impl<T: SubAssign> SubAssign for Size3D<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.width -= other.width;
        self.height -= other.height;
        self.depth -= other.depth;
    }
}

impl<T: Copy + Mul> Mul<T> for Size3D<T> {
    type Output = Size3D<T::Output>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Size3D::new(
            self.width * scale,
            self.height * scale,
            self.depth * scale,
        )
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Size3D<T> {
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.width *= other;
        self.height *= other;
        self.depth *= other;
    }
}

// impl<T: Copy + Mul> Mul<Scale<T>> for Size3D<T1> {
//     type Output = Size3D<T::Output>;

//     #[inline]
//     fn mul(self, scale: Scale<T>) -> Self::Output {
//         Size3D::new(
//             self.width * scale.0,
//             self.height * scale.0,
//             self.depth * scale.0,
//         )
//     }
// }

// impl<T: Copy + MulAssign> MulAssign<Scale<T>> for Size3D<T> {
//     #[inline]
//     fn mul_assign(&mut self, other: Scale<T>) {
//         *self *= other.0;
//     }
// }

impl<T: Copy + Div> Div<T> for Size3D<T> {
    type Output = Size3D<T::Output>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Size3D::new(
            self.width / scale,
            self.height / scale,
            self.depth / scale,
        )
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Size3D<T> {
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.width /= other;
        self.height /= other;
        self.depth /= other;
    }
}

// impl<T: Copy + Div> Div<Scale<T>> for Size3D<T> {
//     type Output = Size3D<T::Output>;

//     #[inline]
//     fn div(self, scale: Scale<T>) -> Self::Output {
//         Size3D::new(
//             self.width / scale.0,
//             self.height / scale.0,
//             self.depth / scale.0,
//         )
//     }
// }

// impl<T: Copy + DivAssign> DivAssign<Scale<T>> for Size3D<T> {
//     #[inline]
//     fn div_assign(&mut self, other: Scale<T>) {
//         *self /= other.0;
//     }
// }

#[cfg(feature = "mint")]
impl<T> From<mint::Vector3<T>> for Size3D<T> {
    #[inline]
    fn from(v: mint::Vector3<T>) -> Self {
        size3(v.x, v.y, v.z)
    }
}
#[cfg(feature = "mint")]
impl<T> Into<mint::Vector3<T>> for Size3D<T> {
    #[inline]
    fn into(self) -> mint::Vector3<T> {
        mint::Vector3 {
            x: self.width,
            y: self.height,
            z: self.depth,
        }
    }
}

// impl<T> From<Vector3D<T>> for Size3D<T> {
//     #[inline]
//     fn from(v: Vector3D<T>) -> Self {
//         size3(v.x, v.y, v.z)
//     }
// }

impl<T> Into<[T; 3]> for Size3D<T> {
    #[inline]
    fn into(self) -> [T; 3] {
        [self.width, self.height, self.depth]
    }
}

impl<T> From<[T; 3]> for Size3D<T> {
    #[inline]
    fn from([w, h, d]: [T; 3]) -> Self {
        size3(w, h, d)
    }
}

impl<T> Into<(T, T, T)> for Size3D<T> {
    #[inline]
    fn into(self) -> (T, T, T) {
        (self.width, self.height, self.depth)
    }
}

impl<T> From<(T, T, T)> for Size3D<T> {
    #[inline]
    fn from(tuple: (T, T, T)) -> Self {
        size3(tuple.0, tuple.1, tuple.2)
    }
}

/// Shorthand for `Size3D::new(w, h, d)`.
#[inline]
pub const fn size3<T>(w: T, h: T, d: T) -> Size3D<T> {
    Size3D::new(w, h, d)
}

#[cfg(test)]
mod size3d {
    mod ops {
        use crate::Size3D;
        // use crate::scale::Scale;

        // pub enum Mm {}
        // pub enum Cm {}

        // pub type Size3DMm<T> = crate::Size3D<T, Mm>;
        // pub type Size3DCm<T> = crate::Size3D<T, Cm>;

        #[test]
        pub fn test_neg() {
            assert_eq!(-Size3D::new(1.0, 2.0, 3.0), Size3D::new(-1.0, -2.0, -3.0));
            assert_eq!(-Size3D::new(0.0, 0.0, 0.0), Size3D::new(-0.0, -0.0, -0.0));
            assert_eq!(-Size3D::new(-1.0, -2.0, -3.0), Size3D::new(1.0, 2.0, 3.0));
        }

        #[test]
        pub fn test_add() {
            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(4.0, 5.0, 6.0);
            assert_eq!(s1 + s2, Size3D::new(5.0, 7.0, 9.0));
            assert_eq!(s1 + &s2, Size3D::new(5.0, 7.0, 9.0));

            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s1 + s2, Size3D::new(1.0, 2.0, 3.0));
            assert_eq!(s1 + &s2, Size3D::new(1.0, 2.0, 3.0));

            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(-4.0, -5.0, -6.0);
            assert_eq!(s1 + s2, Size3D::new(-3.0, -3.0, -3.0));
            assert_eq!(s1 + &s2, Size3D::new(-3.0, -3.0, -3.0));

            let s1 = Size3D::new(0.0, 0.0, 0.0);
            let s2 = Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s1 + s2, Size3D::new(0.0, 0.0, 0.0));
            assert_eq!(s1 + &s2, Size3D::new(0.0, 0.0, 0.0));
        }

        // #[test]
        // pub fn test_sum() {
        //     let sizes = [
        //         Size3D::new(0.0, 1.0, 2.0),
        //         Size3D::new(1.0, 2.0, 3.0),
        //         Size3D::new(2.0, 3.0, 4.0)
        //     ];
        //     let sum = Size3D::new(3.0, 6.0, 9.0);
        //     assert_eq!(sizes.iter().sum::<Size3D<_>>(), sum);
        //     assert_eq!(sizes.into_iter().sum::<Size3D<_>>(), sum);
        // }

        #[test]
        pub fn test_add_assign() {
            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s += Size3D::new(4.0, 5.0, 6.0);
            assert_eq!(s, Size3D::new(5.0, 7.0, 9.0));

            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s += Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s, Size3D::new(1.0, 2.0, 3.0));

            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s += Size3D::new(-4.0, -5.0, -6.0);
            assert_eq!(s, Size3D::new(-3.0, -3.0, -3.0));

            let mut s = Size3D::new(0.0, 0.0, 0.0);
            s += Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s, Size3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_sub() {
            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(4.0, 5.0, 6.0);
            assert_eq!(s1 - s2, Size3D::new(-3.0, -3.0, -3.0));

            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s1 - s2, Size3D::new(1.0, 2.0, 3.0));

            let s1 = Size3D::new(1.0, 2.0, 3.0);
            let s2 = Size3D::new(-4.0, -5.0, -6.0);
            assert_eq!(s1 - s2, Size3D::new(5.0, 7.0, 9.0));

            let s1 = Size3D::new(0.0, 0.0, 0.0);
            let s2 = Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s1 - s2, Size3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_sub_assign() {
            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s -= Size3D::new(4.0, 5.0, 6.0);
            assert_eq!(s, Size3D::new(-3.0, -3.0, -3.0));

            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s -= Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s, Size3D::new(1.0, 2.0, 3.0));

            let mut s = Size3D::new(1.0, 2.0, 3.0);
            s -= Size3D::new(-4.0, -5.0, -6.0);
            assert_eq!(s, Size3D::new(5.0, 7.0, 9.0));

            let mut s = Size3D::new(0.0, 0.0, 0.0);
            s -= Size3D::new(0.0, 0.0, 0.0);
            assert_eq!(s, Size3D::new(0.0, 0.0, 0.0));
        }

        #[test]
        pub fn test_mul_scalar() {
            let s1: Size3D<f32> = Size3D::new(3.0, 5.0, 7.0);

            let result = s1 * 5.0;

            assert_eq!(result, Size3D::new(15.0, 25.0, 35.0));
        }

        #[test]
        pub fn test_mul_assign_scalar() {
            let mut s1: Size3D<f32> = Size3D::new(3.0, 5.0, 7.0);

            s1 *= 5.0;

            assert_eq!(s1, Size3D::new(15.0, 25.0, 35.0));
        }

        // #[test]
        // pub fn test_mul_scale() {
        //     let s1 = Size3DMm::new(1.0, 2.0, 3.0);
        //     let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

        //     let result = s1 * cm_per_mm;

        //     assert_eq!(result, Size3DCm::new(0.1, 0.2, 0.3));
        // }

        // #[test]
        // pub fn test_mul_assign_scale() {
        //     let mut s1 = Size3DMm::new(1.0, 2.0, 3.0);
        //     let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

        //     s1 *= scale;

        //     assert_eq!(s1, Size3DMm::new(0.1, 0.2, 0.3));
        // }

        #[test]
        pub fn test_div_scalar() {
            let s1: Size3D<f32> = Size3D::new(15.0, 25.0, 35.0);

            let result = s1 / 5.0;

            assert_eq!(result, Size3D::new(3.0, 5.0, 7.0));
        }

        #[test]
        pub fn test_div_assign_scalar() {
            let mut s1: Size3D<f32> = Size3D::new(15.0, 25.0, 35.0);

            s1 /= 5.0;

            assert_eq!(s1, Size3D::new(3.0, 5.0, 7.0));
        }

        // #[test]
        // pub fn test_div_scale() {
        //     let s1 = Size3DCm::new(0.1, 0.2, 0.3);
        //     let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

        //     let result = s1 / cm_per_mm;

        //     assert_eq!(result, Size3DMm::new(1.0, 2.0, 3.0));
        // }

        // #[test]
        // pub fn test_div_assign_scale() {
        //     let mut s1 = Size3DMm::new(0.1, 0.2, 0.3);
        //     let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

        //     s1 /= scale;

        //     assert_eq!(s1, Size3DMm::new(1.0, 2.0, 3.0));
        // }

        // #[test]
        // pub fn test_nan_empty() {
        //     use std::f32::NAN;
        //     assert!(Size3D::new(NAN, 2.0, 3.0).is_empty());
        //     assert!(Size3D::new(0.0, NAN, 0.0).is_empty());
        //     assert!(Size3D::new(1.0, 2.0, NAN).is_empty());
        // }
    }
}
