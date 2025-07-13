//! Foundational traits and types for raw pixel data representation.
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
//! use pxlfmt::raw::{RawPixel, U32x8888};
//!
//! let mut raw_pixel = U32x8888::new(0xFF00_00FF);
//! assert_eq!(raw_pixel.get_channel(0), 0xFF); // Red channel
//! assert_eq!(raw_pixel.get_channel(1), 0x00); // Green channel
//!
//! raw_pixel.set_channel(1, 0xFF); // Set Green channel to 0xFF
//! assert_eq!(raw_pixel.get_channel(1), 0xFF); // Now Green
//!
//! // The underlying raw value can be accessed directly
//! assert_eq!(raw_pixel.raw(), 0xFF00_FFFF);
//! ```

/// A trait for types that can represent a raw pixel value.
///
/// This trait provides methods to get and set the individual channels of a pixel.
///
/// ## Example
///
/// ```rust
/// use pxlfmt::raw::RawPixel;
///
/// /// A raw pixel value represented as a 32-bit unsigned integer.
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
/// #[repr(transparent)]
/// pub struct U32x8888(u32);
///
/// impl RawPixel for U32x8888 {
///   type Channel = u8;
///   type Value = u32;
///
///   fn get_channel(&self, offset: usize) -> Self::Channel {
///     (self.0 >> (offset * 8) & 0xFF) as u8
///   }
///
///   fn set_channel(&mut self, offset: usize, value: Self::Channel) {
///     let mask = !(0xFF << (offset * 8));
///     self.0 = (self.0 & mask) | (u32::from(value) << (offset * 8));
///   }
/// }
///
/// impl From<u32> for U32x8888 {
///   fn from(value: u32) -> Self {
///     Self(value)
///   }
/// }
/// ```
pub trait RawPixel: From<Self::Value> {
    /// The underlying type used to the entire value of the pixel.
    type Value;

    /// The underlying type used to represent the pixel's channels.
    ///
    /// This type should be a primitive type that can hold the pixel's raw data.
    type Channel;

    /// Gets the channel at the provided offset.
    ///
    /// The offset is based on the pixel's channel order, where `0` is the first channel.
    #[must_use]
    fn get_channel(&self, offset: usize) -> Self::Channel;

    /// Sets the channel at the provided offset to the given value.
    ///
    /// The offset is based on the pixel's channel order, where `0` is the first channel.
    fn set_channel(&mut self, offset: usize, value: Self::Channel);
}

/// A raw pixel value represented as a 32-bit unsigned integer.
///
/// Each channel is stored as one of the four 8-bit components.
///
/// ## Layout
///
/// This struct is identical to a `u32` in memory (`#[repr(transparent)]`).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct U32x8888(u32);

impl U32x8888 {
    /// Creates a new raw pixel value from the given 32-bit unsigned integer.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the underlying raw value.
    #[must_use]
    pub const fn raw(&self) -> u32 {
        self.0
    }
}

impl From<u32> for U32x8888 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl RawPixel for U32x8888 {
    type Value = u32;
    type Channel = u8;

    fn get_channel(&self, offset: usize) -> Self::Channel {
        (self.0 >> (offset * 8) & 0xFF) as u8
    }

    fn set_channel(&mut self, offset: usize, value: Self::Channel) {
        let mask = !(0xFF << (offset * 8));
        self.0 = (self.0 & mask) | (u32::from(value) << (offset * 8));
    }
}
