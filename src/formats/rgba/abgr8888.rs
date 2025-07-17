use crate::{
    formats::rgba::{Rgba, RgbaFormat},
    pixel::{Format, raw::U32x8888},
};

/// A 32-bit ABGR pixel format with four 8-bit channels.
///
/// This format is used to represent pixels in the ABGR order:
/// - `A`lpha (8 bits)
/// - `B`lue (8 bits)
/// - `G`reen (8 bits)
/// - `R`ed (8 bits)
///
/// The pixel is represented as a 32-bit unsigned integer, where each channel occupies 8 bits.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Abgr8888 {}

impl crate::internal::Sealed for Abgr8888 {}
impl Format for Abgr8888 {
    type RawPixel = U32x8888;
    type Channels = Rgba;
}
impl RgbaFormat for Abgr8888 {
    const ALPHA_OFFSET: usize = 3;
    const BLUE_OFFSET: usize = 2;
    const GREEN_OFFSET: usize = 1;
    const RED_OFFSET: usize = 0;
}

#[cfg(test)]
mod tests {
    use crate::{pixel::Pixel, pixel::raw::RawPixel};

    use super::*;

    #[test]
    fn abgr8888() {
        let mut pixel: Pixel<Abgr8888> = Pixel::new(U32x8888::from(0xFF00_00FF));
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

    #[cfg(feature = "bytemuck")]
    #[test]
    fn cast_slice_bytemuck() {
        let buffer = [U32x8888::from(0xFF00_00FF)];
        let pixels = bytemuck::cast_slice::<_, Pixel<Abgr8888>>(&buffer);
        assert_eq!(pixels.len(), 1);
        assert_eq!(pixels[0].red(), 0xFF);
        assert_eq!(pixels[0].green(), 0x00);
        assert_eq!(pixels[0].blue(), 0x00);
        assert_eq!(pixels[0].alpha(), 0xFF);
    }

    #[cfg(feature = "bytemuck")]
    #[test]
    fn cast_slice_mut() {
        let mut buffer = [U32x8888::from(0xFF00_00FF)];
        let pixels = bytemuck::cast_slice_mut::<_, Pixel<Abgr8888>>(&mut buffer);
        assert_eq!(pixels.len(), 1);
        assert_eq!(pixels[0].red(), 0xFF);
        assert_eq!(pixels[0].green(), 0x00);
        assert_eq!(pixels[0].blue(), 0x00);
        assert_eq!(pixels[0].alpha(), 0xFF);

        pixels[0].set_red(0x01);
        pixels[0].set_green(0x02);
        pixels[0].set_blue(0x03);
        pixels[0].set_alpha(0x04);
        assert_eq!(buffer[0].into_inner(), 0x0403_0201);
    }
}
