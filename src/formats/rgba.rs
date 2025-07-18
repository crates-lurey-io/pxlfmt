//! RGBA pixel formats.

mod abgr8888;
pub use abgr8888::Abgr8888;

mod float_rgba;
pub use float_rgba::FloatRgba;

mod rgba8888;
pub use rgba8888::Rgba8888;

use crate::pixel::{Format, Pixel, raw::RawPixel};

/// Channels representing `R`ed, `G`reen, `B`lue, and `A`lpha components of a pixel.
///
/// Used in pixel formats that support RGBA color representation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Rgba {
    Red,
    Green,
    Blue,
    Alpha,
}

/// A pixel format with red, green, blue, and alpha channels.
///
/// The offsets of individual channels in the pixel's raw representation are defined by this trait:
/// - `RED_OFFSET`: Offset of the red channel.
/// - `GREEN_OFFSET`: Offset of the green channel.
/// - `BLUE_OFFSET`: Offset of the blue channel.
/// - `ALPHA_OFFSET`: Offset of the alpha channel.
pub trait RgbaFormat: Format<Channels = Rgba> {
    /// The offset of the red channel in the pixel's raw representation.
    const RED_OFFSET: usize;

    /// The offset of the green channel in the pixel's raw representation.
    const GREEN_OFFSET: usize;

    /// The offset of the blue channel in the pixel's raw representation.
    const BLUE_OFFSET: usize;

    /// The offset of the alpha channel in the pixel's raw representation.
    const ALPHA_OFFSET: usize;

    /// Returns the red channel value of a pixel.
    fn get_red(pixel: &Self::RawPixel) -> <Self::RawPixel as RawPixel>::Channel {
        pixel.get_channel(Self::RED_OFFSET)
    }

    /// Sets the red channel value of a pixel.
    fn set_red(pixel: &mut Self::RawPixel, value: <Self::RawPixel as RawPixel>::Channel) {
        pixel.set_channel(Self::RED_OFFSET, value);
    }

    /// Returns the green channel value of a pixel.
    fn get_green(pixel: &Self::RawPixel) -> <Self::RawPixel as RawPixel>::Channel {
        pixel.get_channel(Self::GREEN_OFFSET)
    }

    /// Sets the green channel value of a pixel.
    fn set_green(pixel: &mut Self::RawPixel, value: <Self::RawPixel as RawPixel>::Channel) {
        pixel.set_channel(Self::GREEN_OFFSET, value);
    }

    /// Returns the blue channel value of a pixel.
    fn get_blue(pixel: &Self::RawPixel) -> <Self::RawPixel as RawPixel>::Channel {
        pixel.get_channel(Self::BLUE_OFFSET)
    }

    /// Sets the blue channel value of a pixel.
    fn set_blue(pixel: &mut Self::RawPixel, value: <Self::RawPixel as RawPixel>::Channel) {
        pixel.set_channel(Self::BLUE_OFFSET, value);
    }

    /// Returns the alpha channel value of a pixel.
    fn get_alpha(pixel: &Self::RawPixel) -> <Self::RawPixel as RawPixel>::Channel {
        pixel.get_channel(Self::ALPHA_OFFSET)
    }

    /// Sets the alpha channel value of a pixel.
    fn set_alpha(pixel: &mut Self::RawPixel, value: <Self::RawPixel as RawPixel>::Channel) {
        pixel.set_channel(Self::ALPHA_OFFSET, value);
    }
}

impl<F: RgbaFormat> Pixel<F> {
    /// Returns the red channel value of the pixel.
    pub fn red(&self) -> <F::RawPixel as RawPixel>::Channel {
        F::get_red(self.as_raw())
    }

    /// Sets the red channel value of the pixel.
    pub fn set_red(&mut self, value: <F::RawPixel as RawPixel>::Channel) -> &mut Self {
        F::set_red(self.as_raw_mut(), value);
        self
    }

    /// Returns the green channel value of the pixel.
    pub fn green(&self) -> <F::RawPixel as RawPixel>::Channel {
        F::get_green(self.as_raw())
    }

    /// Sets the green channel value of the pixel.
    pub fn set_green(&mut self, value: <F::RawPixel as RawPixel>::Channel) -> &mut Self {
        F::set_green(self.as_raw_mut(), value);
        self
    }

    /// Returns the blue channel value of the pixel.
    pub fn blue(&self) -> <F::RawPixel as RawPixel>::Channel {
        F::get_blue(self.as_raw())
    }

    /// Sets the blue channel value of the pixel.
    pub fn set_blue(&mut self, value: <F::RawPixel as RawPixel>::Channel) -> &mut Self {
        F::set_blue(self.as_raw_mut(), value);
        self
    }

    /// Returns the alpha channel value of the pixel.
    pub fn alpha(&self) -> <F::RawPixel as RawPixel>::Channel {
        F::get_alpha(self.as_raw())
    }

    /// Sets the alpha channel value of the pixel.
    pub fn set_alpha(&mut self, value: <F::RawPixel as RawPixel>::Channel) -> &mut Self {
        F::set_alpha(self.as_raw_mut(), value);
        self
    }
}

impl<F> Pixel<F>
where
    F: RgbaFormat,
{
    /// Creates a new pixel from RGBA channel values.
    ///
    /// This method initializes the pixel with the specified red, green, blue, and alpha values.
    pub fn with_rgba(
        r: <F::RawPixel as RawPixel>::Channel,
        g: <F::RawPixel as RawPixel>::Channel,
        b: <F::RawPixel as RawPixel>::Channel,
        a: <F::RawPixel as RawPixel>::Channel,
    ) -> Self {
        let mut pixel = Self::from_raw(F::RawPixel::DEFAULT);
        pixel.set_red(r).set_green(g).set_blue(b).set_alpha(a);
        pixel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_rgba() {
        let pixel = Pixel::<Rgba8888>::with_rgba(0xFF, 0x00, 0x00, 0xFF);
        assert_eq!(pixel.red(), 0xFF);
        assert_eq!(pixel.green(), 0x00);
        assert_eq!(pixel.blue(), 0x00);
        assert_eq!(pixel.alpha(), 0xFF);
    }
}
