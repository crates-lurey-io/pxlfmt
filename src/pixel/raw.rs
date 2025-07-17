//! Raw pixel data representation.
//!
//! This module provides the lowest-level building blocks for the crate, defining how raw pixel
//! values are stored and manipulated before a specific format is applied. The central component is
//! the [`RawPixel`] trait, which abstracts over packed data types like `u32` or `u16` to allow
//! generic channel access.
//!
//! # Example
//!
//! The following example demonstrates how to use [`U32x8888`] and [`RawPixel`]s methods:
//!
//! ```rust
//! use pxlfmt::pixel::raw::{RawPixel, U32x8888};
//!
//! let mut raw_pixel = U32x8888::from(0xFF00_00FF);
//! assert_eq!(raw_pixel.get_channel(0), 0xFF); // Red channel
//! assert_eq!(raw_pixel.get_channel(1), 0x00); // Green channel
//!
//! raw_pixel.set_channel(1, 0xFF); // Set Green channel to 0xFF
//! assert_eq!(raw_pixel.get_channel(1), 0xFF); // Now Green
//!
//! // The underlying raw value can be accessed directly
//! assert_eq!(raw_pixel.into_inner(), 0xFF00_FFFF);
//! ```

use core::mem;

mod f32x4;
pub use f32x4::F32x4;

mod u32x8888;
pub use u32x8888::U32x8888;

/// A trait for types that can represent a raw pixel value.
///
/// This trait provides methods to get and set the individual channels of a pixel.
///
/// ## Example
///
/// ```rust
/// use pxlfmt::pixel::raw::RawPixel;
///
/// /// A raw pixel value represented as a 32-bit unsigned integer.
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
/// #[repr(transparent)]
/// pub struct U32x8888(u32);
///
/// impl RawPixel for U32x8888 {
///   const DEFAULT: Self = Self(0);
///   type Channel = u8;
///   type Storage = u32;
///
///   fn get_channel(&self, offset: usize) -> Self::Channel {
///     (self.0 >> (offset * 8) & 0xFF) as u8
///   }
///
///   fn set_channel(&mut self, offset: usize, value: Self::Channel) -> &mut Self {
///     let mask = !(0xFF << (offset * 8));
///     self.0 = (self.0 & mask) | (u32::from(value) << (offset * 8));
///     self
///   }
///
///   fn as_inner(&self) -> &Self::Storage {
///     &self.0
///   }
///
///   fn into_inner(self) -> Self::Storage {
///     self.0
///   }
/// }
///
/// impl From<u32> for U32x8888 {
///   fn from(value: u32) -> Self {
///     Self(value)
///   }
/// }
/// ```
pub trait RawPixel: From<Self::Storage> {
    /// The default value type for the pixel.
    const DEFAULT: Self;

    /// The underlying type used to store the entire value of the pixel's data.
    ///
    /// For example, for `U32x8888`, this would be `u32`.
    type Storage;

    /// The underlying type used to represent each of the pixel's channels.
    ///
    /// For example, for `U32x8888`, this would be `u8`.
    type Channel;

    /// How many channels this pixel has.
    ///
    /// Defaults to the size of the storage divided by the size of a channel.
    const CHANNELS: usize = mem::size_of::<Self::Storage>() / mem::size_of::<Self::Channel>();

    /// Gets the channel at the provided offset.
    ///
    /// The offset is based on the pixel's channel order, where `0` is the first channel.
    ///
    /// ## Panics
    ///
    /// If `offset` is out of bounds for the pixel's channel count, this method will panic.
    #[must_use]
    fn get_channel(&self, offset: usize) -> Self::Channel;

    /// Sets the channel at the provided offset to the given value.
    ///
    /// The offset is based on the pixel's channel order, where `0` is the first channel.
    ///
    /// ## Panics
    ///
    /// If `offset` is out of bounds for the pixel's channel count, this method will panic.
    fn set_channel(&mut self, offset: usize, value: Self::Channel) -> &mut Self;

    /// Returns a new pixel with the channel at the provided offset set to the given value.
    ///
    /// The original pixel is consumed.
    ///
    /// The offset is based on the pixel's channel order, where `0` is the first channel.
    ///
    /// ## Panics
    ///
    /// If `offset` is out of bounds for the pixel's channel count, this method will panic.
    #[must_use]
    fn with_channel(mut self, offset: usize, value: Self::Channel) -> Self {
        self.set_channel(offset, value);
        self
    }

    /// Returns the underlying raw value.
    #[must_use]
    fn as_inner(&self) -> &Self::Storage;

    /// Consumes the pixel and returns the underlying raw value.
    #[must_use]
    fn into_inner(self) -> Self::Storage;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_channel() {
        let mut pixel = U32x8888::from(0xFF00_00FF);
        pixel = pixel.with_channel(1, 0xFF);
        assert_eq!(pixel.get_channel(1), 0xFF); // Green channel
    }
}
