#![cfg(feature = "canvas")]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{BaseLine, Color, TextAlign, TextStyle, TextWeight};

mod style;
pub use style::*;

mod image;
pub use image::*;

mod direction;
pub use direction::*;

mod text_metrics;
pub use text_metrics::*;

#[derive(Clone, Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl Default for LineCap {
    fn default() -> Self {
        LineCap::Butt
    }
}

#[derive(Clone, Debug)]
pub enum LineJoin {
    Miter,
    Bevel,
    Round,
}

impl Default for LineJoin {
    fn default() -> Self {
        LineJoin::Miter
    }
}

pub trait CanvasContext {
    // /// Get current global transformation matrix
    // fn get_current_transform(&self) -> Matrix<f64>;

    // /// Set global transformation matrix
    // fn set_current_transform(&self, value: Matrix<f64>);

    /// Get direction
    fn get_direction(&self) -> Direction;

    /// Set direction
    fn set_direction(&self, value: Direction) -> String;

    /// Set fill color
    fn set_fill_color(&self, value: Color);

    // /// Set fill gradient
    // fn set_fill_gradient(&self, value: impl CanvasGradientInterface);

    // /// Set fill pattern
    // fn set_fill_pattern(&self, value: impl CanvasPatternInterface);

    /// Get filter
    fn get_filter(&self) -> String;

    /// Set filter
    fn set_filter(&self, value: &str);

    /// Get font
    fn get_font(&self) -> String;

    /// Set direction
    fn set_font(&self, family: &str, style: TextStyle, weight: TextWeight, size: f64);

    /// Get global alpha
    fn get_global_alpha(&self) -> f64;

    /// Set global alpha
    fn set_global_alpha(&self, value: f64);

    /// Get global composite operation
    fn get_global_composite_operation(&self) -> String;

    /// Set global composite operation
    fn set_global_composite_operation(&self, value: &str);

    /// Whether images and patterns on this canvas will be smoothed when
    /// this canvas is scaled.
    fn is_image_smoothing_enabled(&self) -> bool;

    /// Set image smoothing
    fn set_image_smoothing(&self, value: bool);

    // /// Get image smoothing quality
    // fn get_image_smoothing_quality(&self) -> String;

    // /// Set image smoothing quality
    // fn set_image_smoothing_quality(&self, value: String);

    /// Get line cap
    fn get_line_cap(&self) -> LineCap;

    /// Set line cap
    fn set_line_cap(&self, value: LineCap);

    /// Get line dash offset
    fn get_line_dash_offset(&self) -> f64;

    /// Set line dash offset
    fn set_line_dash_offset(&self, value: f64);

    /// Get line join
    fn get_line_join(&self) -> LineJoin;

    /// Set line dash
    fn set_line_join(&self, value: LineJoin);

    /// Get line width
    fn get_line_width(&self) -> f64;

    /// Set line width
    fn set_line_width(&self, value: f64);

    /// Get miter limit
    fn get_miter_limit(&self) -> f64;

    /// Set miter limit
    fn set_miter_limit(&self, value: f64);

    /// Get shadow blur
    fn get_shadow_blur(&self) -> f64;

    /// Set shadow blur
    fn set_shadow_blur(&self, value: f64);

    /// Get shadow color
    fn get_shadow_color(&self) -> Color;

    /// Set shadow color
    fn set_shadow_color(&self, value: Color);

    /// Get shadow offset x
    fn get_shadow_offset_x(&self) -> f64;

    /// Set shadow offset x
    fn set_shadow_offset_x(&self, value: f64);

    /// Get shadow offset y
    fn get_shadow_offset_y(&self) -> f64;

    /// Set shadow offset y
    fn set_shadow_offset_y(&self, value: f64);

    /// Set stroke color
    fn set_stroke_color(&self, value: Color);

    // /// Set stroke gradient
    // fn set_stroke_gradient(&self, value: impl CanvasGradientInterface);

    // /// Set stroke pattern
    // fn set_stroke_pattern(&self, value: impl CanvasPatternInterface);

    /// Get text align
    fn get_text_align(&self) -> TextAlign;

    /// Set text align
    fn set_text_align(&self, value: TextAlign);

    /// Get text baseline
    fn get_text_baseline(&self) -> BaseLine;

    /// Set text baseline
    fn set_text_baseline(&self, value: BaseLine);

