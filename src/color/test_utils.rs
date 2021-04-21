#![allow(dead_code)]

use super::*;
use core::slice::Iter;
use std::fmt;

pub(super) fn diff_less_than_u8(left: u8, right: u8, diff: u8) -> bool {
    if left > right {
        (left - right) <= diff
    } else {
        (right - left) <= diff
    }
}
pub(super) fn diff_less_than_f64(left: f64, right: f64, diff: f64) -> bool {
    if left > right {
        (left - right) <= diff
    } else {
        (right - left) <= diff
    }
}

pub(super) fn test_conversion<C, F>(test_colors_iterator: Iter<'_, (Color, C)>, callback: F)
    where
        C: fmt::Display + Copy + Clone,
        F: Fn(&Color, &C) -> ()
{
    for (left_color, right_space) in test_colors_iterator {
        callback(left_color, right_space)
    }
}

pub(super) fn test_to_rgb_conversion<C>(test_colors_iterator: Iter<'_, (Color, C)>)
    where C: fmt::Display + Copy + Clone + IntoColor<RgbColor> + Into<Color>
{
    test_utils::test_conversion(
        test_colors_iterator,
        |expected_color, actual_color| {
            let expected_rgb = RgbColor::from(*expected_color);
            let actual_rgb = RgbColor::from_color(*actual_color);
            let RgbColor { red: expected_r, green: expected_g, blue: expected_b } = expected_rgb;
            let RgbColor { red: actual_r, green: actual_g, blue: actual_b} = actual_rgb;
            assert!(test_utils::diff_less_than_u8(actual_r, expected_r, 3), "{} -> {} != {}", *actual_color, actual_rgb, expected_rgb);
            assert!(test_utils::diff_less_than_u8(actual_g, expected_g, 3), "{} -> {} != {}", *actual_color, actual_rgb, expected_rgb);
            assert!(test_utils::diff_less_than_u8(actual_b, expected_b, 3), "{} -> {} != {}", *actual_color, actual_rgb, expected_rgb);
        }
    )
}

lazy_static! {
    pub(super) static ref RGB_HSV: Vec<(Color, HsvColor)> = vec!(
        (palette::BLUE_5,   HsvColor::new(207.0, 78.0,  94.0)),  // rgb(51, 154, 240)
        (palette::CYAN_6,   HsvColor::new(187.0, 89.0,  75.0)),  // rgb(21, 170, 191)
        (palette::TEAL_5,   HsvColor::new(162.0, 84.0,  79.0)),  // rgb(32, 201, 151)
        (palette::GREEN_4,  HsvColor::new(130.0, 52.0,  86.0)),  // rgb(105, 219, 124)
        (palette::LIME_1,   HsvColor::new(80.0,  20.0,  98.0)),  // rgb(233, 250, 200)
        (palette::LIME_6,   HsvColor::new(85.0,  85.0,  79.0)),  // rgb(130, 201, 30)
        (palette::YELLOW_4, HsvColor::new(47.0,  77.0,  100.0)), // rgb(255, 212, 59)
        (palette::YELLOW_8, HsvColor::new(35.0,  100.0, 94.0)),  // rgb(240, 140, 0)
        (palette::ORANGE_2, HsvColor::new(33.0,  34.0,  100.0)), // rgb(255, 216, 168)
        (palette::ORANGE_6, HsvColor::new(27.0,  92.0,  99.0)),  // rgb(253, 126, 20)
    );
}

lazy_static! {
    pub(super) static ref RGB_HSL: Vec<(Color, HslColor)> = vec!(
        (palette::BLUE_5,   HslColor::new(207.0, 86.0,  57.0)), // rgb(51, 154, 240)
        (palette::CYAN_6,   HslColor::new(187.0, 80.0,  42.0)), // rgb(21, 170, 191)
        (palette::TEAL_5,   HslColor::new(162.0, 73.0,  46.0)), // rgb(32, 201, 151)
        (palette::GREEN_4,  HslColor::new(130.0, 61.0,  64.0)), // rgb(105, 219, 124)
        (palette::LIME_1,   HslColor::new(80.0,  83.0,  88.0)), // rgb(233, 250, 200)
        (palette::LIME_6,   HslColor::new(85.0,  74.0,  45.0)), // rgb(130, 201, 30)
        (palette::YELLOW_4, HslColor::new(47.0,  100.0, 62.0)), // rgb(255, 212, 59)
        (palette::YELLOW_8, HslColor::new(35.0,  100.0, 47.0)), // rgb(240, 140, 0)
        (palette::ORANGE_2, HslColor::new(33.0,  100.0, 83.0)), // rgb(255, 216, 168)
        (palette::ORANGE_6, HslColor::new(27.0,  98.0,  54.0)), // rgb(253, 126, 20)
    );
}
