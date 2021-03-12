#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::{color::Color, text::{TextStyle, TextWeight}};
use crate::text::{TextAlign, BaseLine};

use super::direction::Direction;
use super::{
    LineCap, LineJoin,
    image::ImageDataInterface,
    style::{CanvasGradientInterface, CanvasPatternInterface, CanvasStyle},
    text_metrics::TextMetrics,
};

pub trait CanvasContext {
    // fn get_current_transform(&self) -> Box<dyn MatrixInterface>;
    // fn set_current_transform(&self, value: Matrix<f64>);

    fn get_direction(&self) -> Direction;
    fn set_direction(&self, value: Direction) -> String;

    // @Creates('String|CanvasGradient|CanvasPattern'), @Returns('String|CanvasGradient|CanvasPattern')
    // fillStyle: Object;
    // fn get_fill_style(&self) -> Box<CanvasStyle<dyn CanvasGradientInterface, dyn CanvasPatternInterface>>;
    // fn set_fill_style(&self, value: CanvasStyle<impl CanvasGradientInterface, impl CanvasPatternInterface>);
    fn set_fill_style_color(&self, value: Color);
    // fn set_fill_style_gradient(&self, value: impl CanvasGradientInterface);
    // fn set_fill_style_pattern(&self, value: impl CanvasPatternInterface);

    fn get_filter(&self) -> String;
    fn set_filter(&self, value: &str);

    fn get_font(&self) -> String;
    fn set_font(&self, family: &str, style: TextStyle, weight: TextWeight, size: f64);

    fn get_global_alpha(&self) -> f64;
    fn set_global_alpha(&self, value: f64);

    fn get_global_composite_operation(&self) -> String;
    fn set_global_composite_operation(&self, value: &str);

    /// Whether images and patterns on this canvas will be smoothed when this canvas is scaled.
    fn is_image_smoothing_enabled(&self) -> bool;
    fn set_image_smoothing(&self, value: bool);

    // fn get_image_smoothing_quality(&self) -> String;
    // fn set_image_smoothing_quality(&self, value: String);

    fn get_line_cap(&self) -> LineCap;
    fn set_line_cap(&self, value: LineCap);

    // @SupportedBrowser(SupportedBrowser.CHROME), @SupportedBrowser(SupportedBrowser.IE, '11'), @SupportedBrowser(SupportedBrowser.SAFARI), @Unstable()
    fn get_line_dash_offset(&self) -> f64;
    fn set_line_dash_offset(&self, value: f64);

    fn get_line_join(&self) -> LineJoin;
    fn set_line_join(&self, value: LineJoin);

    fn get_line_width(&self) -> f64;
    fn set_line_width(&self, value: f64);

    fn get_miter_limit(&self) -> f64;
    fn set_miter_limit(&self, value: f64);

    fn get_shadow_blur(&self) -> f64;
    fn set_shadow_blur(&self, value: f64);

    fn get_shadow_color(&self) -> Color;
    fn set_shadow_color(&self, value: Color);

    fn get_shadow_offset_x(&self) -> f64;
    fn set_shadow_offset_x(&self, value: f64);

    fn get_shadow_offset_y(&self) -> f64;
    fn set_shadow_offset_y(&self, value: f64);

    // @Creates('String|CanvasGradient|CanvasPattern'), @Returns('String|CanvasGradient|CanvasPattern')
    // fn set_stroke_style(&self, value: CanvasStyle<impl CanvasGradientInterface, impl CanvasPatternInterface>);
    fn set_stroke_style_color(&self, value: Color);
    // fn set_stroke_style_gradient(&self, value: impl CanvasGradientInterface);
    // fn set_stroke_style_pattern(&self, value: impl CanvasPatternInterface);

    fn get_text_align(&self) -> TextAlign;
    fn set_text_align(&self, value: TextAlign);

    fn get_text_baseline(&self) -> BaseLine;
    fn set_text_baseline(&self, value: BaseLine);

