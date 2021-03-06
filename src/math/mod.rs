#![allow(unused_variables)]

use num_traits::Float;
use std::ops;
// use math::round;

pub trait Num<T>: ops::Add<T> + ops::Sub<T> + ops::Mul<T> {}

#[derive(Default, Clone, Copy, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Float + Default,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Get the straight line (Euclidean) distance between the origin (0, 0) and this point
    pub fn magnitude(&self) -> T {
        self.distance_to(Default::default())
    }

    /// Returns the distance between this and other
    pub fn distance_to(&self, target: Point<T>) -> T {
        self.squared_distance_to(target).sqrt()
    }

    /// Returns the squared distance between this and other
    pub fn squared_distance_to(&self, target: Point<T>) -> T {
        let dx = target.x - self.x;
        let dy = target.y - self.y;
        dx * dx + dy  * dy
    }
}

impl<T> PartialEq for Point<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        // let scale = 14; // if scale > 14 -> numbers won't be equals
        // round::stochastic(self.x.to_f64().unwrap(), scale)
        //     == round::stochastic(other.x.to_f64().unwrap(), scale)
        //     && round::stochastic(self.y.to_f64().unwrap(), scale)
        //         == round::stochastic(other.y.to_f64().unwrap(), scale)
        self.x == other.x && self.y == other.y
    }
}

impl<T> ops::Add for Point<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::AddAssign for Point<T>
where
    T: ops::Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T> ops::Sub for Point<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> ops::SubAssign for Point<T>
where
    T: ops::Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl<T> ops::Mul<T> for Point<T>
where
    T: ops::Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> ops::MulAssign<T> for Point<T>
where
    T: ops::Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Rectangle<T> {
    // TODO: Replace "top" and "left" to "origin: Point<T>" it will give us ability to use any of rectangle points as origin point
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T>
where
    T: Float,
{
    pub fn new(left: T, top: T, width: T, height: T) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    #[inline]
    pub fn get_left(&self) -> T {
        self.left
    }

    #[inline]
    pub fn get_top(&self) -> T {
        self.top
    }

    #[inline]
    pub fn get_width(&self) -> T {
        self.width
    }

    #[inline]
    pub fn get_height(&self) -> T {
        self.height
    }

    pub fn get_right(&self) -> T {
        self.left + self.width
    }

    pub fn get_bottom(&self) -> T {
        self.top + self.height
    }

    pub fn get_bottom_left(&self) -> Point<T> {
        Point {
            x: self.left,
            y: self.get_bottom(),
        }
    }

    pub fn get_bottom_right(&self) -> Point<T> {
        Point {
            x: self.get_right(),
            y: self.get_bottom(),
        }
    }

    pub fn get_top_left(&self) -> Point<T> {
        Point {
            x: self.left,
            y: self.top,
        }
    }

    pub fn get_top_right(&self) -> Point<T> {
        Point {
            x: self.get_right(),
            y: self.top,
        }
    }

    /// Returns Bounds tuple (top, right, bottom, left)
    #[inline]
    fn bounds_tuple(&self) -> (T, T, T, T) {
        (self.top, self.get_right(), self.get_bottom(), self.left)
    }

    /// Returns a new rectangle which completely contains self and other
    pub fn bounding_box(&self, other: Self) -> Self {
        let (s_top, s_right, s_bottom, s_left) = self.bounds_tuple();
        let (o_top, o_right, o_bottom, o_left) = other.bounds_tuple();
        let (top, left, right, bottom) = (
            if s_top > o_top { s_top } else { o_top },
            if s_left > o_left { s_left } else { o_left },
            if s_right > o_right { s_right } else { o_right },
            if s_bottom > o_bottom {
                s_bottom
            } else {
                o_bottom
            },
        );
        Self {
            top,
            left,
            width: right - left,
            height: bottom - top,
        }
    }

    /// Tests whether point is inside or along the edges of this
    pub fn contains_point(&self, point: Point<T>) -> bool {
        let (top, right, bottom, left) = self.bounds_tuple();
        point.x >= left && point.x <= right && point.y >= top && point.y <= bottom
    }

    /// Tests whether this entirely contains another
    pub fn contains_rectangle(&self, other: Self) -> bool {
        let (s_top, s_right, s_bottom, s_left) = self.bounds_tuple();
        let (o_top, o_right, o_bottom, o_left) = other.bounds_tuple();
        s_left <= o_left && s_right >= o_right && s_top <= o_top && s_bottom >= o_bottom
    }

    /// Returns true if this intersects other
    pub fn intersects(&self, other: Self) -> bool {
        self.contains_point(other.get_top_left())
            || self.contains_point(other.get_top_right())
            || self.contains_point(other.get_bottom_right())
            || self.contains_point(other.get_bottom_left())
    }
}

impl<T> PartialEq for Rectangle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top
            && self.left == other.left
            && self.width == other.width
            && self.height == other.height
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Matrix<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
    pub e: T,
    pub f: T,
}

