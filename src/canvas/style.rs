#![allow(dead_code)]
use crate::color::Color;

pub struct RadialGradient {
    x0: f64,
    y0: f64,
    r0: f64,
    x1: f64,
    y1: f64,
    r1: f64,
}

pub struct LinearGradient {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

pub struct ColorStop {
    pub offset: f64,
    pub color: Color,
}

pub struct Pattern {

}

pub struct Gradient {

}

impl Gradient {
    pub fn add_color_stop(&self, _stop: ColorStop) {

    }

    pub fn get_color_count(&self) -> usize {
        unimplemented!()
    }

    pub fn get_color_stop(&self, _index: usize) -> Option<ColorStop> {
        unimplemented!()
    }
}
