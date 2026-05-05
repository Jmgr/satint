//! Cross-product integration tests for the operator impls in `src/ops.rs`.
//!
//! Unit-level line coverage hits each branch of the operator macros once, but
//! every macro invocation produces a fresh monomorphization that line coverage
//! cannot distinguish. These tests exercise the full lhs × rhs matrix at
//! runtime so width-specific bugs (e.g. a widening cast that is lossless at
//! one size but lossy at another) surface as concrete test failures.
//!
//! Each test defines a tiny inner `check!` macro encoding the contract for a
//! single pair, then invokes a top-level fan-out macro to apply it across the
//! relevant type list. Asserts carry `concat!(stringify!(...), ...)` labels so
//! a failure identifies the failing pair without manual bookkeeping.

#![allow(
    clippy::cognitive_complexity,
    clippy::unwrap_used,
    reason = "Macro fan-out tests intentionally expand into broad operator matrices"
)]

use satint::{
    DivError, SaturatingInto, Si8, Si16, Si32, Si64, Si128, Sisize, Su8, Su16, Su32, Su64, Su128,
    Susize, TryDiv, TryDivAssign, TryRem, TryRemAssign,
};

// ============================================================================
// Fan-out macros: enumerate the type matrix once, reuse from every test.
// ============================================================================

macro_rules! for_each_signed_wrapper_pair {
    ($body:ident) => {
        $body!(Si8, Si8);
        $body!(Si8, Si16);
        $body!(Si8, Si32);
        $body!(Si8, Si64);
        $body!(Si8, Si128);
        $body!(Si8, Sisize);
        $body!(Si16, Si8);
        $body!(Si16, Si16);
        $body!(Si16, Si32);
        $body!(Si16, Si64);
        $body!(Si16, Si128);
        $body!(Si16, Sisize);
        $body!(Si32, Si8);
        $body!(Si32, Si16);
        $body!(Si32, Si32);
        $body!(Si32, Si64);
        $body!(Si32, Si128);
        $body!(Si32, Sisize);
        $body!(Si64, Si8);
        $body!(Si64, Si16);
        $body!(Si64, Si32);
        $body!(Si64, Si64);
        $body!(Si64, Si128);
        $body!(Si64, Sisize);
        $body!(Si128, Si8);
        $body!(Si128, Si16);
        $body!(Si128, Si32);
        $body!(Si128, Si64);
        $body!(Si128, Si128);
        $body!(Si128, Sisize);
        $body!(Sisize, Si8);
        $body!(Sisize, Si16);
        $body!(Sisize, Si32);
        $body!(Sisize, Si64);
        $body!(Sisize, Si128);
        $body!(Sisize, Sisize);
    };
}

macro_rules! for_each_unsigned_wrapper_pair {
    ($body:ident) => {
        $body!(Su8, Su8);
        $body!(Su8, Su16);
        $body!(Su8, Su32);
        $body!(Su8, Su64);
        $body!(Su8, Su128);
        $body!(Su8, Susize);
        $body!(Su16, Su8);
        $body!(Su16, Su16);
        $body!(Su16, Su32);
        $body!(Su16, Su64);
        $body!(Su16, Su128);
        $body!(Su16, Susize);
        $body!(Su32, Su8);
        $body!(Su32, Su16);
        $body!(Su32, Su32);
        $body!(Su32, Su64);
        $body!(Su32, Su128);
        $body!(Su32, Susize);
        $body!(Su64, Su8);
        $body!(Su64, Su16);
        $body!(Su64, Su32);
        $body!(Su64, Su64);
        $body!(Su64, Su128);
        $body!(Su64, Susize);
        $body!(Su128, Su8);
        $body!(Su128, Su16);
        $body!(Su128, Su32);
        $body!(Su128, Su64);
        $body!(Su128, Su128);
        $body!(Su128, Susize);
        $body!(Susize, Su8);
        $body!(Susize, Su16);
        $body!(Susize, Su32);
        $body!(Susize, Su64);
        $body!(Susize, Su128);
        $body!(Susize, Susize);
    };
}

macro_rules! for_each_signed_lhs_unsigned_rhs_wrapper {
    ($body:ident) => {
        $body!(Si8, Su8);
        $body!(Si8, Su16);
        $body!(Si8, Su32);
        $body!(Si8, Su64);
        $body!(Si8, Su128);
        $body!(Si8, Susize);
        $body!(Si16, Su8);
        $body!(Si16, Su16);
        $body!(Si16, Su32);
        $body!(Si16, Su64);
        $body!(Si16, Su128);
        $body!(Si16, Susize);
        $body!(Si32, Su8);
        $body!(Si32, Su16);
        $body!(Si32, Su32);
        $body!(Si32, Su64);
        $body!(Si32, Su128);
        $body!(Si32, Susize);
        $body!(Si64, Su8);
        $body!(Si64, Su16);
        $body!(Si64, Su32);
        $body!(Si64, Su64);
        $body!(Si64, Su128);
        $body!(Si64, Susize);
        $body!(Si128, Su8);
        $body!(Si128, Su16);
        $body!(Si128, Su32);
        $body!(Si128, Su64);
        $body!(Si128, Su128);
        $body!(Si128, Susize);
        $body!(Sisize, Su8);
        $body!(Sisize, Su16);
        $body!(Sisize, Su32);
        $body!(Sisize, Su64);
        $body!(Sisize, Su128);
        $body!(Sisize, Susize);
    };
}

