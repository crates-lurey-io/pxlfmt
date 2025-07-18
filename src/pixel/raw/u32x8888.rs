use core::fmt::{LowerHex, UpperHex};

use crate::pixel::raw::RawPixel;

/// A raw pixel value represented as a 32-bit unsigned integer.
///
/// Each channel is stored as one of the four 8-bit components.
///
/// ## Layout
///
/// This struct is identical to a `u32` in memory (`#[repr(transparent)]`).
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct U32x8888(u32);

impl U32x8888 {
    /// Creates a new raw pixel value with all channels set to zero.
    #[must_use]
    pub const fn new_zeroed() -> Self {
        Self(0)
    }

    /// Creates a new raw pixel value from the given 32-bit unsigned integer.
    #[must_use]
    pub const fn from_u32(value: u32) -> Self {
        Self(value)
    }

    /// Creates a new raw pixel value from the given 4 8-bit channels.
    #[must_use]
    pub const fn from_channels(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(a as u32 | (b as u32) << 8 | (c as u32) << 16 | (d as u32) << 24)
    }
}

impl From<u32> for U32x8888 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl RawPixel for U32x8888 {
    const DEFAULT: Self = Self(0);
    type Storage = u32;
    type Channel = u8;

    unsafe fn get_channel_unchecked(&self, offset: usize) -> Self::Channel {
        (self.0 >> (offset * 8) & 0xFF) as u8
    }

    unsafe fn set_channel_unchecked(&mut self, offset: usize, value: Self::Channel) -> &mut Self {
        let mask = !(0xFF << (offset * 8));
        self.0 = (self.0 & mask) | (u32::from(value) << (offset * 8));
        self
    }

    fn as_inner(&self) -> &Self::Storage {
        &self.0
    }

    fn into_inner(self) -> Self::Storage {
        self.0
    }
}

impl UpperHex for U32x8888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:X}", self.into_inner())
    }
}

impl LowerHex for U32x8888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:x}", self.into_inner())
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for U32x8888 {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for U32x8888 {}

#[cfg(test)]
mod tets {
    use super::*;

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck_cast() {
        let pixel = U32x8888::from(0xFF00_00FF);
        let binding = [pixel];
        let bytes: &[u8] = bytemuck::cast_slice(&binding);
        assert_eq!(bytes, &[0xFF, 0x00, 0x00, 0xFF]);
    }

    #[test]
    fn as_inner() {
        let pixel = U32x8888::from(0xFF00_00FF);
        assert_eq!(pixel.as_inner(), &0xFF00_00FF);
    }

    #[test]
    fn into_inner() {
        let pixel = U32x8888::from(0xFF00_00FF);
        assert_eq!(pixel.into_inner(), 0xFF00_00FF);
    }

    #[test]
    fn new_zero() {
        let pixel = U32x8888::new_zeroed();
        assert_eq!(pixel.into_inner(), 0);
    }

    #[test]
    fn from_u32() {
        let pixel = U32x8888::from_u32(0xFF00_00FF);
        assert_eq!(pixel.into_inner(), 0xFF00_00FF);
    }

    #[test]
    fn from_channels() {
        let pixel = U32x8888::from_channels(0xFF, 0x00, 0x00, 0xFF);
        assert_eq!(pixel.into_inner(), 0xFF00_00FF);
    }
}
