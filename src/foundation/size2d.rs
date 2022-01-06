use std::{
    cmp::{Eq, PartialEq},
    fmt,
    hash::Hash,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::prelude::{One, Zero};

/// A 2d size tagged with a unit.
#[repr(C)]
pub struct Size2D<T> {
    /// The extent of the element in the `U` units along the `x` axis (usually horizontal).
    pub width: T,
    /// The extent of the element in the `U` units along the `y` axis (usually vertical).
    pub height: T,
}

impl<T: Copy> Copy for Size2D<T> {}

impl<T: Clone> Clone for Size2D<T> {
    fn clone(&self) -> Self {
        Size2D {
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Size2D<T>
where
    T: serde::Deserialize<'de>,
{
    /// Deserializes 2d size from tuple of width and height.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (width, height) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Size2D {
            width,
            height,
            _unit: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Size2D<T>
where
    T: serde::Serialize,
{
    /// Serializes 2d size to tuple of width and height.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.width, &self.height).serialize(serializer)
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T> arbitrary::Arbitrary<'a> for Size2D<T>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let (width, height) = arbitrary::Arbitrary::arbitrary(u)?;
        Ok(Size2D {
            width,
            height,
            _unit: PhantomData,
        })
    }
}

impl<T> Eq for Size2D<T> where T: Eq {}

impl<T> PartialEq for Size2D<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl<T> Hash for Size2D<T>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.width.hash(h);
        self.height.hash(h);
    }
}

impl<T: fmt::Debug> fmt::Debug for Size2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.width, f)?;
        write!(f, "x")?;
        fmt::Debug::fmt(&self.height, f)
    }
}

impl<T: Default> Default for Size2D<T> {
    fn default() -> Self {
        Size2D::new(Default::default(), Default::default())
    }
}

impl<T> Size2D<T> {
    /// The same as [`Zero::zero()`] but available without importing trait.
    ///
    /// [`Zero::zero()`]: ./num/trait.Zero.html#tymethod.zero
    #[inline]
    pub fn zero() -> Self
    where
        T: Zero,
    {
        Size2D::new(Zero::zero(), Zero::zero())
    }

    /// Constructor taking scalar values.
    #[inline]
    pub const fn new(width: T, height: T) -> Self {
        Size2D { width, height }
    }

    // TODO:
    // /// Constructor taking scalar strongly typed lengths.
    // #[inline]
    // pub fn from_lengths(width: Length<T>, height: Length<T>) -> Self {
    //     Size2D::new(width.0, height.0)
    // }

    /// Constructor setting all components to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Size2D {
            width: v.clone(),
            height: v,
        }
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(p: Size2D<T>) -> Self {
        Size2D::new(p.width, p.height)
    }
}

impl<T: Copy> Size2D<T> {
    /// Return this size as an array of two elements (width, then height).
    #[inline]
    pub fn to_array(self) -> [T; 2] {
        [self.width, self.height]
    }

    /// Return this size as a tuple of two elements (width, then height).
    #[inline]
    pub fn to_tuple(self) -> (T, T) {
        (self.width, self.height)
    }

    // TODO:
    // /// Return this size as a vector with width and height.
    // #[inline]
    // pub fn to_vector(self) -> Vector2D<T> {
    //     vec2(self.width, self.height)
    // }

    // /// Drop the units, preserving only the numeric value.
    // #[inline]
    // pub fn to_untyped(self) -> Size2D<T> {
    //     self.cast_unit()
    // }

    // /// Cast the unit
    // #[inline]
    // pub fn cast_unit<V>(self) -> Size2D<T, V> {
    //     Size2D::new(self.width, self.height)
    // }

    // TODO:
    // /// Rounds each component to the nearest integer value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::size2;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(size2::<_, Mm>(-0.1, -0.8).round(), size2::<_, Mm>(0.0, -1.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn round(self) -> Self
    // where
    //     T: Round,
    // {
    //     Size2D::new(self.width.round(), self.height.round())
    // }