macro_rules! for_each_unsigned_lhs_signed_rhs_wrapper {
    ($body:ident) => {
        $body!(Su8, Si8);
        $body!(Su8, Si16);
        $body!(Su8, Si32);
        $body!(Su8, Si64);
        $body!(Su8, Si128);
        $body!(Su8, Sisize);
        $body!(Su16, Si8);
        $body!(Su16, Si16);
        $body!(Su16, Si32);
        $body!(Su16, Si64);
        $body!(Su16, Si128);
        $body!(Su16, Sisize);
        $body!(Su32, Si8);
        $body!(Su32, Si16);
        $body!(Su32, Si32);
        $body!(Su32, Si64);
        $body!(Su32, Si128);
        $body!(Su32, Sisize);
        $body!(Su64, Si8);
        $body!(Su64, Si16);
        $body!(Su64, Si32);
        $body!(Su64, Si64);
        $body!(Su64, Si128);
        $body!(Su64, Sisize);
        $body!(Su128, Si8);
        $body!(Su128, Si16);
        $body!(Su128, Si32);
        $body!(Su128, Si64);
        $body!(Su128, Si128);
        $body!(Su128, Sisize);
        $body!(Susize, Si8);
        $body!(Susize, Si16);
        $body!(Susize, Si32);
        $body!(Susize, Si64);
        $body!(Susize, Si128);
        $body!(Susize, Sisize);
    };
}

macro_rules! for_each_signed_wrapper_x_signed_primitive {
    ($body:ident) => {
        $body!(Si8, i8);
        $body!(Si8, i16);
        $body!(Si8, i32);
        $body!(Si8, i64);
        $body!(Si8, i128);
        $body!(Si8, isize);
        $body!(Si16, i8);
        $body!(Si16, i16);
        $body!(Si16, i32);
        $body!(Si16, i64);
        $body!(Si16, i128);
        $body!(Si16, isize);
        $body!(Si32, i8);
        $body!(Si32, i16);
        $body!(Si32, i32);
        $body!(Si32, i64);
        $body!(Si32, i128);
        $body!(Si32, isize);
        $body!(Si64, i8);
        $body!(Si64, i16);
        $body!(Si64, i32);
        $body!(Si64, i64);
        $body!(Si64, i128);
        $body!(Si64, isize);
        $body!(Si128, i8);
        $body!(Si128, i16);
        $body!(Si128, i32);
        $body!(Si128, i64);
        $body!(Si128, i128);
        $body!(Si128, isize);
        $body!(Sisize, i8);
        $body!(Sisize, i16);
        $body!(Sisize, i32);
        $body!(Sisize, i64);
        $body!(Sisize, i128);
        $body!(Sisize, isize);
    };
}

macro_rules! for_each_unsigned_wrapper_x_unsigned_primitive {
    ($body:ident) => {
        $body!(Su8, u8);
        $body!(Su8, u16);
        $body!(Su8, u32);
        $body!(Su8, u64);
        $body!(Su8, u128);
        $body!(Su8, usize);
        $body!(Su16, u8);
        $body!(Su16, u16);
        $body!(Su16, u32);
        $body!(Su16, u64);
        $body!(Su16, u128);
        $body!(Su16, usize);
        $body!(Su32, u8);
        $body!(Su32, u16);
        $body!(Su32, u32);
        $body!(Su32, u64);
        $body!(Su32, u128);
        $body!(Su32, usize);
        $body!(Su64, u8);
        $body!(Su64, u16);
        $body!(Su64, u32);
        $body!(Su64, u64);
        $body!(Su64, u128);
        $body!(Su64, usize);
        $body!(Su128, u8);
        $body!(Su128, u16);
        $body!(Su128, u32);
        $body!(Su128, u64);
        $body!(Su128, u128);
        $body!(Su128, usize);
        $body!(Susize, u8);
        $body!(Susize, u16);
        $body!(Susize, u32);
        $body!(Susize, u64);
        $body!(Susize, u128);
        $body!(Susize, usize);
    };
}

macro_rules! for_each_signed_wrapper_x_unsigned_primitive {
    ($body:ident) => {
        $body!(Si8, u8);
        $body!(Si8, u16);
        $body!(Si8, u32);
        $body!(Si8, u64);
        $body!(Si8, u128);
        $body!(Si8, usize);
        $body!(Si16, u8);
        $body!(Si16, u16);
        $body!(Si16, u32);
        $body!(Si16, u64);
        $body!(Si16, u128);
        $body!(Si16, usize);
        $body!(Si32, u8);
        $body!(Si32, u16);
        $body!(Si32, u32);
        $body!(Si32, u64);
        $body!(Si32, u128);
        $body!(Si32, usize);
        $body!(Si64, u8);
        $body!(Si64, u16);
        $body!(Si64, u32);
        $body!(Si64, u64);
        $body!(Si64, u128);
        $body!(Si64, usize);
        $body!(Si128, u8);
        $body!(Si128, u16);
        $body!(Si128, u32);
        $body!(Si128, u64);
        $body!(Si128, u128);
        $body!(Si128, usize);
        $body!(Sisize, u8);
        $body!(Sisize, u16);
        $body!(Sisize, u32);
        $body!(Sisize, u64);
        $body!(Sisize, u128);
        $body!(Sisize, usize);
    };
}

