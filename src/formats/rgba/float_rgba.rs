use crate::{
    formats::rgba::{Rgba, RgbaFormat},
    pixel::{Format, raw::F32x4},
};

/// A 128-bit RGBA pixel format with four 32-bit floating point channels.
///
/// This format is used to represent pixels in the RGBA order:
/// - `R`ed (32 bits)
/// - `G`reen (32 bits)
/// - `B`lue (32 bits)
/// - `A`lpha (32 bits)
///
/// The pixel is represented as a 128-bit value, where each channel occupies 32 bits.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FloatRgba {}

impl crate::internal::Sealed for FloatRgba {}

impl Format for FloatRgba {
    type RawPixel = F32x4;
    type Channels = Rgba;
}

impl RgbaFormat for FloatRgba {
    const RED_OFFSET: usize = 0;
    const GREEN_OFFSET: usize = 1;
    const BLUE_OFFSET: usize = 2;
    const ALPHA_OFFSET: usize = 3;
}

#[cfg(test)]
mod tests {
    use crate::pixel::{Pixel, raw::RawPixel};

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn new_zeroed() {
        let pixel: Pixel<FloatRgba> = Pixel::zeroed();
        assert_eq!(pixel.as_raw().into_inner(), [0.0; 4]);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn from_raw() {
        let pixel = Pixel::<FloatRgba>::from_raw([0.5, 0.25, 0.75, 1.0].into());
        assert_eq!(pixel.as_raw().into_inner(), [0.5, 0.25, 0.75, 1.0]);
        assert_eq!(pixel.red(), 0.5);
        assert_eq!(pixel.green(), 0.25);
        assert_eq!(pixel.blue(), 0.75);
        assert_eq!(pixel.alpha(), 1.0);
    }
}
