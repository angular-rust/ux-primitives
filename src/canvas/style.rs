use crate::Color;

pub struct RadialGradient {
    pub x0: f64,
    pub y0: f64,
    pub r0: f64,
    pub x1: f64,
    pub y1: f64,
    pub r1: f64,
}

pub struct LinearGradient {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

pub struct ColorStop {
    pub offset: f64,
    pub color: Color,
}

pub struct Pattern {}

pub struct Gradient {}

impl Gradient {
    pub fn add_color_stop(&self, _stop: ColorStop) {}

    pub fn get_color_count(&self) -> usize {
        unimplemented!()
    }

    pub fn get_color_stop(&self, _index: usize) -> Option<ColorStop> {
        unimplemented!()
    }
}