macro_rules! for_each_unsigned_wrapper_x_signed_primitive {
    ($body:ident) => {
        $body!(Su8, i8);
        $body!(Su8, i16);
        $body!(Su8, i32);
        $body!(Su8, i64);
        $body!(Su8, i128);
        $body!(Su8, isize);
        $body!(Su16, i8);
        $body!(Su16, i16);
        $body!(Su16, i32);
        $body!(Su16, i64);
        $body!(Su16, i128);
        $body!(Su16, isize);
        $body!(Su32, i8);
        $body!(Su32, i16);
        $body!(Su32, i32);
        $body!(Su32, i64);
        $body!(Su32, i128);
        $body!(Su32, isize);
        $body!(Su64, i8);
        $body!(Su64, i16);
        $body!(Su64, i32);
        $body!(Su64, i64);
        $body!(Su64, i128);
        $body!(Su64, isize);
        $body!(Su128, i8);
        $body!(Su128, i16);
        $body!(Su128, i32);
        $body!(Su128, i64);
        $body!(Su128, i128);
        $body!(Su128, isize);
        $body!(Susize, i8);
        $body!(Susize, i16);
        $body!(Susize, i32);
        $body!(Susize, i64);
        $body!(Susize, i128);
        $body!(Susize, isize);
    };
}

macro_rules! for_each_signed_wrapper {
    ($body:ident) => {
        $body!(Si8);
        $body!(Si16);
        $body!(Si32);
        $body!(Si64);
        $body!(Si128);
        $body!(Sisize);
    };
}

macro_rules! for_each_unsigned_wrapper {
    ($body:ident) => {
        $body!(Su8);
        $body!(Su16);
        $body!(Su32);
        $body!(Su64);
        $body!(Su128);
        $body!(Susize);
    };
}

macro_rules! for_each_signed_primitive_source_for_destination {
    ($body:ident, $destination:ty) => {
        $body!($destination, i8);
        $body!($destination, i16);
        $body!($destination, i32);
        $body!($destination, i64);
        $body!($destination, i128);
        $body!($destination, isize);
    };
}

macro_rules! for_each_unsigned_primitive_source_for_destination {
    ($body:ident, $destination:ty) => {
        $body!($destination, u8);
        $body!($destination, u16);
        $body!($destination, u32);
        $body!($destination, u64);
        $body!($destination, u128);
        $body!($destination, usize);
    };
}

macro_rules! for_each_signed_primitive_pair {
    ($body:ident) => {
        for_each_signed_primitive_source_for_destination!($body, i8);
        for_each_signed_primitive_source_for_destination!($body, i16);
        for_each_signed_primitive_source_for_destination!($body, i32);
        for_each_signed_primitive_source_for_destination!($body, i64);
        for_each_signed_primitive_source_for_destination!($body, i128);
        for_each_signed_primitive_source_for_destination!($body, isize);
    };
}

macro_rules! for_each_unsigned_primitive_pair {
    ($body:ident) => {
        for_each_unsigned_primitive_source_for_destination!($body, u8);
        for_each_unsigned_primitive_source_for_destination!($body, u16);
        for_each_unsigned_primitive_source_for_destination!($body, u32);
        for_each_unsigned_primitive_source_for_destination!($body, u64);
        for_each_unsigned_primitive_source_for_destination!($body, u128);
        for_each_unsigned_primitive_source_for_destination!($body, usize);
    };
}

macro_rules! for_each_signed_destination_unsigned_source_primitive_pair {
    ($body:ident) => {
        for_each_unsigned_primitive_source_for_destination!($body, i8);
        for_each_unsigned_primitive_source_for_destination!($body, i16);
        for_each_unsigned_primitive_source_for_destination!($body, i32);
        for_each_unsigned_primitive_source_for_destination!($body, i64);
        for_each_unsigned_primitive_source_for_destination!($body, i128);
        for_each_unsigned_primitive_source_for_destination!($body, isize);
    };
}

macro_rules! for_each_unsigned_destination_signed_source_primitive_pair {
    ($body:ident) => {
        for_each_signed_primitive_source_for_destination!($body, u8);
        for_each_signed_primitive_source_for_destination!($body, u16);
        for_each_signed_primitive_source_for_destination!($body, u32);
        for_each_signed_primitive_source_for_destination!($body, u64);
        for_each_signed_primitive_source_for_destination!($body, u128);
        for_each_signed_primitive_source_for_destination!($body, usize);
    };
}

// ============================================================================
// Primitive conversions.
// ============================================================================

#[test]
fn saturating_signed_primitive_to_signed_primitive() {
    macro_rules! check {
        ($Destination:ty, $Source:ty) => {{
            let label = concat!(stringify!($Source), " -> ", stringify!($Destination));

            let min_actual: $Destination = <$Source>::MIN.saturating_into();
            let min_expected = ((<$Source>::MIN as i128)
                .clamp(<$Destination>::MIN as i128, <$Destination>::MAX as i128))
                as $Destination;
            assert_eq!(min_actual, min_expected, "{label} source MIN");

            let max_actual: $Destination = <$Source>::MAX.saturating_into();
            let max_expected = ((<$Source>::MAX as i128)
                .clamp(<$Destination>::MIN as i128, <$Destination>::MAX as i128))
                as $Destination;
            assert_eq!(max_actual, max_expected, "{label} source MAX");
        }};
    }

    for_each_signed_primitive_pair!(check);
}

