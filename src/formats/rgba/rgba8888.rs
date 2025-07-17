use crate::{
    formats::rgba::{Rgba, RgbaFormat},
    pixel::{Format, raw::U32x8888},
};

/// A 32-bit RGBA pixel format with four 8-bit channels.
///
/// This format is used to represent pixels in the RGBA order:
/// - `R`ed (8 bits)
/// - `G`reen (8 bits)
/// - `B`lue (8 bits)
/// - `A`lpha (8 bits)
///
/// The pixel is represented as a 32-bit unsigned integer, where each channel occupies 8 bits.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rgba8888 {}

impl crate::internal::Sealed for Rgba8888 {}
impl Format for Rgba8888 {
    type RawPixel = U32x8888;
    type Channels = Rgba;
}
impl RgbaFormat for Rgba8888 {
    const RED_OFFSET: usize = 0;
    const GREEN_OFFSET: usize = 1;
    const BLUE_OFFSET: usize = 2;
    const ALPHA_OFFSET: usize = 3;
}

#[cfg(test)]
mod tests {
    use crate::pixel::{
        Pixel,
        raw::{RawPixel, U32x8888},
    };

    use super::*;

    #[test]
    fn rgba8888() {
        let mut pixel: Pixel<Rgba8888> = Pixel::new(U32x8888::from(0xFF00_00FF));
        assert_eq!(pixel.red(), 0xFF);
        assert_eq!(pixel.green(), 0x00);
        assert_eq!(pixel.blue(), 0x00);
        assert_eq!(pixel.alpha(), 0xFF);

        pixel.set_red(0x01);
        pixel.set_green(0x02);
        pixel.set_blue(0x03);
        pixel.set_alpha(0x04);
        assert_eq!(pixel.as_raw().into_inner(), 0x0403_0201);
    }
}
