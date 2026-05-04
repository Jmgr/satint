#![no_std]
#![forbid(unsafe_code)]
//#![cfg_attr(doctest, doc = include_str!("../README.md"))]
//! Saturating scalar wrappers for signed and unsigned primitive integers.

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