#[test]
fn saturating_unsigned_primitive_to_unsigned_primitive() {
    macro_rules! check {
        ($Destination:ty, $Source:ty) => {{
            let label = concat!(stringify!($Source), " -> ", stringify!($Destination));

            let zero_actual: $Destination = <$Source>::MIN.saturating_into();
            assert_eq!(zero_actual, 0, "{label} source MIN");

            let max_actual: $Destination = <$Source>::MAX.saturating_into();
            let max_expected =
                (<$Source>::MAX as u128).min(<$Destination>::MAX as u128) as $Destination;
            assert_eq!(max_actual, max_expected, "{label} source MAX");
        }};
    }

    for_each_unsigned_primitive_pair!(check);
}

#[test]
fn saturating_signed_primitive_to_unsigned_primitive() {
    macro_rules! check {
        ($Destination:ty, $Source:ty) => {{
            let label = concat!(stringify!($Source), " -> ", stringify!($Destination));

            let min_actual: $Destination = <$Source>::MIN.saturating_into();
            assert_eq!(min_actual, 0, "{label} source MIN");

            let max_actual: $Destination = <$Source>::MAX.saturating_into();
            let max_expected =
                (<$Source>::MAX as u128).min(<$Destination>::MAX as u128) as $Destination;
            assert_eq!(max_actual, max_expected, "{label} source MAX");
        }};
    }

    for_each_unsigned_destination_signed_source_primitive_pair!(check);
}

#[test]
fn saturating_unsigned_primitive_to_signed_primitive() {
    macro_rules! check {
        ($Destination:ty, $Source:ty) => {{
            let label = concat!(stringify!($Source), " -> ", stringify!($Destination));

            let zero_actual: $Destination = <$Source>::MIN.saturating_into();
            assert_eq!(zero_actual, 0, "{label} source MIN");

            let max_actual: $Destination = <$Source>::MAX.saturating_into();
            let max_expected =
                (<$Source>::MAX as u128).min(<$Destination>::MAX as u128) as $Destination;
            assert_eq!(max_actual, max_expected, "{label} source MAX");
        }};
    }

    for_each_signed_destination_unsigned_source_primitive_pair!(check);
}

// ============================================================================
// Same-sign wrapper × wrapper: Add, Sub, Mul (and assign variants).
// ============================================================================

#[test]
fn add_signed_wrapper_to_signed_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " + ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) + <$Rhs>::new(7),
                <$Lhs>::new(17),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX + <$Rhs>::new(1),
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );
            assert_eq!(
                <$Lhs>::MIN + <$Rhs>::new(-1),
                <$Lhs>::MIN,
                "{label} saturate at MIN"
            );

            let mut acc = <$Lhs>::new(10);
            acc += <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(17), "{label} +=");
        }};
    }
    for_each_signed_wrapper_pair!(check);
}

#[test]
fn sub_signed_wrapper_to_signed_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " - ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) - <$Rhs>::new(7),
                <$Lhs>::new(3),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MIN - <$Rhs>::new(1),
                <$Lhs>::MIN,
                "{label} saturate at MIN"
            );
            assert_eq!(
                <$Lhs>::MAX - <$Rhs>::new(-1),
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(10);
            acc -= <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(3), "{label} -=");
        }};
    }
    for_each_signed_wrapper_pair!(check);
}

#[test]
fn mul_signed_wrapper_to_signed_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " * ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(6) * <$Rhs>::new(7),
                <$Lhs>::new(42),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX * <$Rhs>::new(2),
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );
            assert_eq!(
                <$Lhs>::MIN * <$Rhs>::new(2),
                <$Lhs>::MIN,
                "{label} saturate at MIN"
            );

            let mut acc = <$Lhs>::new(6);
            acc *= <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(42), "{label} *=");
        }};
    }
    for_each_signed_wrapper_pair!(check);
}

#[test]
fn add_unsigned_wrapper_to_unsigned_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " + ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) + <$Rhs>::new(7),
                <$Lhs>::new(17),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX + <$Rhs>::new(1),
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(10);
            acc += <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(17), "{label} +=");
        }};
    }
    for_each_unsigned_wrapper_pair!(check);
}

#[test]
fn sub_unsigned_wrapper_to_unsigned_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " - ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) - <$Rhs>::new(7),
                <$Lhs>::new(3),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::ZERO - <$Rhs>::new(1),
                <$Lhs>::ZERO,
                "{label} saturate at ZERO"
            );

            let mut acc = <$Lhs>::new(10);
            acc -= <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(3), "{label} -=");
        }};
    }
    for_each_unsigned_wrapper_pair!(check);
}

#[test]
fn mul_unsigned_wrapper_to_unsigned_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " * ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(6) * <$Rhs>::new(7),
                <$Lhs>::new(42),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX * <$Rhs>::new(2),
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(6);
            acc *= <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(42), "{label} *=");
        }};
    }
    for_each_unsigned_wrapper_pair!(check);
}

// ============================================================================
// Same-sign wrapper × primitive: Add, Sub, Mul (and assign variants).
// ============================================================================

