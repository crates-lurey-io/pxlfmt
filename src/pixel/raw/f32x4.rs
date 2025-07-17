use crate::pixel::raw::RawPixel;

/// A raw pixel value represented as 4 32-bit floating point numbers.
///
/// Each channel is stored as one of the four 32-bit components.
///
/// ## Layout
///
/// This struct is identical to a `[f32; 4]` in memory (`#[repr(transparent)]`).
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct F32x4([f32; 4]);

impl F32x4 {
    /// Creates a new raw pixel value with all channels set to zero.
    #[must_use]
    pub const fn new_zeroed() -> Self {
        Self([0.0; 4])
    }

    /// Creates a new raw pixel value from the given array of 4 32-bit floating point numbers.
    #[must_use]
    pub const fn from_f32x4(value: [f32; 4]) -> Self {
        Self(value)
    }

    /// Creates a new raw pixel value from the given 4 32-bit floating point numbers.
    #[must_use]
    pub const fn from_channels(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self([a, b, c, d])
    }
}

impl From<[f32; 4]> for F32x4 {
    fn from(value: [f32; 4]) -> Self {
        Self(value)
    }
}

impl RawPixel for F32x4 {
    const DEFAULT: Self = Self([0.0; 4]);
    type Storage = [f32; 4];
    type Channel = f32;

    fn get_channel(&self, offset: usize) -> Self::Channel {
        self.0[offset]
    }

    fn set_channel(&mut self, offset: usize, value: Self::Channel) -> &mut Self {
        self.0[offset] = value;
        self
    }

    fn as_inner(&self) -> &Self::Storage {
        &self.0
    }

    fn into_inner(self) -> Self::Storage {
        self.0
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for F32x4 {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for F32x4 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck_cast() {
        let pixel = F32x4::from([0.0, 1.0, 2.0, 3.0]);
        let binding = [pixel];
        let bytes: &[u8] = bytemuck::cast_slice(&binding);

        #[rustfmt::skip]
        assert_eq!(
            bytes,
            &[
                0x00, 0x00, 0x00, 0x00, 
                0x00, 0x00, 0x80, 0x3F, 
                0x00, 0x00, 0x00, 0x40, 
                0x00, 0x00, 0x40, 0x40
            ]
        );
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn as_inner() {
        let pixel = F32x4::from([0.0, 1.0, 2.0, 3.0]);
        assert_eq!(pixel.as_inner(), &[0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn into_inner() {
        let pixel = F32x4::from([0.0, 1.0, 2.0, 3.0]);
        assert_eq!(pixel.into_inner(), [0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn new_zero() {
        let pixel = F32x4::new_zeroed();
        assert_eq!(pixel.as_inner(), &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn from_f32x4() {
        let pixel = F32x4::from_f32x4([0.0, 1.0, 2.0, 3.0]);
        assert_eq!(pixel.as_inner(), &[0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn from_channels() {
        let pixel = F32x4::from_channels(0.0, 1.0, 2.0, 3.0);
        assert_eq!(pixel.as_inner(), &[0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn get_channel() {
        let pixel = F32x4::from([0.0, 1.0, 2.0, 3.0]);
        assert_eq!(pixel.get_channel(0), 0.0);
        assert_eq!(pixel.get_channel(1), 1.0);
        assert_eq!(pixel.get_channel(2), 2.0);
        assert_eq!(pixel.get_channel(3), 3.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn set_channel() {
        let mut pixel = F32x4::from([0.0, 1.0, 2.0, 3.0]);
        pixel.set_channel(0, 4.0);
        pixel.set_channel(1, 5.0);
        pixel.set_channel(2, 6.0);
        pixel.set_channel(3, 7.0);
        assert_eq!(pixel.get_channel(0), 4.0);
        assert_eq!(pixel.get_channel(1), 5.0);
        assert_eq!(pixel.get_channel(2), 6.0);
        assert_eq!(pixel.get_channel(3), 7.0);
    }
}
