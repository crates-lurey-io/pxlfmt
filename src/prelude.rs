//! Re-exported types and traits for convenience.
//!
//! ```rust
//! use pxlfmt::prelude::*;
//!
//! let pixel = Pixel::<Rgba8888>::new(0xFF00_00FF);
//! assert_eq!(pixel.red(), 0xFF);
//! assert_eq!(pixel.green(), 0x00);
//! assert_eq!(pixel.blue(), 0x00);
//! assert_eq!(pixel.alpha(), 0xFF);
//! ```

pub use crate::formats::rgba::{Rgba, Rgba8888, RgbaFormat};
pub use crate::pixel::{Format, Pixel};
