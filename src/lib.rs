//! A foundational crate for type-safe, zero-cost pixel format manipulation.
//!
//! # Example
//!
//! ```rust
//! use pxlfmt::{Pixel, Abgr8888, RgbaFormat};
//!
//! // A single pixel in the Abgr8888 format, wrapping a u32 value.
//! let mut pixel = Pixel::<Abgr8888>::from(0xFF00_00FF);
//!
//! // The API provides convenient accessors based on the format.
//! assert_eq!(pixel.red(), 0xFF);
//! assert_eq!(pixel.alpha(), 0xFF);
//!
//! // Modify the pixel's channels.
//! pixel.set_green(0x88);
//! pixel.set_blue(0x44);
//!
//! // The underlying raw value reflects the changes.
//! assert_eq!(pixel.raw().raw(), 0xFF4488FF);
//! ```

#![no_std]

use core::marker::PhantomData;

use crate::{
    internal::Sealed,
    raw::{RawPixel, U32x8888},
};

pub mod raw;
pub mod uint;

pub(crate) mod internal;

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

/// Describes the organization and characteristics of pixel data in memory.
///
/// The following associated types are defined:
/// - `RawPixel`: The type used to represent the raw pixel data (e.g., [`U32x8888`]).
/// - `Channels`: The type representing the channels of the pixel (e.g., [`Rgba`]).
#[allow(private_bounds)]
pub trait Format: Sealed {
    type RawPixel: RawPixel;
    type Channels: Copy + Eq + Ord;
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

/// A 32-bit ABGR pixel format with four 8-bit channels.
///
/// This format is used to represent pixels in the ABGR order:
/// - `A`lpha (8 bits)
/// - `B`lue (8 bits)
/// - `G`reen (8 bits)
/// - `R`ed (8 bits)
///
/// The pixel is represented as a 32-bit unsigned integer, where each channel occupies 8 bits.
pub struct Abgr8888;

impl Sealed for Abgr8888 {}
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

/// A pixel value in a specific format.
///
/// This struct wraps a raw pixel value and provides methods to access and modify its channels.
///
/// # Layout
///
/// This struct is always represnted directly as the raw pixel value, with no additional padding.
///
/// # Example
///
/// ```rust
/// use pxlfmt::{Pixel, Abgr8888};
///
/// let pixel = Pixel::<Abgr8888>::new(0xFF00_00FF);
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

impl<F: Format> Pixel<F> {
    /// Creates a new pixel with the given raw value.
    pub fn new(raw: impl Into<F::RawPixel>) -> Self {
        Self::from_raw_pixel(raw.into())
    }

    #[inline]
    const fn from_raw_pixel(raw: F::RawPixel) -> Self {
        Self {
            raw,
            format: PhantomData,
        }
    }

    /// Returns the raw pixel value.
    pub const fn raw(&self) -> &F::RawPixel {
        &self.raw
    }

    /// Casts a slice of raw pixel values to a slice of `Pixel<F>`.
    pub fn as_slice(buffer: &[F::RawPixel]) -> &[Self] {
        // The cast from a raw pointer to a Pixel pointer is safe because of #[repr(transparent)].
        unsafe { core::slice::from_raw_parts(buffer.as_ptr().cast::<Self>(), buffer.len()) }
    }

    /// Casts a mutable slice of raw pixel values to a mutable slice of `Pixel<F>`.
    pub fn as_slice_mut(buffer: &mut [F::RawPixel]) -> &mut [Self] {
        unsafe { core::slice::from_raw_parts_mut(buffer.as_mut_ptr().cast::<Self>(), buffer.len()) }
    }
}

impl<F: RgbaFormat> Pixel<F> {
    /// Returns the red channel value of the pixel.
    pub fn red(&self) -> <F::RawPixel as RawPixel>::Channel {
        F::get_red(&self.raw)
    }

    /// Sets the red channel value of the pixel.
    pub fn set_red(&mut self, value: <F::RawPixel as RawPixel>::Channel) {
        F::set_red(&mut self.raw, value);
    }

    /// Returns the green channel value of the pixel.
    pub fn green(&self) -> <F::RawPixel as RawPixel>::Channel {
        F::get_green(&self.raw)
    }

    /// Sets the green channel value of the pixel.
    pub fn set_green(&mut self, value: <F::RawPixel as RawPixel>::Channel) {
        F::set_green(&mut self.raw, value);
    }

    /// Returns the blue channel value of the pixel.
    pub fn blue(&self) -> <F::RawPixel as RawPixel>::Channel {
        F::get_blue(&self.raw)
    }

    /// Sets the blue channel value of the pixel.
    pub fn set_blue(&mut self, value: <F::RawPixel as RawPixel>::Channel) {
        F::set_blue(&mut self.raw, value);
    }

    /// Returns the alpha channel value of the pixel.
    pub fn alpha(&self) -> <F::RawPixel as RawPixel>::Channel {
        F::get_alpha(&self.raw)
    }

    /// Sets the alpha channel value of the pixel.
    pub fn set_alpha(&mut self, value: <F::RawPixel as RawPixel>::Channel) {
        F::set_alpha(&mut self.raw, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abgr8888() {
        let mut pixel: Pixel<Abgr8888> = Pixel::new(U32x8888::new(0xFF00_00FF));
        assert_eq!(pixel.red(), 0xFF);
        assert_eq!(pixel.green(), 0x00);
        assert_eq!(pixel.blue(), 0x00);
        assert_eq!(pixel.alpha(), 0xFF);

        pixel.set_red(0x01);
        pixel.set_green(0x02);
        pixel.set_blue(0x03);
        pixel.set_alpha(0x04);
        assert_eq!(pixel.raw().raw(), 0x0403_0201);
    }

    #[test]
    fn cast_slice() {
        let buffer = [U32x8888::new(0xFF00_00FF)];
        let pixels = Pixel::<Abgr8888>::as_slice(&buffer);
        assert_eq!(pixels.len(), 1);
        assert_eq!(pixels[0].red(), 0xFF);
        assert_eq!(pixels[0].green(), 0x00);
        assert_eq!(pixels[0].blue(), 0x00);
        assert_eq!(pixels[0].alpha(), 0xFF);
    }

    #[test]
    fn cast_slice_mut() {
        let mut buffer = [U32x8888::new(0xFF00_00FF)];
        let pixels = Pixel::<Abgr8888>::as_slice_mut(&mut buffer);
        assert_eq!(pixels.len(), 1);
        assert_eq!(pixels[0].red(), 0xFF);
        assert_eq!(pixels[0].green(), 0x00);
        assert_eq!(pixels[0].blue(), 0x00);
        assert_eq!(pixels[0].alpha(), 0xFF);

        pixels[0].set_red(0x01);
        pixels[0].set_green(0x02);
        pixels[0].set_blue(0x03);
        pixels[0].set_alpha(0x04);
        assert_eq!(buffer[0].raw(), 0x0403_0201);
    }
}