#[test]
fn add_signed_wrapper_to_signed_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " + ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10) + 7 as $rhs,
                <$Lhs>::new(17),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX + 1 as $rhs,
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );
            assert_eq!(
                <$Lhs>::MIN + -1 as $rhs,
                <$Lhs>::MIN,
                "{label} saturate at MIN"
            );

            let mut acc = <$Lhs>::new(10);
            acc += 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(17), "{label} +=");
        }};
    }
    for_each_signed_wrapper_x_signed_primitive!(check);
}

#[test]
fn sub_signed_wrapper_to_signed_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " - ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10) - 7 as $rhs,
                <$Lhs>::new(3),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MIN - 1 as $rhs,
                <$Lhs>::MIN,
                "{label} saturate at MIN"
            );
            assert_eq!(
                <$Lhs>::MAX - -1 as $rhs,
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(10);
            acc -= 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(3), "{label} -=");
        }};
    }
    for_each_signed_wrapper_x_signed_primitive!(check);
}

#[test]
fn mul_signed_wrapper_to_signed_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " * ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(6) * 7 as $rhs,
                <$Lhs>::new(42),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX * 2 as $rhs,
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );
            assert_eq!(
                <$Lhs>::MIN * 2 as $rhs,
                <$Lhs>::MIN,
                "{label} saturate at MIN"
            );

            let mut acc = <$Lhs>::new(6);
            acc *= 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(42), "{label} *=");
        }};
    }
    for_each_signed_wrapper_x_signed_primitive!(check);
}

#[test]
fn add_unsigned_wrapper_to_unsigned_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " + ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10) + 7 as $rhs,
                <$Lhs>::new(17),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX + 1 as $rhs,
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(10);
            acc += 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(17), "{label} +=");
        }};
    }
    for_each_unsigned_wrapper_x_unsigned_primitive!(check);
}

#[test]
fn sub_unsigned_wrapper_to_unsigned_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " - ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10) - 7 as $rhs,
                <$Lhs>::new(3),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::ZERO - 1 as $rhs,
                <$Lhs>::ZERO,
                "{label} saturate at ZERO"
            );

            let mut acc = <$Lhs>::new(10);
            acc -= 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(3), "{label} -=");
        }};
    }
    for_each_unsigned_wrapper_x_unsigned_primitive!(check);
}

#[test]
fn mul_unsigned_wrapper_to_unsigned_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " * ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(6) * 7 as $rhs,
                <$Lhs>::new(42),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX * 2 as $rhs,
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(6);
            acc *= 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(42), "{label} *=");
        }};
    }
    for_each_unsigned_wrapper_x_unsigned_primitive!(check);
}

// ============================================================================
// Cross-sign Add/Sub: signed wrapper × unsigned, and unsigned wrapper × signed.
// ============================================================================

#[test]
fn add_signed_wrapper_to_unsigned_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " + ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) + <$Rhs>::new(7),
                <$Lhs>::new(17),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX + <$Rhs>::new(1),
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(10);
            acc += <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(17), "{label} +=");
        }};
    }
    for_each_signed_lhs_unsigned_rhs_wrapper!(check);
}

#[test]
fn sub_signed_wrapper_to_unsigned_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " - ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) - <$Rhs>::new(7),
                <$Lhs>::new(3),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MIN - <$Rhs>::new(1),
                <$Lhs>::MIN,
                "{label} saturate at MIN"
            );

            let mut acc = <$Lhs>::new(10);
            acc -= <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(3), "{label} -=");
        }};
    }
    for_each_signed_lhs_unsigned_rhs_wrapper!(check);
}

#[test]
fn add_unsigned_wrapper_to_signed_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " + ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) + <$Rhs>::new(7),
                <$Lhs>::new(17),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::ZERO + <$Rhs>::new(-1),
                <$Lhs>::ZERO,
                "{label} saturate at ZERO"
            );
            assert_eq!(
                <$Lhs>::MAX + <$Rhs>::new(1),
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(10);
            acc += <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(17), "{label} +=");
        }};
    }
    for_each_unsigned_lhs_signed_rhs_wrapper!(check);
}

#[test]
fn sub_unsigned_wrapper_to_signed_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " - ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) - <$Rhs>::new(7),
                <$Lhs>::new(3),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(5) - <$Rhs>::new(-3),
                <$Lhs>::new(8),
                "{label} negative rhs"
            );
            assert_eq!(
                <$Lhs>::ZERO - <$Rhs>::new(1),
                <$Lhs>::ZERO,
                "{label} saturate at ZERO"
            );

            let mut acc = <$Lhs>::new(10);
            acc -= <$Rhs>::new(7);
            assert_eq!(acc, <$Lhs>::new(3), "{label} -=");
        }};
    }
    for_each_unsigned_lhs_signed_rhs_wrapper!(check);
}

#[test]
fn add_signed_wrapper_to_unsigned_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " + ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10) + 7 as $rhs,
                <$Lhs>::new(17),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MAX + 1 as $rhs,
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(10);
            acc += 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(17), "{label} +=");
        }};
    }
    for_each_signed_wrapper_x_unsigned_primitive!(check);
}

#[test]
fn sub_signed_wrapper_to_unsigned_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " - ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10) - 7 as $rhs,
                <$Lhs>::new(3),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::MIN - 1 as $rhs,
                <$Lhs>::MIN,
                "{label} saturate at MIN"
            );

            let mut acc = <$Lhs>::new(10);
            acc -= 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(3), "{label} -=");
        }};
    }
    for_each_signed_wrapper_x_unsigned_primitive!(check);
}

