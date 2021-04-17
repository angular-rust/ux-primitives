use std::cell::RefCell;

use crate::Color;

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

#[derive(Copy, Clone, Debug)]
pub struct ColorStop {
    pub offset: f64,
    pub color: Color,
}

impl ColorStop {
    pub fn new(offset: f64, color: Color) -> Self {
        Self { offset, color }
    }
}

/// A representation of the RGB (Red, Green, Blue) color space.
#[derive(Copy, Clone, Debug)]
pub enum GradientType {
    Linear(LinearGradient),
    Radial(RadialGradient),
}

impl Default for GradientType {
    fn default() -> Self {
        GradientType::Linear(Default::default())
    }
}

#[derive(Clone, Debug)]
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

    pub fn get_color_count(&self) -> usize {
        let stops = self.stops.borrow();
        stops.len()
    }

    pub fn get_color_stop(&self, index: usize) -> Option<ColorStop> {
        let stops = self.stops.borrow();
        match stops.get(index) {
            Some(stop) => Some(*stop),
            None => None,
        }
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
