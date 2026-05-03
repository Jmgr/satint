# Saturating integers

[![CI](https://github.com/Jmgr/satint/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Jmgr/satint/actions/workflows/ci.yml)
[![coverage](https://img.shields.io/badge/coverage-100%25-brightgreen.svg)](https://github.com/Jmgr/satint/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/satint.svg)](https://crates.io/crates/satint)
[![docs.rs](https://docs.rs/satint/badge.svg)](https://docs.rs/satint)
[![MSRV](https://img.shields.io/badge/rustc-1.85.1%2B-blue.svg)](https://github.com/Jmgr/satint/blob/main/Cargo.toml)
[![no_std](https://img.shields.io/badge/no__std-supported-green.svg)](https://docs.rs/satint)
[![license](https://img.shields.io/crates/l/satint.svg)](https://github.com/Jmgr/satint#license)

`satint` provides small `no_std`, no-alloc wrappers around Rust primitive
integers that use saturating arithmetic for ordinary integer operations.

The crate exposes signed aliases (`Si8`, `Si16`, `Si32`, `Si64`, `Si128`) and
unsigned aliases (`Su8`, `Su16`, `Su32`, `Su64`, `Su128`), plus matching
constructor functions (`si8`, `su32`, and so on).

## Why

Rust already has primitive methods like `u8::saturating_add`, but using them
consistently can be noisy when a value should saturate by default. `satint`
makes that behavior part of the type:

```rust
use satint::{Su8, su8};

assert_eq!((su8(250) + 10).into_inner(), u8::MAX);
assert_eq!((su8(0) - 1).into_inner(), 0);

let mut health = Su8::MAX;
health += 1;
assert_eq!(health, Su8::MAX);
```

## Supported Types

Signed:

```rust
use satint::{Si8, Si16, Si32, Si64, Si128};
use satint::{si8, si16, si32, si64, si128};
```

Unsigned:

```rust
use satint::{Su8, Su16, Su32, Su64, Su128};
use satint::{su8, su16, su32, su64, su128};
```

Each alias is a transparent wrapper over `core::num::Saturating<T>`.

## Arithmetic

`+`, `-`, and `*` saturate for same-width values. The right-hand side can be
either another wrapper or a matching primitive:

```rust
use satint::{Si32, Su8, Su16, si32, su8, su16};

assert_eq!((Su8::MAX + 1).into_inner(), u8::MAX);
assert_eq!((su8(0) - 1).into_inner(), 0);

assert_eq!((Si32::MAX + 1).into_inner(), i32::MAX);
assert_eq!((Si32::MIN - 1).into_inner(), i32::MIN);
assert_eq!((si32(6) * -7).into_inner(), -42);

let mut value = su16(10);
value += 5;
value *= 3;
assert_eq!(value.into_inner(), 45);
```

## Division And Remainder

Division and remainder are intentionally checked methods rather than `/` and
`%` operator impls. They return `None` for division by zero, and for signed
overflow such as `MIN / -1`.

```rust
use satint::{Si32, si32, su32};

assert_eq!(si32(20).checked_div(si32(3)), Some(si32(6)));
assert_eq!(si32(20).checked_rem(si32(3)), Some(si32(2)));

assert_eq!(su32(20).checked_div(su32(0)), None);
assert_eq!(Si32::MIN.checked_div(si32(-1)), None);
```

## Conversions

Lossless widening conversions use `From` / `Into`:

```rust
use satint::{Si32, Su32, si8, su8};

let signed: Si32 = si8(-5).into();
let unsigned: Su32 = su8(200).into();

assert_eq!(signed.into_inner(), -5);
assert_eq!(unsigned.into_inner(), 200);
```

Fallible narrowing and cross-sign conversions use `TryFrom`:

```rust
use satint::{Si8, Su8, si16, su16};

assert_eq!(Su8::try_from(su16(40)).map(Su8::into_inner), Ok(40));
assert!(Su8::try_from(su16(300)).is_err());

assert_eq!(Si8::try_from(si16(-50)).map(Si8::into_inner), Ok(-50));
assert!(Si8::try_from(si16(300)).is_err());
```

Clamping conversions use `SaturatingFrom` or `SaturatingInto`:

```rust
use satint::{SaturatingInto, Si8, Su8, si16, su16};

let unsigned: Su8 = su16(999).saturating_into();
let signed: Si8 = si16(-300).saturating_into();
let pointer_sized: usize = Si8::MIN.saturating_into();

assert_eq!(unsigned, Su8::MAX);
assert_eq!(signed, Si8::MIN);
assert_eq!(pointer_sized, 0);
```

Same-width signedness flips have shorthand inherent methods:

```rust
use satint::{Si32, Su32, si32, su32};

assert_eq!(Su32::MAX.to_signed(), Si32::MAX);
assert_eq!(si32(-1).to_unsigned(), Su32::ZERO);

assert_eq!(su32(42).to_signed(), si32(42));
assert_eq!(si32(42).to_unsigned(), su32(42));
```

Primitive integers can also be used as the source:

```rust
use satint::{SaturatingInto, Su8};

let low: Su8 = (-1_i32).saturating_into();
let high: Su8 = 300_i32.saturating_into();

assert_eq!(low, Su8::ZERO);
assert_eq!(high, Su8::MAX);
```

## Float Conversions

`f32` and `f64` can be converted into any signed or unsigned wrapper, in both
saturating and fallible forms. Both truncate toward zero on the way to an
integer.

`SaturatingFrom` mirrors Rust's `as` cast: `NaN` becomes zero, infinities
saturate to `MIN` / `MAX` (or `0` / `MAX` for unsigned), and finite
out-of-range values clamp to the closest endpoint.

```rust
use satint::{SaturatingFrom, Si32, Su8};

assert_eq!(Si32::saturating_from(3.7_f64).into_inner(), 3);
assert_eq!(Si32::saturating_from(-3.7_f64).into_inner(), -3);
assert_eq!(Si32::saturating_from(f64::NAN).into_inner(), 0);
assert_eq!(Si32::saturating_from(f64::INFINITY), Si32::MAX);
assert_eq!(Su8::saturating_from(-1.0_f32), Su8::ZERO);
assert_eq!(Su8::saturating_from(300.0_f32), Su8::MAX);
```

`TryFrom` rejects `NaN`, `±Inf`, and any finite value whose truncated form
falls outside the destination's range, returning `TryFromFloatError`.

```rust
use satint::{Si16, Su8, TryFromFloatError};

assert_eq!(Si16::try_from(1234.7_f64).map(Si16::into_inner), Ok(1234));
assert!(Si16::try_from(40_000.0_f64).is_err());
assert!(Su8::try_from(-1.0_f32).is_err());
assert!(Si16::try_from(f64::NAN).is_err());

let _: TryFromFloatError = Si16::try_from(f64::NAN).unwrap_err();
```

The reverse direction — wrapper to float — is provided as `From` only for
widths that round-trip exactly: `Si8`, `Si16`, `Su8`, `Su16` for `f32`, and
those plus `Si32`, `Su32` for `f64`. Wider integers are not supported as
sources because not every value would survive the cast losslessly.

```rust
use satint::{si16, su32};

let as_f32: f32 = si16(-1234).into();
let as_f64: f64 = su32(4_000_000_000).into();

assert_eq!(as_f32, -1234.0);
assert_eq!(as_f64, 4_000_000_000.0);
```

## Widening Arithmetic

Same-sign wider-left-hand-side `+` and `-` are supported when the right-hand
side always fits in the left-hand side:

```rust
use satint::{si8, si16, su16, su32};

assert_eq!((su32(40) + su16(2)).into_inner(), 42);
assert_eq!((si16(40) - si8(2)).into_inner(), 38);
```

This is intentionally one-directional: the wider type must be on the left.

## Constants And Iterators

Concrete aliases provide `MIN`, `MAX`, `ZERO`, and `ONE`.

`Sum` and `Product` are implemented for scalar values and references:

```rust
use satint::{Su32, su32};

let values = [su32(1), su32(2), su32(3), su32(4)];

assert_eq!(values.iter().copied().sum::<Su32>().into_inner(), 10);
assert_eq!(values.iter().product::<Su32>().into_inner(), 24);
```

## Optional `serde` and `rand` Support

```rust,ignore
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use satint::{si16, su8};

let mut rng = SmallRng::seed_from_u64(1);

let signed = rng.random_range(si16(-10)..si16(10));
assert!(signed >= si16(-10));
assert!(signed < si16(10));

let unsigned = rng.random_range(su8(1)..=su8(6));
assert!(unsigned >= su8(1));
assert!(unsigned <= su8(6));
```

## `no_std`

`satint` is `#![no_std]` and does not use `alloc`.

## Panics

No operation in this crate panics. Arithmetic operators saturate, division and
remainder are exposed only as `checked_div` / `checked_rem` returning `Option`,
and conversions are either lossless (`From`), fallible (`TryFrom`), or
clamping (`SaturatingFrom` / `SaturatingInto`). The crate is also
`#![forbid(unsafe_code)]`.

## Limitations

- Wrapped values are integers only — there are no floating-point wrappers.
  Float ↔ integer conversions are provided through `SaturatingFrom`,
  `TryFrom`, and `From` (lossless cases only).
- Only `+`, `-`, `*`, their assignment forms, and signed unary `-` are operator
  overloads.
- Division and remainder are available only through `checked_div` and
  `checked_rem`.
- Cross-width arithmetic is limited to same-sign wider-left-hand-side `+` and
  `-`.
- Mixed signed/unsigned arithmetic is not implemented directly. Convert first
  with `From`, `TryFrom`, or the saturating conversion traits.
- Saturation is not error reporting. If you need to detect overflow, use
  primitive checked arithmetic or fallible conversions where appropriate.

## License

Licensed under either of MIT or Apache-2.0, at your option.
