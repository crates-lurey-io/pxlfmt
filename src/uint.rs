//! Unsigned integer types.

use crate::internal::Sealed;

/// Generic trait for the built-in Rust unsigned integer types (e.g. `u8`, `u32`, ...).
///
/// Unlike traits provided by crates like `num_traits`, it is _sealed_.
///
/// This trait exists to avoid/allow:
///
/// 1. Having a mandatory dependency on a crate (such as `num_traits`);
/// 1. Worrying about the trait being implemented for types that are not unsigned integers;
/// 1. Adding new methods that might only be useful in the context of integer-based pixel formats.
#[allow(private_bounds)]
pub trait Uint: Sealed + Sized + Copy + PartialEq + Eq + PartialOrd + Ord {
    /// The integer value of `0`.
    const ZERO: Self;

    /// The maximum value of the integer type.
    const MAX: Self;
}

macro_rules! impl_uint {
  ($($t:ty),*) => {
    $(
      impl Sealed for $t {}
      impl Uint for $t {
        const ZERO: Self = 0;
        const MAX: Self = <$t>::MAX;
      }
    )*
  };
}

impl_uint!(u8, u16, u32, u64);
