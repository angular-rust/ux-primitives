#![cfg(feature = "experimental")]

use super::*;


pub trait WithAlpha<C: From<Color>> {
    fn color(self) -> C;
    fn alpha(self) -> f64;
}

#[derive(Clone, Copy, Debug)]
pub struct Alpha<C: From<Color>> {
    color: C,
    alpha: f64
}

impl<C: From<Color>> Alpha<C> {
    fn new(color: C, alpha: f64) -> Self {
        Self { color, alpha }
    }
}


impl<C: From<Color>> WithAlpha<C> for Alpha<C> {
    fn color(self) -> C { self.color }
    fn alpha(self) -> f64 { self.alpha }
}

impl<C: From<Color>> From<C> for Alpha<C> {
    fn from(color: C) -> Self {
        Self::new(color, 0.0)
    }
}

impl WithAlpha<RgbColor> for RgbaColor {
    fn color(self) -> RgbColor { self.into() }
    fn alpha(self) -> f64 { (self.alpha as f64) / 255.0 }
}