    /// Add arc to current path with anticlockwise param
    fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    );

    /// Add arc to current path
    fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64);

    /// Begin the path
    fn begin_path(&self);

    /// Add bezier curve to current path
    fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);

    /// Clear rectangle on current canvas
    fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64);

    // [path_OR_winding: dynamic, winding: String]

    // /// Set clip to current operation
    // fn clip(path_OR_winding: dynamic, winding: String); // TODO:

    /// Close the current path
    fn close_path(&self);

    // [int? sh_OR_sw, dynamic imageDataColorSettings_OR_sh, Map? imageDataColorSettings]

    // /// Create the image data from slice
    // fn createImageData(data_OR_imagedata_OR_sw: dynamic, sh_OR_sw: int, imageDataColorSettings_OR_sh: dynamic, imageDataColorSettings: Map) -> ImageData; // TODO:

    // /// Create the image data from other
    // fn createImageDataFromImageData(imagedata: ImageData) -> ImageData; // TODO:

    // /// Create linear gradiaent
    // fn createLinearGradient(x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient; // TODO:

    // /// Create pattern
    // fn createPattern(image: Object, repetitionType: String) -> CanvasPattern; // TODO:

    // /// Create pattern from image
    // fn createPatternFromImage(image: ImageElement, repetitionType: String) -> CanvasPattern; // TODO:

    // /// Create radial gradient
    // fn createRadialGradient(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> CanvasGradient; // TODO:

    // [Element? element]
    // fn drawFocusIfNeeded(element_OR_path: dynamic, element: Element); // TODO:

    // /// Draws an image from a CanvasImageSource to this canvas.
    // fn drawImage(source: CanvasImageSource, destX: f64, destY: f64); // TODO:

    // /// Draws an image from a CanvasImageSource to an area of this canvas.
    // fn drawImageScaled(source: CanvasImageSource, destX: f64, destY: f64, destWidth: f64, destHeight: f64); // TODO:

    // /// Draws an image from a CanvasImageSource to an area of this canvas.
    // fn drawImageScaledFromSource(source: CanvasImageSource, sourceX: f64, sourceY: f64, sourceWidth: f64, sourceHeight: f64, destX: f64, destY: f64, destWidth: f64, destHeight: f64); // TODO:

    // /// Draws an image from a CanvasImageSource to an area of this canvas.
    // fn drawImageToRect(source: CanvasImageSource, destRect: Rectangle<f64>, sourceRect: Rectangle<f64>); // TODO:

    /// Add ellipse to current path
    fn ellipse(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    );

    // [dynamic path_OR_winding, String? winding]

    // /// Fill current path
    // fn fill(path_OR_winding: dynamic, winding: String); // TODO:

    /// Fill current path
    fn fill(&self);

    /// Fill rectangle
    fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64);

    /// Draws text to the canvas.
    fn fill_text(&self, text: &str, x: f64, y: f64);

    // fn getContextAttributes() -> Map; // TODO:

    // fn getImageData(sx: i64, sy: i64, sw: i64, sh: i64) -> ImageData; // TODO:

    /// Get line dash
    fn get_line_dash(&self) -> Vec<f64>;

    // [dynamic winding_OR_y, String? winding]
    // fn isPointInPath(path_OR_x: dynamic, x_OR_y: f64, winding_OR_y: dynamic, winding: String) -> bool; // TODO:

    // [f64? y]
    // fn isPointInStroke(path_OR_x: dynamic, x_OR_y: f64, y: f64) -> bool; // TODO:

    /// Add line to path from current position
    fn line_to(&self, x: f64, y: f64);

    /// Measure text using current font face and font size
    fn measure_text(&self, text: &str) -> TextMetrics;

    /// Move cursor to position
    fn move_to(&self, x: f64, y: f64);

    // [int? dirtyX, int? dirtyY, int? dirtyWidth, int? dirtyHeight]
    // fn putImageData(imagedata: ImageData, dx: i64, dy: i64, dirtyX: i64, dirtyY: i64, dirtyWidth: i64, dirtyHeight: i64); // TODO:

    /// Add quadratic curve to current path
    fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64);

    /// Add rectangle to current path
    fn rect(&self, x: f64, y: f64, width: f64, height: f64);

    /// Reset current transformations
    fn reset_transform(&self);

    /// Restore transformations
    fn restore(&self);

    /// Add rotate to current transformations
    fn rotate(&self, angle: f64);

    /// Save current transformations
    fn save(&self);

    /// Add scle to current transformations
    fn scale(&self, x: f64, y: f64);

    // [Path2D? path]
    // fn scrollPathIntoView(path: Path2D); // TODO:

    /// Set line dash
    fn set_line_dash(&self, dash: Vec<f64>);

    /// Set transform matrix
    fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);

    /// Stroke current path
    fn stroke(&self);

    /// Stroke rectangle
    fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64);

    /// Stroke text
    fn stroke_text(&self, text: &str, x: f64, y: f64);

    /// Add transform matrix to current transformations
    fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);

    /// Add translate to current transformations
    fn translate(&self, x: f64, y: f64);
}