#[test]
fn add_unsigned_wrapper_to_signed_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " + ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10) + 7 as $rhs,
                <$Lhs>::new(17),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::ZERO + -1 as $rhs,
                <$Lhs>::ZERO,
                "{label} saturate at ZERO"
            );
            assert_eq!(
                <$Lhs>::MAX + 1 as $rhs,
                <$Lhs>::MAX,
                "{label} saturate at MAX"
            );

            let mut acc = <$Lhs>::new(10);
            acc += 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(17), "{label} +=");
        }};
    }
    for_each_unsigned_wrapper_x_signed_primitive!(check);
}

#[test]
fn sub_unsigned_wrapper_to_signed_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " - ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10) - 7 as $rhs,
                <$Lhs>::new(3),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(5) - -3 as $rhs,
                <$Lhs>::new(8),
                "{label} negative rhs"
            );
            assert_eq!(
                <$Lhs>::ZERO - 1 as $rhs,
                <$Lhs>::ZERO,
                "{label} saturate at ZERO"
            );

            let mut acc = <$Lhs>::new(10);
            acc -= 7 as $rhs;
            assert_eq!(acc, <$Lhs>::new(3), "{label} -=");
        }};
    }
    for_each_unsigned_wrapper_x_signed_primitive!(check);
}

// ============================================================================
// TryDiv / TryRem (and assign variants), wrapper × wrapper and wrapper × prim.
// ============================================================================

#[test]
fn try_div_signed_wrapper_to_signed_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " try_div ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10).try_div(<$Rhs>::new(3)),
                Ok(<$Lhs>::new(3)),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(10).try_div(<$Rhs>::ZERO),
                Err(DivError::DivisionByZero),
                "{label} div by zero",
            );

            let mut acc = <$Lhs>::new(10);
            acc.try_div_assign(<$Rhs>::new(3)).unwrap();
            assert_eq!(acc, <$Lhs>::new(3), "{label} try_div_assign");
            let err = acc.try_div_assign(<$Rhs>::ZERO);
            assert_eq!(err, Err(DivError::DivisionByZero), "{label} assign err");
            assert_eq!(acc, <$Lhs>::new(3), "{label} assign unchanged on err");
        }};
    }
    for_each_signed_wrapper_pair!(check);
}

#[test]
fn try_rem_signed_wrapper_to_signed_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " try_rem ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10).try_rem(<$Rhs>::new(3)),
                Ok(<$Lhs>::new(1)),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(10).try_rem(<$Rhs>::ZERO),
                Err(DivError::DivisionByZero),
                "{label} rem by zero",
            );

            let mut acc = <$Lhs>::new(10);
            acc.try_rem_assign(<$Rhs>::new(3)).unwrap();
            assert_eq!(acc, <$Lhs>::new(1), "{label} try_rem_assign");
            let err = acc.try_rem_assign(<$Rhs>::ZERO);
            assert_eq!(err, Err(DivError::DivisionByZero), "{label} assign err");
            assert_eq!(acc, <$Lhs>::new(1), "{label} assign unchanged on err");
        }};
    }
    for_each_signed_wrapper_pair!(check);
}

#[test]
fn try_div_unsigned_wrapper_to_unsigned_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " try_div ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10).try_div(<$Rhs>::new(3)),
                Ok(<$Lhs>::new(3)),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(10).try_div(<$Rhs>::ZERO),
                Err(DivError::DivisionByZero),
                "{label} div by zero",
            );

            let mut acc = <$Lhs>::new(10);
            acc.try_div_assign(<$Rhs>::new(3)).unwrap();
            assert_eq!(acc, <$Lhs>::new(3), "{label} try_div_assign");
        }};
    }
    for_each_unsigned_wrapper_pair!(check);
}

#[test]
fn try_rem_unsigned_wrapper_to_unsigned_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " try_rem ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10).try_rem(<$Rhs>::new(3)),
                Ok(<$Lhs>::new(1)),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(10).try_rem(<$Rhs>::ZERO),
                Err(DivError::DivisionByZero),
                "{label} rem by zero",
            );

            let mut acc = <$Lhs>::new(10);
            acc.try_rem_assign(<$Rhs>::new(3)).unwrap();
            assert_eq!(acc, <$Lhs>::new(1), "{label} try_rem_assign");
        }};
    }
    for_each_unsigned_wrapper_pair!(check);
}

#[test]
fn try_div_signed_wrapper_to_signed_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " try_div ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10).try_div(3 as $rhs),
                Ok(<$Lhs>::new(3)),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(10).try_div(0 as $rhs),
                Err(DivError::DivisionByZero),
                "{label} div by zero",
            );

            let mut acc = <$Lhs>::new(10);
            acc.try_div_assign(3 as $rhs).unwrap();
            assert_eq!(acc, <$Lhs>::new(3), "{label} try_div_assign");
        }};
    }
    for_each_signed_wrapper_x_signed_primitive!(check);
}

#[test]
fn try_rem_signed_wrapper_to_signed_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " try_rem ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10).try_rem(3 as $rhs),
                Ok(<$Lhs>::new(1)),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(10).try_rem(0 as $rhs),
                Err(DivError::DivisionByZero),
                "{label} rem by zero",
            );

            let mut acc = <$Lhs>::new(10);
            acc.try_rem_assign(3 as $rhs).unwrap();
            assert_eq!(acc, <$Lhs>::new(1), "{label} try_rem_assign");
        }};
    }
    for_each_signed_wrapper_x_signed_primitive!(check);
}

