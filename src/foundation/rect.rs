#![allow(unused_imports)]
use std::{
    borrow::Borrow,
    cmp::PartialOrd,
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, Div, DivAssign, Mul, MulAssign, Range, Sub},
};

use crate::prelude::{One, Zero};

use super::{Box2D, Point, Size};

/// A 2d Rectangle optionally tagged with a unit.
///
/// # Representation
///
/// `Rect` is represented by an origin point and a size.
///
/// See [`Box2D`] for a rectangle represented by two endpoints.
///
/// # Empty rectangle
///
/// A rectangle is considered empty (see [`is_empty`]) if any of the following is true:
/// - it's area is empty,
/// - it's area is negative (`size.x < 0` or `size.y < 0`),
/// - it contains NaNs.
///
/// [`is_empty`]: #method.is_empty
/// [`Box2D`]: struct.Box2D.html
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
)]
pub struct Rect<T> {
    /// Origin of rectangle
    pub origin: Point<T>,
    /// Size of rectangle
    pub size: Size<T>,
}

impl<T: Hash> Hash for Rect<T> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.origin.hash(h);
        self.size.hash(h);
    }
}

impl<T: Copy> Copy for Rect<T> {}

impl<T: Clone> Clone for Rect<T> {
    fn clone(&self) -> Self {
        Self::new(self.origin.clone(), self.size.clone())
    }
}

impl<T: PartialEq> PartialEq for Rect<T> {
    fn eq(&self, other: &Self) -> bool {
        self.origin.eq(&other.origin) && self.size.eq(&other.size)
    }
}

impl<T: Eq> Eq for Rect<T> {}

impl<T: fmt::Debug> fmt::Debug for Rect<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect(")?;
        fmt::Debug::fmt(&self.size, f)?;
        write!(f, " at ")?;
        fmt::Debug::fmt(&self.origin, f)?;
        write!(f, ")")
    }
}

impl<T: Default> Default for Rect<T> {
    fn default() -> Self {
        Rect::new(Default::default(), Default::default())
    }
}

impl<T> Rect<T> {
    /// Constructor.
    #[inline]
    pub const fn new(origin: Point<T>, size: Size<T>) -> Self {
        Rect { origin, size }
    }
}

// impl<T> Rect<T>
// where
//     T: Zero,
// {
//     /// Constructor, setting all sides to zero.
//     #[inline]
//     pub fn zero() -> Self {
//         Rect::new(Point::origin(), Size::zero())
//     }

//     /// Creates a rect of the given size, at offset zero.
//     #[inline]
//     pub fn from_size(size: Size<T>) -> Self {
//         Rect {
//             origin: Point::zero(),
//             size,
//         }
//     }
// }

impl<T> Rect<T>
where
    T: Copy + Add<T, Output = T>,
{
    /// Retrieve origin of rectangle
    #[inline]
    pub fn min(&self) -> Point<T> {
        self.origin
    }

    // #[inline]
    // pub fn max(&self) -> Point<T> {
    //     self.origin + self.size
    // }

    /// Retrieve maximum x of rectangle
    #[inline]
    pub fn max_x(&self) -> T {
        self.origin.x + self.size.width
    }

    /// Retrieve minimum x of rectangle
    #[inline]
    pub fn min_x(&self) -> T {
        self.origin.x
    }

    /// Retrieve maximum y of rectangle
    #[inline]
    pub fn max_y(&self) -> T {
        self.origin.y + self.size.height
    }

    /// Retrieve minimum y of rectangle
    #[inline]
    pub fn min_y(&self) -> T {
        self.origin.y
    }

    /// Retrieve rectangle width
    #[inline]
    pub fn width(&self) -> T {
        self.size.width
    }

    /// Retrieve rectangle height
    #[inline]
    pub fn height(&self) -> T {
        self.size.height
    }

    /// Retrieve x range of rectangle
    #[inline]
    pub fn x_range(&self) -> Range<T> {
        self.min_x()..self.max_x()
    }

    /// Retrieve y range of rectangle
    #[inline]
    pub fn y_range(&self) -> Range<T> {
        self.min_y()..self.max_y()
    }

    // /// Returns the same rectangle, translated by a vector.
    // #[inline]
    // #[must_use]
    // pub fn translate(&self, by: Vector<T>) -> Self {
    //     Self::new(self.origin + by, self.size)
    // }

    // #[inline]
    // pub fn to_box2d(&self) -> Box2D<T> {
    //     Box2D {
    //         min: self.min(),
    //         max: self.max(),
    //     }
    // }
}

