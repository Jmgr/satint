# Saturating integers

[![CI](https://github.com/Jmgr/satint/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Jmgr/satint/actions/workflows/ci.yml)
[![coverage](https://img.shields.io/badge/coverage-100%25-brightgreen.svg)](https://github.com/Jmgr/satint/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/satint.svg)](https://crates.io/crates/satint)
[![docs.rs](https://docs.rs/satint/badge.svg)](https://docs.rs/satint)
[![MSRV](https://img.shields.io/badge/rustc-1.85.1%2B-blue.svg)](https://github.com/Jmgr/satint/blob/main/Cargo.toml)
[![no_std](https://img.shields.io/badge/no__std-supported-green.svg)](https://docs.rs/satint)
[![license](https://img.shields.io/crates/l/satint.svg)](https://github.com/Jmgr/satint#license)

`satint` provides `no_std`, no-alloc integer wrapper types whose arithmetic
operators saturate at the destination type's numeric bounds.

The signed wrappers are `Si8`, `Si16`, `Si32`, `Si64`, `Si128`, and `Sisize`.
The unsigned wrappers are `Su8`, `Su16`, `Su32`, `Su64`, `Su128`, and `Susize`.
Each type has a matching `const` constructor function: `si8`, `si16`, `sisize`,
`su8`, `su16`, `susize`, and so on.

```rust
use satint::{Su8, su8};

assert_eq!(su8(250) + su8(10), Su8::MAX);
assert_eq!(su8(0) - 1, Su8::ZERO);

let mut health = Su8::MAX;
health += 1;
assert_eq!(health, Su8::MAX);
```

## Types And Constructors

Wrappers are transparent newtypes around `core::num::Saturating<T>`. Use the
associated `new` constructor, the short free constructor, or `From` for the
matching primitive. Use `into_inner` to recover the primitive value.

```rust
use satint::{Si16, Su32, sisize, si16, su32, susize};

let signed = Si16::new(-40);
let also_signed = si16(-40);
let unsigned = Su32::from(40_u32);
let pointer_sized = sisize(-10) + susize(5);

assert_eq!(signed, also_signed);
assert_eq!(unsigned, su32(40));
assert_eq!(pointer_sized, sisize(-5));
assert_eq!(signed.into_inner(), -40);
```

Each wrapper provides `BITS`, `MIN`, `MAX`, `ZERO`, and `ONE`.

```rust
use satint::{Si8, Su8};

assert_eq!(Si8::MIN.into_inner(), i8::MIN);
assert_eq!(Su8::MAX.into_inner(), u8::MAX);
assert_eq!(Su8::ZERO + Su8::ONE, Su8::ONE);
```

## Arithmetic

`+`, `-`, `*`, and signed unary `-` use saturating arithmetic. Assignment forms
such as `+=` and `*=` have the same behavior. The left-hand side determines the
output type, so cross-width arithmetic clamps to the left-hand side's range.

```rust
use satint::{Si8, Si16, Su8, Su16, si8, si16, su8};

assert_eq!(Su8::MAX + 1, Su8::MAX);
assert_eq!(su8(0) - 1, Su8::ZERO);
assert_eq!(si8(100) * 2, Si8::MAX);
assert_eq!(-Si8::MIN, Si8::MAX);

assert_eq!(si16(120) + si8(10), si16(130));
assert_eq!(Si8::MAX + Si16::new(10), Si8::MAX);

let mut value = Su16::new(10);
value += Su8::new(5);
value *= 3;
assert_eq!(value.into_inner(), 45);
```

Mixed signed/unsigned addition and subtraction are supported for wrapper and
primitive right-hand sides. Mixed signed/unsigned multiplication is not.

```rust
use satint::{Si8, Su8, si8, su8};

assert_eq!(si8(10) - su8(30), Si8::new(-20));
assert_eq!(si8(120) + su8(20), Si8::MAX);
assert_eq!(su8(5) + -10, Su8::ZERO);
assert_eq!(su8(5) - -10, Su8::new(15));
```

Left shifts saturate when bits would be shifted out of range. Right shifts
match the primitive integer behavior, except oversized unsigned shifts produce
zero and oversized signed shifts extend the sign.

```rust
use satint::{Si8, Su8, si8, su8};

assert_eq!(su8(0b0100_0000) << 2, Su8::MAX);
assert_eq!(su8(0b1000_0000) >> 8, Su8::ZERO);
assert_eq!(si8(-1) >> 8, si8(-1));
```

Bitwise `&`, `|`, `^`, and `!` are available for another value of the same
wrapper type or the matching primitive type.

```rust
use satint::{Su8, su8};

let mask = su8(0b1100);
assert_eq!(mask & su8(0b1010), su8(0b1000));
assert_eq!(mask | 0b0011_u8, su8(0b1111));
assert_eq!(!Su8::ZERO, Su8::MAX);
```

## Division And Remainder

The inherent `checked_*` methods accept the same wrapper type and return
`Option`.

```rust
use satint::{Si32, si32, su32};

assert_eq!(si32(20).checked_div(si32(3)), Some(si32(6)));
assert_eq!(si32(20).checked_rem(si32(3)), Some(si32(2)));
assert_eq!(su32(20).checked_div(su32(0)), None);
assert_eq!(Si32::MIN.checked_div(si32(-1)), None);
```

The `TryDiv`, `TryRem`, `TryDivAssign`, and `TryRemAssign` traits accept any
same-sign wrapper width or same-sign primitive right-hand side and return
`Result<_, DivError>`.

```rust
use satint::{DivError, Si8, Si16, TryDiv, TryDivAssign, TryRem, si8};

assert_eq!(si8(20).try_div(Si16::new(3)), Ok(si8(6)));
assert_eq!(si8(20).try_rem(3_i8), Ok(si8(2)));
assert_eq!(si8(20).try_div(0_i8), Err(DivError::DivisionByZero));

let mut value = Si8::new(20);
value.try_div_assign(4_i8).unwrap();
assert_eq!(value, si8(5));
```

With the optional `panicking-ops` feature, `/`, `%`, `/=`, and `%=` are also
implemented. Those operators panic on zero divisors just like primitive integer
division.

## Conversions

Use `From` and `Into` for conversions that cannot lose information.

```rust
use satint::{Si32, Su32, si8, su8};

let signed: Si32 = si8(-5).into();
let unsigned: Su32 = su8(200).into();
let primitive: u32 = unsigned.into();

assert_eq!(signed.into_inner(), -5);
assert_eq!(primitive, 200);
```

Use `SaturatingFrom` or `SaturatingInto` when the source may not fit. These
traits clamp to the destination range. Signed-to-unsigned conversions clamp
negative values to zero. The traits are implemented between all primitive
integer types, between wrappers, and between wrappers and primitive integers.

```rust
use satint::{SaturatingFrom, SaturatingInto, Si8, Su8, si16, su16};

let unsigned: Su8 = su16(999).saturating_into();
let signed: Si8 = si16(-300).saturating_into();
let from_primitive = Su8::saturating_from(-12_i32);
let primitive: u8 = u8::saturating_from(si16(300));
let narrowed: i8 = i16::MAX.saturating_into();
let non_negative: usize = isize::MIN.saturating_into();

assert_eq!(unsigned, Su8::MAX);
assert_eq!(signed, Si8::MIN);
assert_eq!(from_primitive, Su8::ZERO);
assert_eq!(primitive, u8::MAX);
assert_eq!(narrowed, i8::MAX);
assert_eq!(non_negative, 0);
```

Same-width signedness conversions also have inherent saturating helpers.

```rust
use satint::{Si32, Su32, si32, su32};

assert_eq!(Su32::MAX.to_signed(), Si32::MAX);
assert_eq!(si32(-1).to_unsigned(), Su32::ZERO);
assert_eq!(su32(42).to_signed(), si32(42));
assert_eq!(si32(42).to_unsigned(), su32(42));
```

Floating-point sources can be converted into primitive integers or wrappers with
`SaturatingFrom` and `SaturatingInto`. These conversions use Rust's `as` cast
behavior: finite values truncate toward zero, out-of-range values clamp, and
`NaN` becomes zero.

```rust
use satint::{SaturatingFrom, SaturatingInto, Si32, Su8};

let primitive: i8 = 200.0_f32.saturating_into();
let unsigned_primitive = u8::saturating_from(-1.0_f64);

assert_eq!(Si32::saturating_from(3.7_f64).into_inner(), 3);
assert_eq!(Si32::saturating_from(-3.7_f64).into_inner(), -3);
assert_eq!(Si32::saturating_from(f64::NAN), Si32::ZERO);
assert_eq!(Si32::saturating_from(f64::INFINITY), Si32::MAX);
assert_eq!(Su8::saturating_from(-1.0_f32), Su8::ZERO);
assert_eq!(Su8::saturating_from(300.0_f32), Su8::MAX);
assert_eq!(primitive, i8::MAX);
assert_eq!(unsigned_primitive, 0);
```

Wrapper-to-float `From` impls are provided only where every source value is
represented exactly: `Si8`, `Si16`, `Su8`, and `Su16` convert to `f32`; those
types plus `Si32` and `Su32` convert to `f64`.

```rust
use satint::{si16, su32};

let as_f32: f32 = si16(-1234).into();
let as_f64: f64 = su32(4_000_000_000).into();

assert_eq!(as_f32, -1234.0);
assert_eq!(as_f64, 4_000_000_000.0);
```

## Numeric Helpers

Wrappers expose many primitive integer helpers, returning wrapper values when
the primitive method would return an integer of the same type.

```rust
use satint::{Si8, Su8, si8, su8};

assert_eq!(si8(-5).abs(), si8(5));
assert_eq!(Si8::MIN.abs(), Si8::MAX);
assert_eq!(si8(-10).abs_diff(si8(5)), su8(15));
assert_eq!(si8(16).checked_isqrt(), Some(si8(4)));

assert_eq!(su8(15).next_power_of_two(), su8(16));
assert_eq!(Su8::MAX.next_power_of_two(), Su8::MAX);
assert_eq!(su8(15).checked_next_power_of_two(), Some(su8(16)));
assert_eq!(su8(16).isqrt(), su8(4));
```

Common bit, endian, checked division/remainder, power, and integer logarithm
helpers mirror primitive integer methods.

```rust
use satint::{Si8, Su16, su16};

let value = su16(0x1234);
assert_eq!(value.count_ones(), 5);
assert_eq!(Su16::from_be_bytes(value.to_be_bytes()), value);
assert_eq!(su16(2).pow(20), Su16::MAX);
assert_eq!(su16(100).checked_ilog10(), Some(2));

assert!(Si8::MIN.is_min());
assert!(Su16::ZERO.is_zero());
```

## Iterators

`Sum` and `Product` are implemented for scalar values and references.

```rust
use satint::{Su32, su32};

let values = [su32(1), su32(2), su32(3), su32(4)];

assert_eq!(values.iter().copied().sum::<Su32>(), su32(10));
assert_eq!(values.iter().product::<Su32>(), su32(24));
```

## Optional Features

The `serde` feature serializes and deserializes wrappers as their inner
primitive integer values.

The `rand` feature implements `rand` 0.10 uniform sampling for every wrapper.

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

`satint` is `#![no_std]`, does not use `alloc`, and forbids unsafe code.

## License

Licensed under either of MIT or Apache-2.0, at your option.