    // anticlockwise: bool = false
    fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    );
    fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64);
    fn begin_path(&self);
    fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);
    fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64);

    // [path_OR_winding: dynamic, winding: String]
    // fn clip(path_OR_winding: dynamic, winding: String); // TODO:
    fn close_path(&self);

    // @Creates('ImageData|=Object')
    // [int? sh_OR_sw, dynamic imageDataColorSettings_OR_sh, Map? imageDataColorSettings]
    // fn createImageData(data_OR_imagedata_OR_sw: dynamic, sh_OR_sw: int, imageDataColorSettings_OR_sh: dynamic, imageDataColorSettings: Map) -> ImageData; // TODO:

    // fn createImageDataFromImageData(imagedata: ImageData) -> ImageData; // TODO:
    // fn createLinearGradient(x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient; // TODO:
    // fn createPattern(image: Object, repetitionType: String) -> CanvasPattern; // TODO:
    // fn createPatternFromImage(image: ImageElement, repetitionType: String) -> CanvasPattern; // TODO:
    // fn createRadialGradient(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> CanvasGradient; // TODO:

    // [Element? element]
    // fn drawFocusIfNeeded(element_OR_path: dynamic, element: Element); // TODO:

    // Draws an image from a CanvasImageSource to this canvas.
    // fn drawImage(source: CanvasImageSource, destX: f64, destY: f64); // TODO:

    // Draws an image from a CanvasImageSource to an area of this canvas.
    // fn drawImageScaled(source: CanvasImageSource, destX: f64, destY: f64, destWidth: f64, destHeight: f64); // TODO:

    // Draws an image from a CanvasImageSource to an area of this canvas.
    // fn drawImageScaledFromSource(source: CanvasImageSource, sourceX: f64, sourceY: f64, sourceWidth: f64, sourceHeight: f64, destX: f64, destY: f64, destWidth: f64, destHeight: f64); // TODO:

    // Draws an image from a CanvasImageSource to an area of this canvas.
    // fn drawImageToRect(source: CanvasImageSource, destRect: Rectangle<f64>, sourceRect: Rectangle<f64>); // TODO:

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
    // fn fill(path_OR_winding: dynamic, winding: String); // TODO:
    
    fn fill(&self);

    fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64);

    // Draws text to the canvas.
    fn fill_text(&self, text: &str, x: f64, y: f64);

    // fn getContextAttributes() -> Map; // TODO:

    // @Creates('ImageData|=Object')
    // fn getImageData(sx: i64, sy: i64, sw: i64, sh: i64) -> ImageData; // TODO:

    // @SupportedBrowser(SupportedBrowser.CHROME), @SupportedBrowser(SupportedBrowser.IE, '11'), @SupportedBrowser(SupportedBrowser.SAFARI), @Unstable()
    fn get_line_dash(&self) -> Vec<f64>;

    // [dynamic winding_OR_y, String? winding]
    // fn isPointInPath(path_OR_x: dynamic, x_OR_y: f64, winding_OR_y: dynamic, winding: String) -> bool; // TODO:

    // [f64? y]
    // fn isPointInStroke(path_OR_x: dynamic, x_OR_y: f64, y: f64) -> bool; // TODO:
    fn line_to(&self, x: f64, y: f64);

    fn measure_text(&self, text: &str) -> TextMetrics;

    fn move_to(&self, x: f64, y: f64);

    // [int? dirtyX, int? dirtyY, int? dirtyWidth, int? dirtyHeight]
    // fn putImageData(imagedata: ImageData, dx: i64, dy: i64, dirtyX: i64, dirtyY: i64, dirtyWidth: i64, dirtyHeight: i64); // TODO:
    fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64);
    fn rect(&self, x: f64, y: f64, width: f64, height: f64);
    fn reset_transform(&self);
    fn restore(&self);
    fn rotate(&self, angle: f64);
    fn save(&self);
    fn scale(&self, x: f64, y: f64);

    // [Path2D? path]
    // fn scrollPathIntoView(path: Path2D); // TODO:

    /// Sets the color used inside shapes. h is in degrees, 0-360. s, l are in percent, 0-100. a is 0-1.
    fn set_fill_color_hsl(&self, h: i64, s: f64, l: f64, a: f64);

    /// Sets the color used inside shapes. r, g, b are 0-255, a is 0-1.
    fn set_fill_color_rgb(&self, r: i64, g: i64, b: i64, a: f64);

    // @SupportedBrowser(SupportedBrowser.CHROME), @SupportedBrowser(SupportedBrowser.IE, '11'), @SupportedBrowser(SupportedBrowser.SAFARI), @Unstable()
    fn set_line_dash(&self, dash: Vec<f64>);

    /// Sets the color used for stroking shapes. h is in degrees, 0-360. s, l are in percent, 0-100. a is 0-1.
    fn set_stroke_color_hsl(&self, h: i64, s: f64, l: f64, a: f64);

    /// Sets the color used for stroking shapes. r, g, b are 0-255, a is 0-1.
    fn set_stroke_color_rgb(&self, r: i64, g: i64, b: i64, a: f64);

    fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);

    fn stroke(&self);
    fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64);

    fn stroke_text(&self, text: &str, x: f64, y: f64);

    fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);
    fn translate(&self, x: f64, y: f64);
}