/// Interface of SVGMatrix:
/// @see https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix
impl<T> Matrix<T>
where
    T: Float,
{
    pub fn flip_x(&self) -> Matrix<T> {
        unimplemented!()
    }
    pub fn flip_y(&self) -> Matrix<T> {
        unimplemented!()
    }
    pub fn inverse(&self) -> Matrix<T> {
        unimplemented!()
    }
    pub fn multiply(&self, second_matrix: Matrix<T>) -> Matrix<T> {
        unimplemented!()
    }
    pub fn rotate(&self, angle: T) -> Matrix<T> {
        unimplemented!()
    }
    pub fn rotate_from_vector(&self, x: T, y: T) -> Matrix<T> {
        unimplemented!()
    }
    pub fn scale(&self, scale_factor: T) -> Matrix<T> {
        unimplemented!()
    }
    pub fn scale_non_uniform(&self, scale_factor_x: T, scale_factor_y: T) -> Matrix<T> {
        unimplemented!()
    }
    pub fn skew_x(&self, angle: T) -> Matrix<T> {
        unimplemented!()
    }
    pub fn skew_y(&self, angle: T) -> Matrix<T> {
        unimplemented!()
    }

    pub fn translate(x: T, y: T) -> Matrix<T> {
        unimplemented!()
    }
    // TODO: ==
}

#[cfg(test)]
mod tests {
    use super::Point;
    use super::Rectangle;

    #[test]
    fn test_point_distance() {
        assert_eq!(
            Point::new(0.0, 10.0).distance_to(Point::new(100.0, 10.0)),
            100.0
        );
        assert_eq!(
            Point::new(0.0, 0.0).squared_distance_to(Point::new(100.0, 100.0)),
            20_000_f64
        );
        assert_eq!(
            Point::new(0.0, 0.0).distance_to(Point::new(100.0, 100.0)),
            20_000_f64.sqrt()
        );
        assert_eq!(Point::new(100.0, 100.0).magnitude(), 20_000_f64.sqrt());
    }

    #[test]
    fn test_point_operators() {
        assert_eq!(Point::new(1.0, 1.0), Point::new(1.0, 1.0));
        assert!((Point::new(1.0, 1.0) == Point::new(1.0, 1.0)));
        assert_ne!(Point::new(1.0, 1.0), Point::new(2.0, 1.0));
        assert!((Point::new(1.0, 1.0) != Point::new(2.0, 1.0)));
        assert_eq!(
            Point::new(1.0, 1.0) + Point::new(2.0, 2.0),
            Point::new(3.0, 3.0)
        );
        assert_eq!(Point::new(2.5, 4.2) * 2.0, Point::new(5.0, 8.4));

        let mut add_assing = Point::new(1.0, 1.0);
        add_assing += Point::new(2.3, 5.7);
        assert_eq!(add_assing, Point::new(3.3, 6.7));

        let mut sub_assing = Point::new(7.4, 12.7);
        sub_assing -= Point::new(3.5, 5.8);
        assert_eq!(sub_assing, Point::new(3.9, 6.9));
    }

    #[test]
    fn test_rect_points() {
        let rect = Rectangle::new(100.0, 100.0, 100.0, 100.0);
        assert_eq!(rect.get_top_left(), Point::new(100.0, 100.0));
        assert_eq!(rect.get_top_right(), Point::new(200.0, 100.0));
        assert_eq!(rect.get_bottom_right(), Point::new(200.0, 200.0));
        assert_eq!(rect.get_bottom_left(), Point::new(100.0, 200.0));
        assert_ne!(rect.get_bottom_left(), Point::new(100.0, 201.0));
        //assert_eq!(rect.get_top_right(), Point::new(100.0, 100.0));
    }

    #[test]
    fn test_rect_contains_point() {
        let rect = Rectangle::new(100.0, 100.0, 50.0, 50.0);
        assert!(rect.contains_point(Point::new(120.0, 120.0)));
        assert!(rect.contains_point(Point::new(100.0, 100.0)));
        assert!(rect.contains_point(Point::new(120.0, 149.0)));
        assert!(rect.contains_point(Point::new(150.0, 150.0)));
        assert!(!rect.contains_point(Point::new(50.0, 50.0)));
        assert!(!rect.contains_point(Point::new(120.0, 151.0)));
        assert!(!rect.contains_point(Point::new(151.0, 120.0)));
    }

    #[test]
    fn test_rect_intersection() {
        let rect = Rectangle::new(100.0, 100.0, 100.0, 100.0);
        assert!(rect.intersects(Rectangle::new(150.0, 150.0, 10.0, 10.0)));
        assert!(rect.intersects(Rectangle::new(200.0, 200.0, 10.0, 10.0)));
        assert!(!rect.intersects(Rectangle::new(201.0, 200.0, 10.0, 10.0)));

        assert!(rect.intersects(Rectangle::new(0.0, 0.0, 100.0, 100.0)));
        assert!(!rect.intersects(Rectangle::new(0.0, 0.0, 99.0, 100.0)));
        assert!(!rect.intersects(Rectangle::new(0.0, 0.0, 100.0, 99.0)));

        assert!(!rect.intersects(Rectangle::new(0.0, 0.0, 100.0, 99.99999999999999)));
        assert!(rect.intersects(Rectangle::new(0.0, 0.0, 100.0, 99.999999999999999)));
    }
}