// impl<T> Rect<T>
// where
//     T: Copy + PartialOrd + Add<T, Output = T>,
// {
//     /// Returns true if this rectangle contains the point. Points are considered
//     /// in the rectangle if they are on the left or top edge, but outside if they
//     /// are on the right or bottom edge.
//     #[inline]
//     pub fn contains(&self, p: Point<T>) -> bool {
//         self.to_box2d().contains(p)
//     }

//     #[inline]
//     pub fn intersects(&self, other: &Self) -> bool {
//         self.to_box2d().intersects(&other.to_box2d())
//     }
// }

// impl<T> Rect<T>
// where
//     T: Copy + PartialOrd + Add<T, Output = T> + Sub<T, Output = T>,
// {
//     #[inline]
//     pub fn intersection(&self, other: &Self) -> Option<Self> {
//         let box2d = self.to_box2d().intersection_unchecked(&other.to_box2d());

//         if box2d.is_empty() {
//             return None;
//         }

//         Some(box2d.to_rect())
//     }
// }

impl<T> Rect<T>
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T>,
{
    /// Create inflated rectangle
    #[inline]
    #[must_use]
    pub fn inflate(&self, width: T, height: T) -> Self {
        Rect::new(
            Point::new(self.origin.x - width, self.origin.y - height),
            Size::new(
                self.size.width + width + width,
                self.size.height + height + height,
            ),
        )
    }
}

// impl<T> Rect<T>
// where
//     T: Copy + Zero + PartialOrd + Add<T, Output = T>,
// {
//     /// Returns true if this rectangle contains the interior of rect. Always
//     /// returns true if rect is empty, and always returns false if rect is
//     /// nonempty but this rectangle is empty.
//     #[inline]
//     pub fn contains_rect(&self, rect: &Self) -> bool {
//         rect.is_empty()
//             || (self.min_x() <= rect.min_x()
//                 && rect.max_x() <= self.max_x()
//                 && self.min_y() <= rect.min_y()
//                 && rect.max_y() <= self.max_y())
//     }
// }

// TODO:
// impl<T> Rect<T>
// where
//     T: Copy + Zero + PartialOrd + Add<T, Output = T> + Sub<T, Output = T>,
// {
//     /// Calculate the size and position of an inner rectangle.
//     ///
//     /// Subtracts the side offsets from all sides. The horizontal and vertical
//     /// offsets must not be larger than the original side length.
//     /// This method assumes y oriented downward.
//     pub fn inner_rect(&self, offsets: SideOffsets2D<T, U>) -> Self {
//         let rect = Rect::new(
//             Point::new(self.origin.x + offsets.left, self.origin.y + offsets.top),
//             Size::new(
//                 self.size.width - offsets.horizontal(),
//                 self.size.height - offsets.vertical(),
//             ),
//         );
//         debug_assert!(rect.size.width >= Zero::zero());
//         debug_assert!(rect.size.height >= Zero::zero());
//         rect
//     }
// }

// impl<T> Rect<T>
// where
//     T: Copy + Add<T, Output = T> + Sub<T, Output = T>,
// {
//     /// Calculate the size and position of an outer rectangle.
//     ///
//     /// Add the offsets to all sides. The expanded rectangle is returned.
//     /// This method assumes y oriented downward.
//     pub fn outer_rect(&self, offsets: SideOffsets2D<T, U>) -> Self {
//         Rect::new(
//             Point::new(self.origin.x - offsets.left, self.origin.y - offsets.top),
//             Size::new(
//                 self.size.width + offsets.horizontal(),
//                 self.size.height + offsets.vertical(),
//             ),
//         )
//     }
// }

