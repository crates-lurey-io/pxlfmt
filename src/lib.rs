//! A foundational crate for type-safe, zero-cost pixel format manipulation.
//!
//! # Example
//!
//! ```rust
//! use pxlfmt::prelude::*;
//!
//! // A single pixel in the Rgba8888 format, wrapping a u32 value.
//! let mut pixel = Pixel::<Rgba8888>::new(0xFF00_00FF);
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
//! assert_eq!(pixel.as_raw().into_inner(), 0xFF4488FF);
//! ```

#![no_std]

pub mod core;
pub mod formats;
pub mod prelude;
pub mod raw;
pub mod uint;

pub(crate) mod internal;
