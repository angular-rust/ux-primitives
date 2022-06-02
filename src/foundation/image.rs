use bytes::Bytes;

/// Describes pixel format properties
#[derive(Copy, Clone, Debug)]
pub enum PixelFormat {
    /// Invalid pixel format
    Invalid,
    /// Represents ARgb32 format
    ARgb32,
    /// Represents Rgb24 format
    Rgb24,
    /// Represents A8 format
    A8,
    /// Represents A1 format
    A1,
    /// Represents Rgb16_565 format
    Rgb16_565,
    /// Represents Rgb30 format
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
    /// Image format
    pub format: PixelFormat,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// Image data
    pub data: Bytes,
}

impl ImageData {
    /// Create image data with params
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