// impl<T> Rect<T>
// where
//     T: Copy + Zero + PartialOrd + Sub<T, Output = T>,
// {
//     /// Returns the smallest rectangle defined by the top/bottom/left/right-most
//     /// points provided as parameter.
//     ///
//     /// Note: This function has a behavior that can be surprising because
//     /// the right-most and bottom-most points are exactly on the edge
//     /// of the rectangle while the `contains` function is has exclusive
//     /// semantic on these edges. This means that the right-most and bottom-most
//     /// points provided to `from_points` will count as not contained by the rect.
//     /// This behavior may change in the future.
//     pub fn from_points<I>(points: I) -> Self
//     where
//         I: IntoIterator,
//         I::Item: Borrow<Point<T>>,
//     {
//         Box2D::from_points(points).to_rect()
//     }
// }

// impl<T> Rect<T>
// where
//     T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
// {
//     /// Linearly interpolate between this rectangle and another rectangle.
//     #[inline]
//     pub fn lerp(&self, other: Self, t: T) -> Self {
//         Self::new(
//             self.origin.lerp(other.origin, t),
//             self.size.lerp(other.size, t),
//         )
//     }
// }

// impl<T> Rect<T>
// where
//     T: Copy + One + Add<Output = T> + Div<Output = T>,
// {
//     pub fn center(&self) -> Point<T> {
//         let two = T::one() + T::one();
//         self.origin + self.size.to_vector() / two
//     }
// }

// impl<T> Rect<T>
// where
//     T: Copy + PartialOrd + Add<T, Output = T> + Sub<T, Output = T> + Zero,
// {
//     #[inline]
//     pub fn union(&self, other: &Self) -> Self {
//         if self.size == Zero::zero() {
//             return *other;
//         }
//         if other.size == Zero::zero() {
//             return *self;
//         }

//         self.to_box2d().union(&other.to_box2d()).to_rect()
//     }
// }

impl<T> Rect<T> {
    /// Create scaled rectangle
    #[inline]
    pub fn scale<S: Copy>(&self, x: S, y: S) -> Self
    where
        T: Copy + Mul<S, Output = T>,
    {
        Rect::new(
            Point::new(self.origin.x * x, self.origin.y * y),
            Size::new(self.size.width * x, self.size.height * y),
        )
    }
}

impl<T: Copy + Mul<T, Output = T>> Rect<T> {
    /// Retrieve area of rectangle
    #[inline]
    pub fn area(&self) -> T {
        self.size.area()
    }
}

// impl<T: Copy + Zero + PartialOrd> Rect<T> {
//     #[inline]
//     pub fn is_empty(&self) -> bool {
//         self.size.is_empty()
//     }
// }

// impl<T: Copy + Zero + PartialOrd> Rect<T> {
//     #[inline]
//     pub fn to_non_empty(&self) -> Option<Self> {
//         if self.is_empty() {
//             return None;
//         }

//         Some(*self)
//     }
// }

// impl<T: Copy + Mul> Mul<T> for Rect<T> {
//     type Output = Rect<T::Output>;

//     #[inline]
//     fn mul(self, scale: T) -> Self::Output {
//         Rect::new(self.origin * scale, self.size * scale)
//     }
// }

// TODO:
// impl<T: Copy + MulAssign> MulAssign<T> for Rect<T> {
//     #[inline]
//     fn mul_assign(&mut self, scale: T) {
//         *self *= Scale::new(scale);
//     }
// }

// impl<T: Copy + Div> Div<T> for Rect<T> {
//     type Output = Rect<T::Output>;