    // TODO:
    // /// Rounds each component to the smallest integer equal or greater than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::size2;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(size2::<_, Mm>(-0.1, -0.8).ceil(), size2::<_, Mm>(0.0, 0.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn ceil(self) -> Self
    // where
    //     T: Ceil,
    // {
    //     Size2D::new(self.width.ceil(), self.height.ceil())
    // }

    // TODO:
    // /// Rounds each component to the biggest integer equal or lower than the original value.
    // ///
    // /// This behavior is preserved for negative values (unlike the basic cast).
    // ///
    // /// ```rust
    // /// # use euclid::size2;
    // /// enum Mm {}
    // ///
    // /// assert_eq!(size2::<_, Mm>(-0.1, -0.8).floor(), size2::<_, Mm>(-1.0, -1.0))
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn floor(self) -> Self
    // where
    //     T: Floor,
    // {
    //     Size2D::new(self.width.floor(), self.height.floor())
    // }

    /// Returns result of multiplication of both components
    pub fn area(self) -> T::Output
    where
        T: Mul,
    {
        self.width * self.height
    }

    // /// Linearly interpolate each component between this size and another size.
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// use euclid::size2;
    // /// use euclid::default::Size2D;
    // ///
    // /// let from: Size2D<_> = size2(0.0, 10.0);
    // /// let to:  Size2D<_> = size2(8.0, -4.0);
    // ///
    // /// assert_eq!(from.lerp(to, -1.0), size2(-8.0,  24.0));
    // /// assert_eq!(from.lerp(to,  0.0), size2( 0.0,  10.0));
    // /// assert_eq!(from.lerp(to,  0.5), size2( 4.0,   3.0));
    // /// assert_eq!(from.lerp(to,  1.0), size2( 8.0,  -4.0));
    // /// assert_eq!(from.lerp(to,  2.0), size2(16.0, -18.0));
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

// impl<T: NumCast + Copy> Size2D<T> {
//     /// Cast from one numeric representation to another, preserving the units.
//     ///
//     /// When casting from floating point to integer coordinates, the decimals are truncated
//     /// as one would expect from a simple cast, but this behavior does not always make sense
//     /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
//     #[inline]
//     pub fn cast<NewT: NumCast>(self) -> Size2D<NewT> {
//         self.try_cast().unwrap()
//     }

//     /// Fallible cast from one numeric representation to another, preserving the units.
//     ///
//     /// When casting from floating point to integer coordinates, the decimals are truncated
//     /// as one would expect from a simple cast, but this behavior does not always make sense
//     /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
//     pub fn try_cast<NewT: NumCast>(self) -> Option<Size2D<NewT>> {
//         match (NumCast::from(self.width), NumCast::from(self.height)) {
//             (Some(w), Some(h)) => Some(Size2D::new(w, h)),
//             _ => None,
//         }
//     }

//     // Convenience functions for common casts

//     /// Cast into an `f32` size.
//     #[inline]
//     pub fn to_f32(self) -> Size2D<f32> {
//         self.cast()
//     }

//     /// Cast into an `f64` size.
//     #[inline]
//     pub fn to_f64(self) -> Size2D<f64> {
//         self.cast()
//     }

//     /// Cast into an `uint` size, truncating decimals if any.
//     ///
//     /// When casting from floating point sizes, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_usize(self) -> Size2D<usize> {
//         self.cast()
//     }

//     /// Cast into an `u32` size, truncating decimals if any.
//     ///
//     /// When casting from floating point sizes, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_u32(self) -> Size2D<u32> {
//         self.cast()
//     }

//     /// Cast into an `u64` size, truncating decimals if any.
//     ///
//     /// When casting from floating point sizes, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_u64(self) -> Size2D<u64> {
//         self.cast()
//     }

//     /// Cast into an `i32` size, truncating decimals if any.
//     ///
//     /// When casting from floating point sizes, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_i32(self) -> Size2D<i32> {
//         self.cast()
//     }

//     /// Cast into an `i64` size, truncating decimals if any.
//     ///
//     /// When casting from floating point sizes, it is worth considering whether
//     /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
//     /// the desired conversion behavior.
//     #[inline]
//     pub fn to_i64(self) -> Size2D<i64> {
//         self.cast()
//     }
// }

// impl<T: Float> Size2D<T> {
//     /// Returns true if all members are finite.
//     #[inline]
//     pub fn is_finite(self) -> bool {
//         self.width.is_finite() && self.height.is_finite()
//     }
// }

// impl<T: Signed> Size2D<T> {
//     /// Computes the absolute value of each component.
//     ///
//     /// For `f32` and `f64`, `NaN` will be returned for component if the component is `NaN`.
//     ///
//     /// For signed integers, `::MIN` will be returned for component if the component is `::MIN`.
//     pub fn abs(self) -> Self {
//         size2(self.width.abs(), self.height.abs())
//     }

//     /// Returns `true` if both components is positive and `false` any component is zero or negative.
//     pub fn is_positive(self) -> bool {
//         self.width.is_positive() && self.height.is_positive()
//     }
// }

// impl<T: PartialOrd> Size2D<T> {
//     /// Returns the size each component of which are minimum of this size and another.
//     #[inline]
//     pub fn min(self, other: Self) -> Self {
//         size2(min(self.width, other.width), min(self.height, other.height))
//     }

//     /// Returns the size each component of which are maximum of this size and another.
//     #[inline]
//     pub fn max(self, other: Self) -> Self {
//         size2(max(self.width, other.width), max(self.height, other.height))
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
//         self.width >= other.width && self.height >= other.height
//     }

//     /// Returns vector with results of "greater then" operation on each component.
//     pub fn greater_than(self, other: Self) -> BoolVector2D {
//         BoolVector2D {
//             x: self.width > other.width,
//             y: self.height > other.height,
//         }
//     }

//     /// Returns vector with results of "lower then" operation on each component.
//     pub fn lower_than(self, other: Self) -> BoolVector2D {
//         BoolVector2D {
//             x: self.width < other.width,
//             y: self.height < other.height,
//         }
//     }

//     /// Returns `true` if any component of size is zero, negative, or NaN.
//     pub fn is_empty(self) -> bool
//     where
//         T: Zero,
//     {
//         let zero = T::zero();
//         // The condition is experessed this way so that we return true in
//         // the presence of NaN.
//         !(self.width > zero && self.height > zero)
//     }
// }

// impl<T: PartialEq> Size2D<T> {
//     /// Returns vector with results of "equal" operation on each component.
//     pub fn equal(self, other: Self) -> BoolVector2D {
//         BoolVector2D {
//             x: self.width == other.width,
//             y: self.height == other.height,
//         }
//     }

//     /// Returns vector with results of "not equal" operation on each component.
//     pub fn not_equal(self, other: Self) -> BoolVector2D {
//         BoolVector2D {
//             x: self.width != other.width,
//             y: self.height != other.height,
//         }
//     }
// }

// impl<T: Round> Round for Size2D<T> {
//     /// See [`Size2D::round()`](#method.round).
//     #[inline]
//     fn round(self) -> Self {
//         self.round()
//     }
// }

// impl<T: Ceil> Ceil for Size2D<T> {
//     /// See [`Size2D::ceil()`](#method.ceil).
//     #[inline]
//     fn ceil(self) -> Self {
//         self.ceil()
//     }
// }

// impl<T: Floor> Floor for Size2D<T> {
//     /// See [`Size2D::floor()`](#method.floor).
//     #[inline]
//     fn floor(self) -> Self {
//         self.floor()
//     }
// }

impl<T: Zero> Zero for Size2D<T> {
    #[inline]
    fn zero() -> Self {
        Size2D::new(Zero::zero(), Zero::zero())
    }
}

impl<T: Neg> Neg for Size2D<T> {
    type Output = Size2D<T::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Size2D::new(-self.width, -self.height)
    }
}

