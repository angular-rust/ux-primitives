#![allow(unused_variables)]
#[derive(Default, Clone, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Get the straight line (Euclidean) distance between the origin (0, 0) and this point
    pub fn magnitude(&self) -> T {
        unimplemented!()
    }

    /// Returns the distance between this and other
    pub fn distance_to(&self, other: Point<T>) -> T {
        unimplemented!()
    }

    /// Returns the squared distance between this and other
    pub fn squared_distance_to(&self, other: Point<T>) -> T {
        unimplemented!()
    }

    // TODO: +,-,*, ==
}

#[derive(Default, Clone, Debug)]
pub struct Rectangle<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T> {
    pub fn get_bottom(&self) -> T {
        unimplemented!()
    }

    pub fn get_bottom_left(&self) -> Point<T> {
        unimplemented!()
    }

    pub fn get_bottom_right(&self) -> Point<T> {
        unimplemented!()
    }

    pub fn get_right(&self) -> T {
        unimplemented!()
    }

    pub fn get_top_left(&self) -> Point<T> {
        unimplemented!()
    }

    pub fn get_top_right(&self) -> Point<T> {
        unimplemented!()
    }

    /// Returns a new rectangle which completely contains this and other
    pub fn bounding_box(&self, other: Rectangle<T>) -> Rectangle<T> {
        unimplemented!()
    }

    /// Tests whether another is inside or along the edges of this
    pub fn contains_point(&self, another: Point<T>) -> bool {
        unimplemented!()
    }

    /// Tests whether this entirely contains another
    pub fn contains_rectangle(&self, another: Rectangle<T>) -> bool {
        unimplemented!()
    }

    /// Returns true if this intersects other
    pub fn intersects(&self, other: Rectangle<T>) -> bool {
        unimplemented!()
    }

    // TODO: ==
}

#[derive(Default, Clone, Debug)]
pub struct Matrix<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
    pub e: T,
    pub f: T,
}

impl<T> Matrix<T> {
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