//     #[inline]
//     fn div(self, scale: T) -> Self::Output {
//         Rect::new(self.origin / scale.clone(), self.size / scale)
//     }
// }

// impl<T: Copy + DivAssign> DivAssign<T> for Rect<T> {
//     #[inline]
//     fn div_assign(&mut self, scale: T) {
//         *self /= Scale::new(scale);
//     }
// }

// impl<T: Copy + Mul> Mul<Scale<T>> for Rect<T> {
//     type Output = Rect<T::Output>;

//     #[inline]
//     fn mul(self, scale: Scale<T>) -> Self::Output {
//         Rect::new(self.origin * scale.clone(), self.size * scale)
//     }
// }

// impl<T: Copy + MulAssign> MulAssign<Scale<T>> for Rect<T> {
//     #[inline]
//     fn mul_assign(&mut self, scale: Scale<T>) {
//         self.origin *= scale.clone();
//         self.size *= scale;
//     }
// }

// impl<T: Copy + Div> Div<Scale<T>> for Rect<T> {
//     type Output = Rect<T::Output, U1>;

//     #[inline]
//     fn div(self, scale: Scale<T>) -> Self::Output {
//         Rect::new(self.origin / scale.clone(), self.size / scale)
//     }
// }

// impl<T: Copy + DivAssign> DivAssign<Scale<T>> for Rect<T> {
//     #[inline]
//     fn div_assign(&mut self, scale: Scale<T>) {
//         self.origin /= scale.clone();
//         self.size /= scale;
//     }
// }

// impl<T: Copy> Rect<T> {
//     /// Drop the units, preserving only the numeric value.
//     #[inline]
//     pub fn to_untyped(&self) -> Rect<T> {
//         Rect::new(self.origin.to_untyped(), self.size.to_untyped())
//     }

//     /// Tag a unitless value with units.
//     #[inline]
//     pub fn from_untyped(r: &Rect<T>) -> Rect<T> {
//         Rect::new(Point::from_untyped(r.origin), Size::from_untyped(r.size))
//     }

//     /// Cast the unit
//     #[inline]
//     pub fn cast_unit<V>(&self) -> Rect<T> {
//         Rect::new(self.origin.cast_unit(), self.size.cast_unit())
//     }
// }

// impl<T: Floor + Ceil + Round + Add<T, Output = T> + Sub<T, Output = T>> Rect<T> {
//     /// Return a rectangle with edges rounded to integer coordinates, such that
//     /// the returned rectangle has the same set of pixel centers as the original
//     /// one.
//     /// Edges at offset 0.5 round up.
//     /// Suitable for most places where integral device coordinates
//     /// are needed, but note that any translation should be applied first to
//     /// avoid pixel rounding errors.
//     /// Note that this is *not* rounding to nearest integer if the values are negative.
//     /// They are always rounding as floor(n + 0.5).
//     ///
//     /// # Usage notes
//     /// Note, that when using with floating-point `T` types that method can significantly
//     /// loose precision for large values, so if you need to call this method very often it
//     /// is better to use [`Box2D`].
//     ///
//     /// [`Box2D`]: struct.Box2D.html
//     #[must_use]
//     pub fn round(&self) -> Self {
//         self.to_box2d().round().to_rect()
//     }

//     /// Return a rectangle with edges rounded to integer coordinates, such that
//     /// the original rectangle contains the resulting rectangle.
//     ///
//     /// # Usage notes
//     /// Note, that when using with floating-point `T` types that method can significantly
//     /// loose precision for large values, so if you need to call this method very often it
//     /// is better to use [`Box2D`].
//     ///
//     /// [`Box2D`]: struct.Box2D.html
//     #[must_use]
//     pub fn round_in(&self) -> Self {
//         self.to_box2d().round_in().to_rect()
//     }