impl<T: Add> Add for Size2D<T> {
    type Output = Size2D<T::Output>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Size2D::new(self.width + other.width, self.height + other.height)
    }
}

impl<T: Copy + Add<T, Output = T>> Add<&Self> for Size2D<T> {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        Size2D::new(self.width + other.width, self.height + other.height)
    }
}

impl<T: Add<Output = T> + Zero> Sum for Size2D<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

// TODO:
// impl<'a, T: 'a + Add<Output = T> + Copy + Zero: 'a> Sum<&'a Self> for Size2D<T> {
//     fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
//         iter.fold(Self::zero(), Add::add)
//     }
// }

impl<T: AddAssign> AddAssign for Size2D<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl<T: Sub> Sub for Size2D<T> {
    type Output = Size2D<T::Output>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Size2D::new(self.width - other.width, self.height - other.height)
    }
}

impl<T: SubAssign> SubAssign for Size2D<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.width -= other.width;
        self.height -= other.height;
    }
}

impl<T: Copy + Mul> Mul<T> for Size2D<T> {
    type Output = Size2D<T::Output>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Size2D::new(self.width * scale, self.height * scale)
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Size2D<T> {
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.width *= other;
        self.height *= other;
    }
}

// impl<T: Copy + Mul> Mul<Scale<T>> for Size2D<T> {
//     type Output = Size2D<T::Output>;

