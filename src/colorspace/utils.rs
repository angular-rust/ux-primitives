use super::Float;
use core::slice::Iter;

#[inline(always)]
pub(crate) fn min_max_tuple(color_cmp_iter: Iter<'_, Float>) -> (Float, Float) {
    color_cmp_iter.fold((1.0 as Float, 0 as Float), |acc, color_cmp| -> (Float, Float) {
        (
            if *color_cmp < acc.0 {
                *color_cmp
            } else {
                acc.0
            },
            if *color_cmp > acc.1 {
                *color_cmp
            } else {
                acc.1
            },
        )
    })
}

/// Returns `value` clamped between `low` and `high`.
#[inline(always)]
pub fn clamp<T: PartialOrd>(value: T, low: T, high: T) -> T {
    debug_assert!(low < high, "low is bigger than high!");
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

#[inline(always)]
pub(crate) fn hue_bound(hue: Float) -> Float {
    let hue_rest = hue % 360.;
    if (hue_rest - 0.).abs() < Float::EPSILON {
        return 0.;
    }
    if hue_rest < Float::MIN_POSITIVE {
        360. - hue_rest
    } else {
        hue_rest
    }
}

#[inline(always)]
pub(crate) fn percentage_to_fraction(value: Float) -> Float {
    clamp(value, 0., 100.) / 100.
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_clamp() {
        assert_eq!(clamp(0.5, 1.0, 2.0), 1.0);
        assert_eq!(clamp(1.5, 1.0, 2.0), 1.5);
        assert_eq!(clamp(3.0, 1.0, 2.0), 2.0);

        let low = 1.0;
        let value = 2.0;
        let high = 3.0;
        assert_eq!(clamp(&value, &low, &high), &value);
    }
}