//     /// Return a rectangle with edges rounded to integer coordinates, such that
//     /// the original rectangle is contained in the resulting rectangle.
//     ///
//     /// # Usage notes
//     /// Note, that when using with floating-point `T` types that method can significantly
//     /// loose precision for large values, so if you need to call this method very often it
//     /// is better to use [`Box2D`].
//     ///
//     /// [`Box2D`]: struct.Box2D.html
//     #[must_use]
//     pub fn round_out(&self) -> Self {
//         self.to_box2d().round_out().to_rect()
//     }
// }

// impl<T> From<Size<T>> for Rect<T>
// where
//     T: Zero,
// {
//     fn from(size: Size<T>) -> Self {
//         Self::from_size(size)
//     }
// }

/// Shorthand for `Rect::new(Point::new(x, y), Size::new(w, h))`.
pub const fn rect<T>(x: T, y: T, w: T, h: T) -> Rect<T> {
    Rect::new(Point::new(x, y), Size::new(w, h))
}

#[cfg(test)]
mod tests {
    use crate::foundation::{Point, Rect, Size};
    // use crate::default::{Point, Rect, Size};
    // use crate::side_offsets::SideOffsets2D;
    // use crate::{point2, rect, size2, vec2};

    // #[test]
    // fn test_translate() {
    //     let p = Rect::new(Point::new(0u32, 0u32), Size::new(50u32, 40u32));
    //     let pp = p.translate(vec2(10, 15));

    //     assert!(pp.size.width == 50);
    //     assert!(pp.size.height == 40);
    //     assert!(pp.origin.x == 10);
    //     assert!(pp.origin.y == 15);

    //     let r = Rect::new(Point::new(-10, -5), Size::new(50, 40));
    //     let rr = r.translate(vec2(0, -10));

    //     assert!(rr.size.width == 50);
    //     assert!(rr.size.height == 40);
    //     assert!(rr.origin.x == -10);
    //     assert!(rr.origin.y == -15);
    // }

    // #[test]
    // fn test_union() {
    //     let p = Rect::new(Point::new(0, 0), Size::new(50, 40));
    //     let q = Rect::new(Point::new(20, 20), Size::new(5, 5));
    //     let r = Rect::new(Point::new(-15, -30), Size::new(200, 15));
    //     let s = Rect::new(Point::new(20, -15), Size::new(250, 200));

    //     let pq = p.union(&q);
    //     assert!(pq.origin == Point::new(0, 0));
    //     assert!(pq.size == Size::new(50, 40));

    //     let pr = p.union(&r);
    //     assert!(pr.origin == Point::new(-15, -30));
    //     assert!(pr.size == Size::new(200, 70));

    //     let ps = p.union(&s);
    //     assert!(ps.origin == Point::new(0, -15));
    //     assert!(ps.size == Size::new(270, 200));
    // }

    // #[test]
    // fn test_intersection() {
    //     let p = Rect::new(Point::new(0, 0), Size::new(10, 20));
    //     let q = Rect::new(Point::new(5, 15), Size::new(10, 10));
    //     let r = Rect::new(Point::new(-5, -5), Size::new(8, 8));

    //     let pq = p.intersection(&q);
    //     assert!(pq.is_some());
    //     let pq = pq.unwrap();
    //     assert!(pq.origin == Point::new(5, 15));
    //     assert!(pq.size == Size::new(5, 5));

    //     let pr = p.intersection(&r);
    //     assert!(pr.is_some());
    //     let pr = pr.unwrap();
    //     assert!(pr.origin == Point::new(0, 0));
    //     assert!(pr.size == Size::new(3, 3));

    //     let qr = q.intersection(&r);
    //     assert!(qr.is_none());
    // }

    // #[test]
    // fn test_intersection_overflow() {
    //     // test some scenarios where the intersection can overflow but
    //     // the min_x() and max_x() don't. Gecko currently fails these cases
    //     let p = Rect::new(Point::new(-2147483648, -2147483648), Size::new(0, 0));
    //     let q = Rect::new(
    //         Point::new(2136893440, 2136893440),
    //         Size::new(279552, 279552),
    //     );
    //     let r = Rect::new(Point::new(-2147483648, -2147483648), Size::new(1, 1));

