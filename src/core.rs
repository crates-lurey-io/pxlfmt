//! Core pixel format traits and types.

use crate::raw::RawPixel;
use core::{
    fmt::{LowerHex, UpperHex},
    marker::PhantomData,
};

/// Describes the organization and characteristics of pixel data in memory.
///
/// The following associated types are defined:
/// - `RawPixel`: The type used to represent the raw pixel data (e.g., [`U32x8888`][]).
/// - `Channels`: The type representing the channels of the pixel (e.g., [`Rgba`][]).
///
/// [`U32x8888`]: crate::raw::U32x8888
/// [`Rgba`]: crate::formats::rgba::Rgba
#[allow(private_bounds)]
pub trait Format: crate::internal::Sealed {
    type RawPixel: RawPixel;
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
/// use pxlfmt::{core::Pixel, formats::rgba::Abgr8888};
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
    pub(crate) raw: F::RawPixel,
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
    pub const fn as_raw(&self) -> &F::RawPixel {
        &self.raw
    }

    /// Casts a slice of raw pixel values to a slice of `Pixel<F>`.
    pub fn as_slice(buffer: &[F::RawPixel]) -> &[Self] {
        // The cast from a raw pointer to a Pixel pointer is safe because of #[repr(transparent)].
        unsafe { core::slice::from_raw_parts(buffer.as_ptr().cast::<Self>(), buffer.len()) }
    }

    /// Casts a mutable slice of raw pixel alues to a mutable slice of `Pixel<F>`.
    pub fn as_slice_mut(buffer: &mut [F::RawPixel]) -> &mut [Self] {
        unsafe { core::slice::from_raw_parts_mut(buffer.as_mut_ptr().cast::<Self>(), buffer.len()) }
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
}
