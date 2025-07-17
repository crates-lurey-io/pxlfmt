//! [`Pixel`] is the organization and characteristics of pixel data in memory.

use crate::pixel::raw::RawPixel;
use core::{
    fmt::{LowerHex, UpperHex},
    marker::PhantomData,
};

pub mod raw;

/// Describes the organization and characteristics of pixel data in memory.
#[allow(private_bounds)]
pub trait Format: 'static + Copy + crate::internal::Sealed {
    /// Used to represent the raw pixel data in memory (e.g., [`U32x8888`][]).
    ///
    /// [`U32x8888`]: crate::pixel::raw::U32x8888
    type RawPixel: RawPixel;

    /// The type representing the channels of the pixel (e.g., [`Rgba`][]).
    ///
    /// [`Rgba`]: crate::formats::rgba::Rgba
    type Channels: Copy + Eq + Ord;
}

/// A pixel value in a specific format.
///
/// This struct wraps a raw pixel value and provides methods to access and modify its channels.
///
/// ## Layout
///
/// This struct is always represnted directly as the raw pixel value, with no additional padding.
///
/// ## Example
///
/// ```rust
/// use pxlfmt::{formats::rgba::Abgr8888, pixel::Pixel};
///
/// let pixel = Pixel::<Abgr8888>::with_rgba(0xFF, 0x00, 0x00, 0xFF);
/// assert_eq!(pixel.red(), 0xFF);
/// assert_eq!(pixel.green(), 0x00);
/// assert_eq!(pixel.blue(), 0x00);
/// assert_eq!(pixel.alpha(), 0xFF);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Pixel<F: Format> {
    raw: F::RawPixel,
    format: PhantomData<F>,
}

impl<F: Format> Default for Pixel<F> {
    fn default() -> Self {
        Self::from_raw(F::RawPixel::DEFAULT)
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<F> bytemuck::Zeroable for Pixel<F>
where
    F: Format,
    F::RawPixel: bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<F> bytemuck::Pod for Pixel<F>
where
    F: Format + Copy,
    F::RawPixel: bytemuck::Pod,
{
}

impl<F: Format> Pixel<F> {
    /// Creates a new pixel by converting a value that can be converted into the raw pixel type.
    pub fn new(raw: impl Into<F::RawPixel>) -> Self {
        Self::from_raw(raw.into())
    }

    /// Creates a new pixel from a raw pixel value.
    pub const fn from_raw(raw: F::RawPixel) -> Self {
        Self {
            raw,
            format: PhantomData,
        }
    }

    /// Returns a reference to the raw pixel value.
    pub const fn as_raw(&self) -> &F::RawPixel {
        &self.raw
    }

    /// Returns a mutable reference to the raw pixel value.
    pub fn as_raw_mut(&mut self) -> &mut F::RawPixel {
        &mut self.raw
    }

    /// Consumes the pixel and returns the raw pixel value.
    pub fn into_raw(self) -> F::RawPixel {
        self.raw
    }
}

impl<F> AsRef<F::RawPixel> for Pixel<F>
where
    F: Format,
{
    fn as_ref(&self) -> &F::RawPixel {
        self.as_raw()
    }
}

impl<F> AsMut<F::RawPixel> for Pixel<F>
where
    F: Format,
{
    fn as_mut(&mut self) -> &mut F::RawPixel {
        self.as_raw_mut()
    }
}

impl<F> UpperHex for Pixel<F>
where
    F: Format,
    F::RawPixel: UpperHex,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.raw.fmt(f)
    }
}

impl<F> LowerHex for Pixel<F>
where
    F: Format,
    F::RawPixel: LowerHex,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.raw.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use alloc::format;

    #[test]
    fn upper_hex() {
        let pixel = Pixel::<crate::formats::rgba::Rgba8888>::new(0xFF00_00FF);
        assert_eq!(format!("{pixel:X}"), "FF0000FF");
    }

    #[test]
    fn lower_hex() {
        let pixel = Pixel::<crate::formats::rgba::Rgba8888>::new(0xFF00_00FF);
        assert_eq!(format!("{pixel:x}"), "ff0000ff");
    }

    #[test]
    fn u32_pixel_is_copy() {
        fn is_copy<T: Copy>() {}
        is_copy::<Pixel<crate::formats::rgba::Rgba8888>>();
        is_copy::<Pixel<crate::formats::rgba::Abgr8888>>();
    }

    #[test]
    #[cfg(feature = "bytemuck")]
    fn u32_pixel_cast_bytemuck() {
        let pixels = [Pixel::<crate::formats::rgba::Rgba8888>::new(0xFF00_00FF)];
        let bytes: &[u8] = bytemuck::cast_slice(&pixels);
        assert_eq!(bytes, &[0xFF, 0x00, 0x00, 0xFF]);

        let pixels_back: &[Pixel<crate::formats::rgba::Rgba8888>] = bytemuck::cast_slice(bytes);
        assert_eq!(pixels_back.len(), 1);
        assert_eq!(pixels_back[0].as_raw().into_inner(), 0xFF00_00FF);
    }

    #[test]
    fn into_raw() {
        let pixel = Pixel::<crate::formats::rgba::Rgba8888>::with_rgba(0xFF, 0x00, 0x00, 0xFF);
        assert_eq!(pixel.into_raw().into_inner(), 0xFF00_00FF);
    }

    #[test]
    fn as_raw() {
        let pixel = Pixel::<crate::formats::rgba::Rgba8888>::with_rgba(0xFF, 0x00, 0x00, 0xFF);
        assert_eq!(pixel.as_raw().into_inner(), 0xFF00_00FF);
    }

    #[test]
    fn as_raw_mut() {
        let mut pixel = Pixel::<crate::formats::rgba::Rgba8888>::with_rgba(0xFF, 0x00, 0x00, 0xFF);
        pixel.as_raw_mut().set_channel(0, 0x01);
        assert_eq!(pixel.as_raw().into_inner(), 0xFF00_0001);
    }

    #[test]
    fn as_ref() {
        let pixel = Pixel::<crate::formats::rgba::Rgba8888>::with_rgba(0xFF, 0x00, 0x00, 0xFF);
        assert_eq!(pixel.as_ref().into_inner(), 0xFF00_00FF);
    }

    #[test]
    fn as_mut() {
        let mut pixel = Pixel::<crate::formats::rgba::Rgba8888>::with_rgba(0xFF, 0x00, 0x00, 0xFF);
        pixel.as_mut().set_channel(0, 0x01);
        assert_eq!(pixel.as_raw().into_inner(), 0xFF00_0001);
    }

    #[test]
    fn default() {
        let pixel = Pixel::<crate::formats::rgba::Rgba8888>::default();
        assert_eq!(pixel.as_raw().into_inner(), 0x0000_0000);
    }

    #[test]
    fn is_copy() {
        trait DrawPixel<F: Format> {
            fn draw_10x(&mut self, color: Pixel<F>);
        }

        struct Canvas<F: Format> {
            pixels: alloc::vec::Vec<Pixel<F>>,
        }

        impl<F: Format> DrawPixel<F> for Canvas<F> {
            fn draw_10x(&mut self, color: Pixel<F>) {
                for _ in 0..10 {
                    self.pixels.push(color);
                }
            }
        }

        let mut canvas = Canvas::<crate::formats::rgba::Rgba8888> {
            pixels: alloc::vec::Vec::new(),
        };
        let color = Pixel::<crate::formats::rgba::Rgba8888>::with_rgba(0xFF, 0x00, 0x00, 0xFF);
        canvas.draw_10x(color);
    }
}