#[test]
fn try_div_unsigned_wrapper_to_unsigned_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " try_div ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10).try_div(3 as $rhs),
                Ok(<$Lhs>::new(3)),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(10).try_div(0 as $rhs),
                Err(DivError::DivisionByZero),
                "{label} div by zero",
            );

            let mut acc = <$Lhs>::new(10);
            acc.try_div_assign(3 as $rhs).unwrap();
            assert_eq!(acc, <$Lhs>::new(3), "{label} try_div_assign");
        }};
    }
    for_each_unsigned_wrapper_x_unsigned_primitive!(check);
}

#[test]
fn try_rem_unsigned_wrapper_to_unsigned_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " try_rem ", stringify!($rhs));
            assert_eq!(
                <$Lhs>::new(10).try_rem(3 as $rhs),
                Ok(<$Lhs>::new(1)),
                "{label} normal"
            );
            assert_eq!(
                <$Lhs>::new(10).try_rem(0 as $rhs),
                Err(DivError::DivisionByZero),
                "{label} rem by zero",
            );

            let mut acc = <$Lhs>::new(10);
            acc.try_rem_assign(3 as $rhs).unwrap();
            assert_eq!(acc, <$Lhs>::new(1), "{label} try_rem_assign");
        }};
    }
    for_each_unsigned_wrapper_x_unsigned_primitive!(check);
}

// `try_rem`'s widened i128 path can surface `DivError::Overflow`, but only at
// Si128::MIN % Si128(-1) (and the equivalent i128 primitive RHS) since smaller
// signed widths fit into i128 with headroom.
#[test]
fn try_rem_signed_overflow_only_at_widest_width() {
    assert_eq!(Si128::MIN.try_rem(Si128::new(-1)), Err(DivError::Overflow));
    assert_eq!(Si128::MIN.try_rem(-1_i128), Err(DivError::Overflow));
    // Smaller widths widen to i128 without overflow, so Si8::MIN % -1 succeeds.
    assert_eq!(Si8::MIN.try_rem(Si8::new(-1)), Ok(Si8::ZERO));
    assert_eq!(Si64::MIN.try_rem(Si64::new(-1)), Ok(Si64::ZERO));
}

// ============================================================================
// Panicking Div/Rem (feature-gated): mirror the TryDiv/TryRem matrix on the
// happy path. Division-by-zero panics and is not exercised here.
// ============================================================================

#[cfg(feature = "panicking-ops")]
#[test]
fn panicking_div_rem_signed_wrapper_to_signed_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " /,% ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) / <$Rhs>::new(3),
                <$Lhs>::new(3),
                "{label} div"
            );
            assert_eq!(
                <$Lhs>::new(10) % <$Rhs>::new(3),
                <$Lhs>::new(1),
                "{label} rem"
            );

            let mut acc = <$Lhs>::new(10);
            acc /= <$Rhs>::new(3);
            assert_eq!(acc, <$Lhs>::new(3), "{label} /=");
            let mut acc = <$Lhs>::new(10);
            acc %= <$Rhs>::new(3);
            assert_eq!(acc, <$Lhs>::new(1), "{label} %=");
        }};
    }
    for_each_signed_wrapper_pair!(check);
}

#[cfg(feature = "panicking-ops")]
#[test]
fn panicking_div_rem_unsigned_wrapper_to_unsigned_wrapper() {
    macro_rules! check {
        ($Lhs:ident, $Rhs:ident) => {{
            let label = concat!(stringify!($Lhs), " /,% ", stringify!($Rhs));
            assert_eq!(
                <$Lhs>::new(10) / <$Rhs>::new(3),
                <$Lhs>::new(3),
                "{label} div"
            );
            assert_eq!(
                <$Lhs>::new(10) % <$Rhs>::new(3),
                <$Lhs>::new(1),
                "{label} rem"
            );

            let mut acc = <$Lhs>::new(10);
            acc /= <$Rhs>::new(3);
            assert_eq!(acc, <$Lhs>::new(3), "{label} /=");
            let mut acc = <$Lhs>::new(10);
            acc %= <$Rhs>::new(3);
            assert_eq!(acc, <$Lhs>::new(1), "{label} %=");
        }};
    }
    for_each_unsigned_wrapper_pair!(check);
}

#[cfg(feature = "panicking-ops")]
#[test]
fn panicking_div_rem_signed_wrapper_to_signed_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " /,% ", stringify!($rhs));
            assert_eq!(<$Lhs>::new(10) / 3 as $rhs, <$Lhs>::new(3), "{label} div");
            assert_eq!(<$Lhs>::new(10) % 3 as $rhs, <$Lhs>::new(1), "{label} rem");

            let mut acc = <$Lhs>::new(10);
            acc /= 3 as $rhs;
            assert_eq!(acc, <$Lhs>::new(3), "{label} /=");
            let mut acc = <$Lhs>::new(10);
            acc %= 3 as $rhs;
            assert_eq!(acc, <$Lhs>::new(1), "{label} %=");
        }};
    }
    for_each_signed_wrapper_x_signed_primitive!(check);
}

