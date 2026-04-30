# satint

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

assert_eq!((su8(250) + su8(10)).into_inner(), u8::MAX);
assert_eq!((su8(0) - su8(1)).into_inner(), 0);

let mut health = Su8::MAX;
health += su8(1);
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

`+`, `-`, and `*` saturate for same-width values:

```rust
use satint::{Si32, Su8, si32, su8};

assert_eq!((Su8::MAX + su8(1)).into_inner(), u8::MAX);
assert_eq!((su8(0) - su8(1)).into_inner(), 0);

assert_eq!((Si32::MAX + si32(1)).into_inner(), i32::MAX);
assert_eq!((Si32::MIN - si32(1)).into_inner(), i32::MIN);
assert_eq!((si32(6) * si32(-7)).into_inner(), -42);
```

Primitive right-hand sides are also supported:

```rust
use satint::{Su16, su16};

let mut value = su16(10);
value += 5;
value *= 3;

assert_eq!(value.into_inner(), 45);
assert_eq!((Su16::MAX + 1).into_inner(), u16::MAX);
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
use satint::{Si32, Su16, Su32, si8, su8};

let signed: Si32 = si8(-5).into();
let unsigned: Su32 = su8(200).into();
let unsigned_to_signed: Si32 = Su16::new(40_000).into();

assert_eq!(signed.into_inner(), -5);
assert_eq!(unsigned.into_inner(), 200);
assert_eq!(unsigned_to_signed.into_inner(), 40_000);
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
use satint::{SaturatingFrom, SaturatingInto, Si8, Su8, si16, su16};

assert_eq!(Su8::saturating_from(su16(300)).into_inner(), u8::MAX);
assert_eq!(Si8::saturating_from(si16(-300)).into_inner(), i8::MIN);

let value: Su8 = su16(999).saturating_into();
assert_eq!(value.into_inner(), u8::MAX);
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

## Optional `serde` Support

Enable the `serde` feature to serialize and deserialize scalar values as their inner integer representation:

```toml
[dependencies]
satint = { version = "0.1", features = ["serde"] }
```

The feature depends on `serde` without enabling serde's default features, so it remains compatible with `no_std` users.

## `no_std`

`satint` is `#![no_std]` and does not use `alloc`.

## Panics

No operation in this crate panics. Arithmetic operators saturate, division and
remainder are exposed only as `checked_div` / `checked_rem` returning `Option`,
and conversions are either lossless (`From`), fallible (`TryFrom`), or
clamping (`SaturatingFrom` / `SaturatingInto`). The crate is also
`#![forbid(unsafe_code)]`.

## Limitations

- This crate is for integer types only. Floats do not have integer-style
  saturating bounds.
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

Licensed under the MIT or the Apache license.