    //     assert!(p.is_empty());
    //     let pq = p.intersection(&q);
    //     assert!(pq.is_none());

    //     let qr = q.intersection(&r);
    //     assert!(qr.is_none());
    // }

    // #[test]
    // fn test_contains() {
    //     let r = Rect::new(Point::new(-20, 15), Size::new(100, 200));

    //     assert!(r.contains(Point::new(0, 50)));
    //     assert!(r.contains(Point::new(-10, 200)));

    //     // The `contains` method is inclusive of the top/left edges, but not the
    //     // bottom/right edges.
    //     assert!(r.contains(Point::new(-20, 15)));
    //     assert!(!r.contains(Point::new(80, 15)));
    //     assert!(!r.contains(Point::new(80, 215)));
    //     assert!(!r.contains(Point::new(-20, 215)));

    //     // Points beyond the top-left corner.
    //     assert!(!r.contains(Point::new(-25, 15)));
    //     assert!(!r.contains(Point::new(-15, 10)));

    //     // Points beyond the top-right corner.
    //     assert!(!r.contains(Point::new(85, 20)));
    //     assert!(!r.contains(Point::new(75, 10)));

    //     // Points beyond the bottom-right corner.
    //     assert!(!r.contains(Point::new(85, 210)));
    //     assert!(!r.contains(Point::new(75, 220)));

    //     // Points beyond the bottom-left corner.
    //     assert!(!r.contains(Point::new(-25, 210)));
    //     assert!(!r.contains(Point::new(-15, 220)));

    //     let r = Rect::new(Point::new(-20.0, 15.0), Size::new(100.0, 200.0));
    //     assert!(r.contains_rect(&r));
    //     assert!(!r.contains_rect(&r.translate(vec2(0.1, 0.0))));
    //     assert!(!r.contains_rect(&r.translate(vec2(-0.1, 0.0))));
    //     assert!(!r.contains_rect(&r.translate(vec2(0.0, 0.1))));
    //     assert!(!r.contains_rect(&r.translate(vec2(0.0, -0.1))));
    //     // Empty rectangles are always considered as contained in other rectangles,
    //     // even if their origin is not.
    //     let p = Point::new(1.0, 1.0);
    //     assert!(!r.contains(p));
    //     assert!(r.contains_rect(&Rect::new(p, Size::zero())));
    // }

    #[test]
    fn test_scale() {
        let p = Rect::new(Point::new(0u32, 0u32), Size::new(50u32, 40u32));
        let pp = p.scale(10, 15);

        assert!(pp.size.width == 500);
        assert!(pp.size.height == 600);
        assert!(pp.origin.x == 0);
        assert!(pp.origin.y == 0);

        let r = Rect::new(Point::new(-10, -5), Size::new(50, 40));
        let rr = r.scale(1, 20);

        assert!(rr.size.width == 50);
        assert!(rr.size.height == 800);
        assert!(rr.origin.x == -10);
        assert!(rr.origin.y == -100);
    }

    #[test]
    fn test_inflate() {
        let p = Rect::new(Point::new(0, 0), Size::new(10, 10));
        let pp = p.inflate(10, 20);

        assert!(pp.size.width == 30);
        assert!(pp.size.height == 50);
        assert!(pp.origin.x == -10);
        assert!(pp.origin.y == -20);

        let r = Rect::new(Point::new(0, 0), Size::new(10, 20));
        let rr = r.inflate(-2, -5);

        assert!(rr.size.width == 6);
        assert!(rr.size.height == 10);
        assert!(rr.origin.x == 2);
        assert!(rr.origin.y == 5);
    }