//     #[inline]
//     fn mul(self, scale: Scale<T>) -> Self::Output {
//         Size2D::new(self.width * scale.0, self.height * scale.0)
//     }
// }

// impl<T: Copy + MulAssign> MulAssign<Scale<T>> for Size2D<T> {
//     #[inline]
//     fn mul_assign(&mut self, other: Scale<T>) {
//         *self *= other.0;
//     }
// }

impl<T: Copy + Div> Div<T> for Size2D<T> {
    type Output = Size2D<T::Output>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Size2D::new(self.width / scale, self.height / scale)
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Size2D<T> {
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.width /= other;
        self.height /= other;
    }
}

// impl<T: Copy + Div> Div<Scale<T>> for Size2D<T> {
//     type Output = Size2D<T::Output>;

//     #[inline]
//     fn div(self, scale: Scale<T>) -> Self::Output {
//         Size2D::new(self.width / scale.0, self.height / scale.0)
//     }
// }

// impl<T: Copy + DivAssign> DivAssign<Scale<T>> for Size2D<T> {
//     #[inline]
//     fn div_assign(&mut self, other: Scale<T>) {
//         *self /= other.0;
//     }
// }

/// Shorthand for `Size2D::new(w, h)`.
#[inline]
pub const fn size2<T>(w: T, h: T) -> Size2D<T> {
    Size2D::new(w, h)
}

#[cfg(feature = "mint")]
impl<T> From<mint::Vector2<T>> for Size2D<T> {
    #[inline]
    fn from(v: mint::Vector2<T>) -> Self {
        Size2D {
            width: v.x,
            height: v.y,
            _unit: PhantomData,
        }
    }
}
#[cfg(feature = "mint")]
impl<T> Into<mint::Vector2<T>> for Size2D<T> {
    #[inline]
    fn into(self) -> mint::Vector2<T> {
        mint::Vector2 {
            x: self.width,
            y: self.height,
        }
    }
}

// impl<T> From<Vector2D<T>> for Size2D<T> {
//     #[inline]
//     fn from(v: Vector2D<T>) -> Self {
//         size2(v.x, v.y)
//     }
// }

impl<T> Into<[T; 2]> for Size2D<T> {
    #[inline]
    fn into(self) -> [T; 2] {
        [self.width, self.height]
    }
}

