use bytes::Bytes;

/// Describes pixel format properties
#[derive(Copy, Clone, Debug)]
pub enum PixelFormat {
    Invalid,
    ARgb32,
    Rgb24,
    A8,
    A1,
    Rgb16_565,
    Rgb30,
}

impl Default for PixelFormat {
    fn default() -> Self {
        PixelFormat::ARgb32
    }
}

/// Represens image data with parameters
#[derive(Debug, Clone)]
pub struct ImageData {
    pub format: PixelFormat,
    pub width: u32,
    pub height: u32,
    pub data: Bytes,
}

impl ImageData {
    pub fn new(format: PixelFormat, width: u32, height: u32, data: Bytes) -> Self {
        Self {
            format,
            width,
            height,
            data,
        }
    }
}

impl Default for ImageData {
    fn default() -> Self {
        Self {
            format: Default::default(),
            width: 0,
            height: 0,
            data: Default::default(),
        }
    }
}