    // #[test]
    // fn test_inner_outer_rect() {
    //     let inner_rect = Rect::new(point2(20, 40), size2(80, 100));
    //     let offsets = SideOffsets2D::new(20, 10, 10, 10);
    //     let outer_rect = inner_rect.outer_rect(offsets);
    //     assert_eq!(outer_rect.origin.x, 10);
    //     assert_eq!(outer_rect.origin.y, 20);
    //     assert_eq!(outer_rect.size.width, 100);
    //     assert_eq!(outer_rect.size.height, 130);
    //     assert_eq!(outer_rect.inner_rect(offsets), inner_rect);
    // }

    #[test]
    fn test_min_max_x_y() {
        let p = Rect::new(Point::new(0u32, 0u32), Size::new(50u32, 40u32));
        assert!(p.max_y() == 40);
        assert!(p.min_y() == 0);
        assert!(p.max_x() == 50);
        assert!(p.min_x() == 0);

        let r = Rect::new(Point::new(-10, -5), Size::new(50, 40));
        assert!(r.max_y() == 35);
        assert!(r.min_y() == -5);
        assert!(r.max_x() == 40);
        assert!(r.min_x() == -10);
    }

    #[test]
    fn test_width_height() {
        let r = Rect::new(Point::new(-10, -5), Size::new(50, 40));
        assert!(r.width() == 50);
        assert!(r.height() == 40);
    }

    // #[test]
    // fn test_is_empty() {
    //     assert!(Rect::new(Point::new(0u32, 0u32), Size::new(0u32, 0u32)).is_empty());
    //     assert!(Rect::new(Point::new(0u32, 0u32), Size::new(10u32, 0u32)).is_empty());
    //     assert!(Rect::new(Point::new(0u32, 0u32), Size::new(0u32, 10u32)).is_empty());
    //     assert!(!Rect::new(Point::new(0u32, 0u32), Size::new(1u32, 1u32)).is_empty());
    //     assert!(Rect::new(Point::new(10u32, 10u32), Size::new(0u32, 0u32)).is_empty());
    //     assert!(Rect::new(Point::new(10u32, 10u32), Size::new(10u32, 0u32)).is_empty());
    //     assert!(Rect::new(Point::new(10u32, 10u32), Size::new(0u32, 10u32)).is_empty());
    //     assert!(!Rect::new(Point::new(10u32, 10u32), Size::new(1u32, 1u32)).is_empty());
    // }

    // #[test]
    // fn test_round() {
    //     let mut x = -2.0;
    //     let mut y = -2.0;
    //     let mut w = -2.0;
    //     let mut h = -2.0;
    //     while x < 2.0 {
    //         while y < 2.0 {
    //             while w < 2.0 {
    //                 while h < 2.0 {
    //                     let rect = Rect::new(Point::new(x, y), Size::new(w, h));

    //                     assert!(rect.contains_rect(&rect.round_in()));
    //                     assert!(rect.round_in().inflate(1.0, 1.0).contains_rect(&rect));

    //                     assert!(rect.round_out().contains_rect(&rect));
    //                     assert!(rect.inflate(1.0, 1.0).contains_rect(&rect.round_out()));

    //                     assert!(rect.inflate(1.0, 1.0).contains_rect(&rect.round()));
    //                     assert!(rect.round().inflate(1.0, 1.0).contains_rect(&rect));

    //                     h += 0.1;
    //                 }
    //                 w += 0.1;
    //             }
    //             y += 0.1;
    //         }
    //         x += 0.1
    //     }
    // }

    // #[test]
    // fn test_center() {
    //     let r: Rect<i32> = rect(-2, 5, 4, 10);
    //     assert_eq!(r.center(), point2(0, 10));

    //     let r: Rect<f32> = rect(1.0, 2.0, 3.0, 4.0);
    //     assert_eq!(r.center(), point2(2.5, 4.0));
    // }

    // #[test]
    // fn test_nan() {
    //     let r1: Rect<f32> = rect(-2.0, 5.0, 4.0, std::f32::NAN);
    //     let r2: Rect<f32> = rect(std::f32::NAN, -1.0, 3.0, 10.0);

    //     assert_eq!(r1.intersection(&r2), None);
    // }
}