impl<T> From<[T; 2]> for Size2D<T> {
    #[inline]
    fn from([w, h]: [T; 2]) -> Self {
        size2(w, h)
    }
}

impl<T> Into<(T, T)> for Size2D<T> {
    #[inline]
    fn into(self) -> (T, T) {
        (self.width, self.height)
    }
}

impl<T> From<(T, T)> for Size2D<T> {
    #[inline]
    fn from(tuple: (T, T)) -> Self {
        size2(tuple.0, tuple.1)
    }
}

#[cfg(test)]
mod size2d {
    use crate::foundation::Size2D;
    #[cfg(feature = "mint")]
    use mint;

    #[test]
    pub fn test_area() {
        let p = Size2D::new(1.5, 2.0);
        assert_eq!(p.area(), 3.0);
    }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let s1 = Size2D::new(1.0, 2.0);
        let sm: mint::Vector2<_> = s1.into();
        let s2 = Size2D::from(sm);

        assert_eq!(s1, s2);
    }

    mod ops {
        use crate::foundation::Size2D;
        // use crate::scale::Scale;

        // pub enum Mm {}
        // pub enum Cm {}

        // pub type Size2DMm<T> = crate::Size2D<T, Mm>;
        // pub type Size2DCm<T> = crate::Size2D<T, Cm>;

        #[test]
        pub fn test_neg() {
            assert_eq!(-Size2D::new(1.0, 2.0), Size2D::new(-1.0, -2.0));
            assert_eq!(-Size2D::new(0.0, 0.0), Size2D::new(-0.0, -0.0));
            assert_eq!(-Size2D::new(-1.0, -2.0), Size2D::new(1.0, 2.0));
        }

        #[test]
        pub fn test_add() {
            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(3.0, 4.0);
            assert_eq!(s1 + s2, Size2D::new(4.0, 6.0));
            assert_eq!(s1 + &s2, Size2D::new(4.0, 6.0));

            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(0.0, 0.0);
            assert_eq!(s1 + s2, Size2D::new(1.0, 2.0));
            assert_eq!(s1 + &s2, Size2D::new(1.0, 2.0));

            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(-3.0, -4.0);
            assert_eq!(s1 + s2, Size2D::new(-2.0, -2.0));
            assert_eq!(s1 + &s2, Size2D::new(-2.0, -2.0));

            let s1 = Size2D::new(0.0, 0.0);
            let s2 = Size2D::new(0.0, 0.0);
            assert_eq!(s1 + s2, Size2D::new(0.0, 0.0));
            assert_eq!(s1 + &s2, Size2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_add_assign() {
            let mut s = Size2D::new(1.0, 2.0);
            s += Size2D::new(3.0, 4.0);
            assert_eq!(s, Size2D::new(4.0, 6.0));

            let mut s = Size2D::new(1.0, 2.0);
            s += Size2D::new(0.0, 0.0);
            assert_eq!(s, Size2D::new(1.0, 2.0));

            let mut s = Size2D::new(1.0, 2.0);
            s += Size2D::new(-3.0, -4.0);
            assert_eq!(s, Size2D::new(-2.0, -2.0));

            let mut s = Size2D::new(0.0, 0.0);
            s += Size2D::new(0.0, 0.0);
            assert_eq!(s, Size2D::new(0.0, 0.0));
        }

        // #[test]
        // pub fn test_sum() {
        //     let sizes = [
        //         Size2D::new(0.0, 1.0),
        //         Size2D::new(1.0, 2.0),
        //         Size2D::new(2.0, 3.0)
        //     ];
        //     let sum = Size2D::new(3.0, 6.0);
        //     assert_eq!(sizes.iter().sum::<Size2D<_>>(), sum);
        //     assert_eq!(sizes.into_iter().sum::<Size2D<_>>(), sum);
        // }

        #[test]
        pub fn test_sub() {
            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(3.0, 4.0);
            assert_eq!(s1 - s2, Size2D::new(-2.0, -2.0));

            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(0.0, 0.0);
            assert_eq!(s1 - s2, Size2D::new(1.0, 2.0));

            let s1 = Size2D::new(1.0, 2.0);
            let s2 = Size2D::new(-3.0, -4.0);
            assert_eq!(s1 - s2, Size2D::new(4.0, 6.0));

            let s1 = Size2D::new(0.0, 0.0);
            let s2 = Size2D::new(0.0, 0.0);
            assert_eq!(s1 - s2, Size2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_sub_assign() {
            let mut s = Size2D::new(1.0, 2.0);
            s -= Size2D::new(3.0, 4.0);
            assert_eq!(s, Size2D::new(-2.0, -2.0));

            let mut s = Size2D::new(1.0, 2.0);
            s -= Size2D::new(0.0, 0.0);
            assert_eq!(s, Size2D::new(1.0, 2.0));

            let mut s = Size2D::new(1.0, 2.0);
            s -= Size2D::new(-3.0, -4.0);
            assert_eq!(s, Size2D::new(4.0, 6.0));

            let mut s = Size2D::new(0.0, 0.0);
            s -= Size2D::new(0.0, 0.0);
            assert_eq!(s, Size2D::new(0.0, 0.0));
        }

        #[test]
        pub fn test_mul_scalar() {
            let s1: Size2D<f32> = Size2D::new(3.0, 5.0);

            let result = s1 * 5.0;

            assert_eq!(result, Size2D::new(15.0, 25.0));
        }

        #[test]
        pub fn test_mul_assign_scalar() {
            let mut s1 = Size2D::new(3.0, 5.0);

            s1 *= 5.0;

            assert_eq!(s1, Size2D::new(15.0, 25.0));
        }

        // #[test]
        // pub fn test_mul_scale() {
        //     let s1 = Size2DMm::new(1.0, 2.0);
        //     let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

        //     let result = s1 * cm_per_mm;

        //     assert_eq!(result, Size2DCm::new(0.1, 0.2));
        // }

        // #[test]
        // pub fn test_mul_assign_scale() {
        //     let mut s1 = Size2DMm::new(1.0, 2.0);
        //     let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

        //     s1 *= scale;

        //     assert_eq!(s1, Size2DMm::new(0.1, 0.2));
        // }

        #[test]
        pub fn test_div_scalar() {
            let s1: Size2D<f32> = Size2D::new(15.0, 25.0);

            let result = s1 / 5.0;

            assert_eq!(result, Size2D::new(3.0, 5.0));
        }

        #[test]
        pub fn test_div_assign_scalar() {
            let mut s1: Size2D<f32> = Size2D::new(15.0, 25.0);

            s1 /= 5.0;

            assert_eq!(s1, Size2D::new(3.0, 5.0));
        }

        // #[test]
        // pub fn test_div_scale() {
        //     let s1 = Size2DCm::new(0.1, 0.2);
        //     let cm_per_mm: Scale<f32, Mm, Cm> = Scale::new(0.1);

        //     let result = s1 / cm_per_mm;

        //     assert_eq!(result, Size2DMm::new(1.0, 2.0));
        // }

        // #[test]
        // pub fn test_div_assign_scale() {
        //     let mut s1 = Size2DMm::new(0.1, 0.2);
        //     let scale: Scale<f32, Mm, Mm> = Scale::new(0.1);

        //     s1 /= scale;

        //     assert_eq!(s1, Size2DMm::new(1.0, 2.0));
        // }

        // #[test]
        // pub fn test_nan_empty() {
        //     use std::f32::NAN;
        //     assert!(Size2D::new(NAN, 2.0).is_empty());
        //     assert!(Size2D::new(0.0, NAN).is_empty());
        //     assert!(Size2D::new(NAN, -2.0).is_empty());
        // }
    }
}
