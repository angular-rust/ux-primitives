use std::cell::RefCell;

use crate::foundation::colorspace::Color;

/// Represent radial gradient specified by six parameters
#[derive(Default, Copy, Clone, Debug)]
pub struct RadialGradient {
    pub x0: f64,
    pub y0: f64,
    pub r0: f64,
    pub x1: f64,
    pub y1: f64,
    pub r1: f64,
}

impl RadialGradient {
    // may be use Point`s
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

/// Represent radial gradient specified by four parameters

#[derive(Default, Copy, Clone, Debug)]
pub struct LinearGradient {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl LinearGradient {
    // may be use Point`s
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self { x0, y0, x1, y1 }
    }
}


/// Define the an offset and a color, to a given canvas gradient. 
#[derive(Debug, Copy, Clone)]
pub struct ColorStop {
    pub offset: f64,
    pub color: Color,
}

impl ColorStop {
    pub fn new(offset: f64, color: Color) -> Self {
        Self { offset, color }
    }
}

/// Define the gradient type with parameters
#[derive(Debug, Copy, Clone)]
pub enum GradientType {
    Linear(LinearGradient),
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
    pub kind: GradientType,
    pub stops: RefCell<Vec<ColorStop>>,
}

impl Gradient {
    pub fn new(kind: GradientType) -> Self {
        Self {
            kind,
            stops: Default::default(),
        }
    }

    pub fn add_color_stop(&self, stop: ColorStop) {
        let mut stops = self.stops.borrow_mut();
        stops.push(stop)
    }

    pub fn color_count(&self) -> usize {
        let stops = self.stops.borrow();
        stops.len()
    }

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
