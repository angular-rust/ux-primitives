use std::cell::RefCell;

use crate::foundation::colorspace::Color;

/// Represents radial gradient specified by six parameters
#[derive(Default, Copy, Clone, Debug)]
pub struct RadialGradient {
    /// Represents first point x position of gradient
    pub x0: f64,
    /// Represents first point y position of gradient
    pub y0: f64,
    /// Represents first point radius of gradient
    pub r0: f64,
    /// Represents second point x position of gradient
    pub x1: f64,
    /// Represents second point y position of gradient
    pub y1: f64,
    /// Represents second point radius of gradient
    pub r1: f64,
}

impl RadialGradient {
    /// Create new radial gradient with params
    pub fn new(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> Self {
        Self {
            x0,
            y0,
            r0,
            x1,
            y1,
            r1,
        }
    }
}

/// Represents radial gradient specified by four parameters

#[derive(Default, Copy, Clone, Debug)]
pub struct LinearGradient {
    /// Represents first point x position of gradient
    pub x0: f64,
    /// Represents first point y position of gradient
    pub y0: f64,
    /// Represents second point x position of gradient
    pub x1: f64,
    /// Represents second point y position of gradient
    pub y1: f64,
}

impl LinearGradient {
    /// Create new linear gradient with params
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self { x0, y0, x1, y1 }
    }
}


/// Define the an offset and a color, to a given canvas gradient. 
#[derive(Debug, Copy, Clone)]
pub struct ColorStop {
    /// Reperesent the offset of color stop
    pub offset: f64,
    /// Reperesent the color
    pub color: Color,
}

impl ColorStop {
    /// Create new color stop with params
    pub fn new(offset: f64, color: Color) -> Self {
        Self { offset, color }
    }
}

/// Define the gradient type with parameters
#[derive(Debug, Copy, Clone)]
pub enum GradientType {
    /// Linear gradient
    Linear(LinearGradient),
    /// Radial gradient
    Radial(RadialGradient),
}

impl Default for GradientType {
    fn default() -> Self {
        GradientType::Linear(Default::default())
    }
}

/// Represents an opaque object describing a gradient.
#[derive(Debug, Clone)]
pub struct Gradient {
    /// Kind of gradient
    pub kind: GradientType,
    /// Color stop store
    pub stops: RefCell<Vec<ColorStop>>,
}

impl Gradient {
    /// Create new gradient with type
    pub fn new(kind: GradientType) -> Self {
        Self {
            kind,
            stops: Default::default(),
        }
    }

    /// Create color stop to gradient
    pub fn add_color_stop(&self, stop: ColorStop) {
        let mut stops = self.stops.borrow_mut();
        stops.push(stop)
    }

    /// Retrieve count of color stop's
    pub fn color_count(&self) -> usize {
        let stops = self.stops.borrow();
        stops.len()
    }

    /// Retrieve color stop by index
    pub fn get_color_stop(&self, index: usize) -> Option<ColorStop> {
        let stops = self.stops.borrow();
        stops.get(index).copied()
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            stops: Default::default(),
        }
    }
}
