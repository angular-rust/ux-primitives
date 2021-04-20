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

#[cfg(feature = "color_from_css")]
#[inline(always)]
pub(crate) fn color_from_short_rgb_u16(c: u16) -> Color {
    let (red, green, blue, _) = (
        ((c >> 8) + ((c >> 8) << 4)) as u8,
        ((c & 0x0f0) + ((c & 0x0f0) >> 4)) as u8,
        ((c & 0x00f) + ((c & 0x00f) << 4)) as u8,
        0xff
    );
    Color::RGB(red, green, blue)
}

#[cfg(feature = "color_from_css")]
#[inline(always)]
pub(crate) fn color_from_rgb_u32(c: u32) -> Color {
    let (red, green, blue, _) = (
        (c >> 16) as u8,
        ((c & 0x00ff00) >> 8) as u8,
        (c & 0x0000ff) as u8,
        0xff
    );
    Color::RGB(red, green, blue)
}

#[cfg(feature = "color_from_css")]
#[inline(always)]
pub(crate) fn color_from_short_rgba_u16(c: u16) -> Color {
    let (red, green, blue, alpha) = (
        ((c >> 12) + ((c >> 12) << 4)) as u8,
        (((c & 0x0f00) >> 4) + ((c & 0x0f00) >> 8)) as u8,
        ((c & 0x00f0) + ((c & 0x00f0) >> 4)) as u8,
        ((c & 0x000f) + ((c & 0x000f) << 4)) as u8
    );
    Color::RGBA(red, green, blue, alpha)
}

#[cfg(feature = "color_from_css")]
#[inline(always)]
pub(crate) fn color_from_rgba_u32(c: u32) -> Color {
    let (red, green, blue, alpha) = (
        (c >> 24) as u8,
        ((c & 0x00ff0000) >> 16) as u8,
        ((c & 0x0000ff00) >> 8) as u8,
        (c & 0x000000ff) as u8
    );
    Color::RGBA(red, green, blue, alpha)
}
