#![no_std]
#![forbid(unsafe_code)]
//! Saturating scalar wrappers for signed and unsigned primitive integers.
//!
//! The crate exposes signed wrappers ([`Si8`], [`Si16`], [`Si32`], [`Si64`],
//! [`Si128`]) and unsigned wrappers ([`Su8`], [`Su16`], [`Su32`], [`Su64`],
//! [`Su128`]). Arithmetic operators saturate at the left-hand side type's
//! numeric bounds.
//!
//! ```
//! use satint::{Si8, Su8, si8, su8};
//!
//! assert_eq!(su8(250) + su8(10), Su8::MAX);
//! assert_eq!(su8(0) - 1, Su8::ZERO);
//! assert_eq!(si8(100) * 2, Si8::MAX);
//! assert_eq!(-Si8::MIN, Si8::MAX);
//! ```
//!
//! Checked division and remainder are available as inherent `checked_*`
//! methods for same-type operands, and as [`TryDiv`] / [`TryRem`] traits when
//! the right-hand side is another same-sign wrapper width or a primitive.
//!
//! ```
//! use satint::{DivError, TryDiv, TryRem, si8, si16};
//!
//! assert_eq!(si8(20).checked_div(si8(3)), Some(si8(6)));
//! assert_eq!(si8(20).try_div(si16(3)), Ok(si8(6)));
//! assert_eq!(si8(20).try_rem(3_i8), Ok(si8(2)));
//! assert_eq!(si8(20).try_div(0_i8), Err(DivError::DivisionByZero));
//! ```
//!
//! Use [`From`] / [`Into`] for lossless conversions and [`SaturatingFrom`] /
//! [`SaturatingInto`] when the source may be out of range.
//!
//! ```
//! use satint::{SaturatingFrom, SaturatingInto, Si8, Si32, Su8, si16, su8};
//!
//! let widened: Si32 = Si8::new(-5).into();
//! let clamped: Su8 = si16(-1).saturating_into();
//! let from_float = Si8::saturating_from(200.0_f32);
//!
//! assert_eq!(widened, Si32::new(-5));
//! assert_eq!(clamped, Su8::ZERO);
//! assert_eq!(from_float, Si8::MAX);
//! assert_eq!(su8(42).to_signed(), Si8::new(42));
//! ```

mod convert;
mod si;
mod su;

#[cfg(feature = "rand")]
mod rand;
#[cfg(feature = "serde")]
mod serde;

mod common;
mod ops;

pub use common::{SaturatingFrom, SaturatingInto};
pub use ops::{DivError, TryDiv, TryDivAssign, TryRem, TryRemAssign};
pub use si::{Si8, Si16, Si32, Si64, Si128, si8, si16, si32, si64, si128};
pub use su::{Su8, Su16, Su32, Su64, Su128, su8, su16, su32, su64, su128};
