#![allow(unused_imports)]

//! Color scheme is basically a set of colors, constructed according to [color theory](http://en.wikipedia.org/wiki/Color_theory).

use crate::foundation::colorspace::{Color, HsvColor};

mod analogous;
pub use self::analogous::*;

mod complementary;
pub use self::complementary::*;

mod compound;
pub use self::compound::*;

mod flipped_compound;
pub use self::flipped_compound::*;

mod monochrome;
pub use self::monochrome::*;

mod split_complementary;
pub use self::split_complementary::*;

mod tetrad;
pub use self::tetrad::*;

mod triad;
pub use self::triad::*;

/// Alias for `HsvColor` where `b` is for "brightness".
pub type Hsb = HsvColor;

fn ryb_rotate(color: Color, angle: f32) -> HsvColor {
    const RYB_WHEEL: [[u16; 2]; 25] = [
        [0, 0],
        [15, 8],
        [30, 17],
        [45, 26],
        [60, 34],
        [75, 41],
        [90, 48],
        [105, 54],
        [120, 60],
        [135, 81],
        [150, 103],
        [165, 123],
        [180, 138],
        [195, 155],
        [210, 171],
        [225, 187],
        [240, 204],
        [255, 219],
        [270, 234],
        [285, 251],
        [300, 267],
        [315, 282],
        [330, 298],
        [345, 329],
        [360, 0],
    ];

    let mut hsb: HsvColor = color.into();

    let mut a: f32 = 0.0;
    for idx in 0..RYB_WHEEL.len() {
        let x0 = RYB_WHEEL[idx][0] as f32;
        let y0 = RYB_WHEEL[idx][1] as f32;

        let x1 = RYB_WHEEL[idx + 1][0] as f32;
        let mut y1 = RYB_WHEEL[idx + 1][1] as f32;

        if y1 < y0 {
            y1 += 360.0;
        }

        if y0 <= hsb.hue && hsb.hue <= y1 {
            a = 1.0 * x0 + (x1 - x0) * (hsb.hue - y0) / (y1 - y0);
            break;
        }
    }

    a += angle % 360.0;

    if a < 0.0 {
        a += 360.0;
    }

    if a > 360.0 {
        a -= 360.0;
    }

    a %= 360.0;

    let mut new_hue: f32 = 0.0;
    for idx in 0..RYB_WHEEL.len() {
        let xx0 = RYB_WHEEL[idx][0] as f32;
        let yy0 = RYB_WHEEL[idx][1] as f32;

        let xx1 = RYB_WHEEL[idx + 1][0] as f32;
        let mut yy1 = RYB_WHEEL[idx + 1][1] as f32;

        if yy1 < yy0 {
            yy1 += 360.0;
        }

        if xx0 <= a && a <= xx1 {
            new_hue = yy0 + (yy1 - yy0) * (a - xx0) / (xx1 - xx0);
            break;
        }
    }

    hsb.hue = new_hue;

    hsb
}

#[inline]
fn wrap(x: f32, min: f32, threshold: f32, plus: f32) -> f32 {
    if x - min < threshold {
        x + plus
    } else {
        x - min
    }
}