#[cfg(feature = "panicking-ops")]
#[test]
fn panicking_div_rem_unsigned_wrapper_to_unsigned_primitive() {
    macro_rules! check {
        ($Lhs:ident, $rhs:ty) => {{
            let label = concat!(stringify!($Lhs), " /,% ", stringify!($rhs));
            assert_eq!(<$Lhs>::new(10) / 3 as $rhs, <$Lhs>::new(3), "{label} div");
            assert_eq!(<$Lhs>::new(10) % 3 as $rhs, <$Lhs>::new(1), "{label} rem");

            let mut acc = <$Lhs>::new(10);
            acc /= 3 as $rhs;
            assert_eq!(acc, <$Lhs>::new(3), "{label} /=");
            let mut acc = <$Lhs>::new(10);
            acc %= 3 as $rhs;
            assert_eq!(acc, <$Lhs>::new(1), "{label} %=");
        }};
    }
    for_each_unsigned_wrapper_x_unsigned_primitive!(check);
}

// ============================================================================
// Unary: shifts and negation.
// ============================================================================

#[test]
fn shl_shr_signed_wrappers() {
    macro_rules! check {
        ($Lhs:ident) => {{
            let label = stringify!($Lhs);
            // Trivial: zero value or zero shift returns self.
            assert_eq!(<$Lhs>::ZERO << 3, <$Lhs>::ZERO, "{label} 0 << n");
            assert_eq!(<$Lhs>::new(5) << 0, <$Lhs>::new(5), "{label} v << 0");
            // Normal shift with sufficient headroom.
            assert_eq!(
                <$Lhs>::new(1) << 2,
                <$Lhs>::new(4),
                "{label} positive normal"
            );
            assert_eq!(
                <$Lhs>::new(-1) << 2,
                <$Lhs>::new(-4),
                "{label} negative normal"
            );
            // Oversized rhs: positive saturates to MAX, negative to MIN.
            assert_eq!(
                <$Lhs>::new(1) << <$Lhs>::BITS,
                <$Lhs>::MAX,
                "{label} >= BITS positive"
            );
            assert_eq!(
                <$Lhs>::new(-1) << <$Lhs>::BITS,
                <$Lhs>::MIN,
                "{label} >= BITS negative"
            );
            // Insufficient headroom saturates.
            assert_eq!(<$Lhs>::MAX << 1, <$Lhs>::MAX, "{label} positive overflow");
            assert_eq!(<$Lhs>::MIN << 1, <$Lhs>::MIN, "{label} negative overflow");

            let mut acc = <$Lhs>::new(1);
            acc <<= 2;
            assert_eq!(acc, <$Lhs>::new(4), "{label} <<=");

            // Shr arithmetic shift.
            assert_eq!(<$Lhs>::new(8) >> 2, <$Lhs>::new(2), "{label} positive >> n");
            assert_eq!(
                <$Lhs>::new(-8) >> 2,
                <$Lhs>::new(-2),
                "{label} negative >> n"
            );
            // Oversized rhs: positive collapses to 0, negative to -1.
            assert_eq!(
                <$Lhs>::new(8) >> <$Lhs>::BITS,
                <$Lhs>::ZERO,
                "{label} >= BITS positive"
            );
            assert_eq!(
                <$Lhs>::new(-1) >> <$Lhs>::BITS,
                <$Lhs>::new(-1),
                "{label} >= BITS negative"
            );

            let mut acc = <$Lhs>::new(8);
            acc >>= 2;
            assert_eq!(acc, <$Lhs>::new(2), "{label} >>=");
        }};
    }
    for_each_signed_wrapper!(check);
}

#[test]
fn shl_shr_unsigned_wrappers() {
    macro_rules! check {
        ($Lhs:ident) => {{
            let label = stringify!($Lhs);
            // Normal shift.
            assert_eq!(<$Lhs>::new(1) << 3, <$Lhs>::new(8), "{label} normal <<");
            // Oversized rhs saturates to MAX.
            assert_eq!(
                <$Lhs>::new(1) << <$Lhs>::BITS,
                <$Lhs>::MAX,
                "{label} >= BITS"
            );
            // High bits set: would overflow, saturates.
            assert_eq!(<$Lhs>::MAX << 1, <$Lhs>::MAX, "{label} overflow");

            let mut acc = <$Lhs>::new(1);
            acc <<= 3;
            assert_eq!(acc, <$Lhs>::new(8), "{label} <<=");

            // Normal shr.
            assert_eq!(<$Lhs>::new(8) >> 2, <$Lhs>::new(2), "{label} normal >>");
            // Oversized rhs collapses to 0.
            assert_eq!(<$Lhs>::MAX >> <$Lhs>::BITS, <$Lhs>::ZERO, "{label} >= BITS");

            let mut acc = <$Lhs>::new(8);
            acc >>= 2;
            assert_eq!(acc, <$Lhs>::new(2), "{label} >>=");
        }};
    }
    for_each_unsigned_wrapper!(check);
}

#[test]
fn neg_signed_wrappers() {
    macro_rules! check {
        ($Lhs:ident) => {{
            let label = stringify!($Lhs);
            assert_eq!(-<$Lhs>::new(5), <$Lhs>::new(-5), "{label} positive");
            assert_eq!(-<$Lhs>::new(-5), <$Lhs>::new(5), "{label} negative");
            // MIN saturates rather than overflowing to "-MIN".
            assert_eq!(-<$Lhs>::MIN, <$Lhs>::MAX, "{label} MIN saturates");
        }};
    }
    for_each_signed_wrapper!(check);
}
