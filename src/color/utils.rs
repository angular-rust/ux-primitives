use core::slice::Iter;
#[cfg(feature = "color_from_css")]
use super::Color;

#[inline(always)]
pub(crate) fn min_max_tuple(color_cmp_iter: Iter<'_, f64>) -> (f64, f64) {
    color_cmp_iter.fold((1.0f64, 0f64), |acc, color_cmp| -> (f64, f64) {
        (
            if *color_cmp < acc.0 { *color_cmp } else { acc.0 },
            if *color_cmp > acc.1 { *color_cmp } else { acc.1 },
        )
    })
}

#[inline(always)]
pub(crate) fn hue_bound(hue: f64) -> f64 {
    let hue_rest = hue % 360.;
    if (hue_rest.abs() - 0.) < f64::EPSILON {
        return 0.
    }
    if hue_rest < f64::MIN_POSITIVE {
        360. - hue_rest
    } else {
        hue_rest
    }
}

pub(crate) fn percentage_delta_bound(delta: f64) -> f64 {
    let abs_delta = delta.abs();
    if (abs_delta - 0.) < f64::EPSILON {
        return 0.
    }
    if (abs_delta - 100.) >= f64::MIN_POSITIVE {
        let sign = if delta < f64::MIN_POSITIVE { -1. } else { 1. };
        return 100. * sign
    }
    delta
}

#[inline(always)]
pub(crate) fn percentage_bound(value: f64) -> f64 {
    if (value - 0.) < f64::MIN_POSITIVE {
        0.
    } else if (value - 100.) >= f64::MIN_POSITIVE {
        100.
    } else {
        value
    }
}

#[inline(always)]
pub(crate) fn percentage_to_fraction(value: f64) -> f64 {
    percentage_bound(value) / 100.
}
