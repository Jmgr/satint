use rstest::rstest;
use satint::{
    DivError, SaturatingFrom, SaturatingInto, Si8, Si16, Si32, Si64, Si128, Su8, Su16, Su32, Su64,
    Su128, TryDiv, TryDivAssign, TryRem, TryRemAssign, si8, si16, si32, si64, si128, su8, su16,
    su32, su64, su128,
};
use std::cmp::Ordering;

macro_rules! unsigned_operator_tests {
    ($module:ident, $scalar:ty, $ctor:ident, $primitive:ty) => {
        mod $module {
            use super::*;

            #[rstest]
            #[case::normal($ctor(10), $ctor(7), $ctor(17))]
            #[case::saturating(<$scalar>::MAX, $ctor(1), <$scalar>::MAX)]
            fn add_wrapper_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $scalar,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs + rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(10), 7 as $primitive, $ctor(17))]
            #[case::saturating(<$scalar>::MAX, 1 as $primitive, <$scalar>::MAX)]
            fn add_primitive_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $primitive,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs + rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(10), $ctor(7), $ctor(3))]
            #[case::saturating(<$scalar>::ZERO, $ctor(1), <$scalar>::ZERO)]
            fn sub_wrapper_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $scalar,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs - rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(10), 7 as $primitive, $ctor(3))]
            #[case::saturating(<$scalar>::ZERO, 1 as $primitive, <$scalar>::ZERO)]
            fn sub_primitive_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $primitive,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs - rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(6), $ctor(7), $ctor(42))]
            #[case::saturating(<$scalar>::MAX, $ctor(2), <$scalar>::MAX)]
            fn mul_wrapper_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $scalar,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs * rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(6), 7 as $primitive, $ctor(42))]
            #[case::saturating(<$scalar>::MAX, 2 as $primitive, <$scalar>::MAX)]
            fn mul_primitive_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $primitive,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs * rhs, expected);
            }

            #[test]
            fn assign_wrapper_rhs() {
                let mut add = $ctor(10);
                add += $ctor(7);
                assert_eq!(add, $ctor(17));

                let mut add_saturating = <$scalar>::MAX;
                add_saturating += $ctor(1);
                assert_eq!(add_saturating, <$scalar>::MAX);

                let mut sub = $ctor(10);
                sub -= $ctor(7);
                assert_eq!(sub, $ctor(3));

                let mut sub_saturating = <$scalar>::ZERO;
                sub_saturating -= $ctor(1);
                assert_eq!(sub_saturating, <$scalar>::ZERO);

                let mut mul = $ctor(6);
                mul *= $ctor(7);
                assert_eq!(mul, $ctor(42));

                let mut mul_saturating = <$scalar>::MAX;
                mul_saturating *= $ctor(2);
                assert_eq!(mul_saturating, <$scalar>::MAX);
            }

            #[test]
            fn assign_primitive_rhs() {
                let mut add = $ctor(10);
                add += 7 as $primitive;
                assert_eq!(add, $ctor(17));

                let mut add_saturating = <$scalar>::MAX;
                add_saturating += 1 as $primitive;
                assert_eq!(add_saturating, <$scalar>::MAX);

                let mut sub = $ctor(10);
                sub -= 7 as $primitive;
                assert_eq!(sub, $ctor(3));

                let mut sub_saturating = <$scalar>::ZERO;
                sub_saturating -= 1 as $primitive;
                assert_eq!(sub_saturating, <$scalar>::ZERO);

                let mut mul = $ctor(6);
                mul *= 7 as $primitive;
                assert_eq!(mul, $ctor(42));

                let mut mul_saturating = <$scalar>::MAX;
                mul_saturating *= 2 as $primitive;
                assert_eq!(mul_saturating, <$scalar>::MAX);
            }

            #[test]
            fn checked_div_rem() {
                assert_eq!($ctor(20).checked_div($ctor(3)), Some($ctor(6)));
                assert_eq!($ctor(20).checked_rem($ctor(3)), Some($ctor(2)));
                assert_eq!($ctor(20).checked_div($ctor(0)), None);
                assert_eq!($ctor(20).checked_rem($ctor(0)), None);
            }

            #[test]
            fn try_div_wrapper_rhs() {
                assert_eq!($ctor(20).try_div($ctor(3)), Ok($ctor(6)));
                assert_eq!($ctor(20).try_div($ctor(0)), Err(DivError::DivisionByZero));
            }

            #[test]
            fn try_rem_wrapper_rhs() {
                assert_eq!($ctor(20).try_rem($ctor(3)), Ok($ctor(2)));
                assert_eq!($ctor(20).try_rem($ctor(0)), Err(DivError::DivisionByZero));
            }

            #[test]
            fn try_div_primitive_rhs() {
                assert_eq!($ctor(20).try_div(3 as $primitive), Ok($ctor(6)));
                assert_eq!(
                    $ctor(20).try_div(0 as $primitive),
                    Err(DivError::DivisionByZero)
                );
            }

            #[test]
            fn try_rem_primitive_rhs() {
                assert_eq!($ctor(20).try_rem(3 as $primitive), Ok($ctor(2)));
                assert_eq!(
                    $ctor(20).try_rem(0 as $primitive),
                    Err(DivError::DivisionByZero)
                );
            }

            #[test]
            fn try_div_assign_wrapper_rhs() {
                let mut div = $ctor(20);
                assert_eq!(div.try_div_assign($ctor(3)), Ok(()));
                assert_eq!(div, $ctor(6));

                let mut div_by_zero = $ctor(20);
                assert_eq!(
                    div_by_zero.try_div_assign($ctor(0)),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(div_by_zero, $ctor(20));
            }

            #[test]
            fn try_rem_assign_wrapper_rhs() {
                let mut rem = $ctor(20);
                assert_eq!(rem.try_rem_assign($ctor(3)), Ok(()));
                assert_eq!(rem, $ctor(2));

                let mut rem_by_zero = $ctor(20);
                assert_eq!(
                    rem_by_zero.try_rem_assign($ctor(0)),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(rem_by_zero, $ctor(20));
            }

            #[test]
            fn try_div_assign_primitive_rhs() {
                let mut div = $ctor(20);
                assert_eq!(div.try_div_assign(3 as $primitive), Ok(()));
                assert_eq!(div, $ctor(6));

                let mut div_by_zero = $ctor(20);
                assert_eq!(
                    div_by_zero.try_div_assign(0 as $primitive),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(div_by_zero, $ctor(20));
            }

            #[test]
            fn try_rem_assign_primitive_rhs() {
                let mut rem = $ctor(20);
                assert_eq!(rem.try_rem_assign(3 as $primitive), Ok(()));
                assert_eq!(rem, $ctor(2));

                let mut rem_by_zero = $ctor(20);
                assert_eq!(
                    rem_by_zero.try_rem_assign(0 as $primitive),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(rem_by_zero, $ctor(20));
            }

            #[test]
            fn wrapper_helpers() {
                assert_eq!(format!("{:?}", $ctor(7)), "Su(7)");
                assert_eq!($ctor(7).to_string(), "7");

                assert_eq!(<$scalar>::from(7 as $primitive), $ctor(7));

                let primitive: $primitive = $ctor(7).into();
                assert_eq!(primitive, 7);

                assert!($ctor(7) == 7 as $primitive);
                assert!(7 as $primitive == $ctor(7));
                assert_eq!(
                    $ctor(7).partial_cmp(&(8 as $primitive)),
                    Some(Ordering::Less)
                );
                assert_eq!(
                    (7 as $primitive).partial_cmp(&$ctor(8)),
                    Some(Ordering::Less)
                );

                assert_eq!([$ctor(2), $ctor(3)].into_iter().sum::<$scalar>(), $ctor(5));
                assert_eq!([$ctor(2), $ctor(3)].iter().sum::<$scalar>(), $ctor(5));
                assert_eq!(
                    [$ctor(2), $ctor(3)].into_iter().product::<$scalar>(),
                    $ctor(6)
                );
                assert_eq!([$ctor(2), $ctor(3)].iter().product::<$scalar>(), $ctor(6));
            }

            #[test]
            fn primitive_saturating_conversions() {
                fn convert_identity<T: SaturatingFrom<T>>(value: T) -> T {
                    T::saturating_from(value)
                }

                let identity: $primitive = (42 as $primitive).saturating_into();
                let generic_identity = convert_identity(std::hint::black_box(42 as $primitive));
                let same_width: $scalar = (42 as $primitive).saturating_into();
                let from_isize: $scalar = 42_isize.saturating_into();
                let from_usize: $scalar = 42_usize.saturating_into();
                let negative: $scalar = (-10_i32).saturating_into();
                let large_signed: $scalar = i128::MAX.saturating_into();
                let wide: $scalar = u128::MAX.saturating_into();
                let expected_large_signed = if (<$primitive>::MAX as u128) < (i128::MAX as u128) {
                    <$scalar>::MAX
                } else {
                    $ctor(i128::MAX as $primitive)
                };

                assert_eq!(identity, 42 as $primitive);
                assert_eq!(generic_identity, 42 as $primitive);
                assert_eq!(same_width, $ctor(42 as $primitive));
                assert_eq!(from_isize, $ctor(42 as $primitive));
                assert_eq!(from_usize, $ctor(42 as $primitive));
                assert_eq!(negative, <$scalar>::ZERO);
                assert_eq!(large_signed, expected_large_signed);
                assert_eq!(wide, <$scalar>::MAX);
                assert_eq!(
                    <$scalar>::saturating_from(42 as $primitive),
                    $ctor(42 as $primitive)
                );
            }
        }
    };
}

macro_rules! signed_operator_tests {
    ($module:ident, $scalar:ty, $ctor:ident, $primitive:ty) => {
        mod $module {
            use super::*;

            #[rstest]
            #[case::normal($ctor(10), $ctor(-7), $ctor(3))]
            #[case::saturating(<$scalar>::MAX, $ctor(1), <$scalar>::MAX)]
            fn add_wrapper_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $scalar,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs + rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(10), -7 as $primitive, $ctor(3))]
            #[case::saturating(<$scalar>::MAX, 1 as $primitive, <$scalar>::MAX)]
            fn add_primitive_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $primitive,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs + rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(10), $ctor(7), $ctor(3))]
            #[case::saturating(<$scalar>::MIN, $ctor(1), <$scalar>::MIN)]
            fn sub_wrapper_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $scalar,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs - rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(10), 7 as $primitive, $ctor(3))]
            #[case::saturating(<$scalar>::MIN, 1 as $primitive, <$scalar>::MIN)]
            fn sub_primitive_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $primitive,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs - rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(6), $ctor(-7), $ctor(-42))]
            #[case::positive_saturating(<$scalar>::MAX, $ctor(2), <$scalar>::MAX)]
            #[case::negative_saturating(<$scalar>::MIN, $ctor(2), <$scalar>::MIN)]
            fn mul_wrapper_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $scalar,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs * rhs, expected);
            }

            #[rstest]
            #[case::normal($ctor(6), -7 as $primitive, $ctor(-42))]
            #[case::positive_saturating(<$scalar>::MAX, 2 as $primitive, <$scalar>::MAX)]
            #[case::negative_saturating(<$scalar>::MIN, 2 as $primitive, <$scalar>::MIN)]
            fn mul_primitive_rhs(
                #[case] lhs: $scalar,
                #[case] rhs: $primitive,
                #[case] expected: $scalar,
            ) {
                assert_eq!(lhs * rhs, expected);
            }

            #[test]
            fn assign_wrapper_rhs() {
                let mut add = $ctor(10);
                add += $ctor(-7);
                assert_eq!(add, $ctor(3));

                let mut add_saturating = <$scalar>::MAX;
                add_saturating += $ctor(1);
                assert_eq!(add_saturating, <$scalar>::MAX);

                let mut sub = $ctor(10);
                sub -= $ctor(7);
                assert_eq!(sub, $ctor(3));

                let mut sub_saturating = <$scalar>::MIN;
                sub_saturating -= $ctor(1);
                assert_eq!(sub_saturating, <$scalar>::MIN);

                let mut mul = $ctor(6);
                mul *= $ctor(-7);
                assert_eq!(mul, $ctor(-42));

                let mut mul_saturating = <$scalar>::MIN;
                mul_saturating *= $ctor(2);
                assert_eq!(mul_saturating, <$scalar>::MIN);
            }

            #[test]
            fn assign_primitive_rhs() {
                let mut add = $ctor(10);
                add += -7 as $primitive;
                assert_eq!(add, $ctor(3));

                let mut add_saturating = <$scalar>::MAX;
                add_saturating += 1 as $primitive;
                assert_eq!(add_saturating, <$scalar>::MAX);

                let mut sub = $ctor(10);
                sub -= 7 as $primitive;
                assert_eq!(sub, $ctor(3));

                let mut sub_saturating = <$scalar>::MIN;
                sub_saturating -= 1 as $primitive;
                assert_eq!(sub_saturating, <$scalar>::MIN);

                let mut mul = $ctor(6);
                mul *= -7 as $primitive;
                assert_eq!(mul, $ctor(-42));

                let mut mul_saturating = <$scalar>::MIN;
                mul_saturating *= 2 as $primitive;
                assert_eq!(mul_saturating, <$scalar>::MIN);
            }

            #[test]
            fn checked_div_rem() {
                assert_eq!($ctor(20).checked_div($ctor(3)), Some($ctor(6)));
                assert_eq!($ctor(20).checked_rem($ctor(3)), Some($ctor(2)));
                assert_eq!($ctor(20).checked_div($ctor(0)), None);
                assert_eq!($ctor(20).checked_rem($ctor(0)), None);
                assert_eq!(<$scalar>::MIN.checked_div($ctor(-1)), None);
                assert_eq!(<$scalar>::MIN.checked_rem($ctor(-1)), None);
            }

            #[test]
            fn try_div_wrapper_rhs() {
                assert_eq!($ctor(20).try_div($ctor(3)), Ok($ctor(6)));
                assert_eq!($ctor(20).try_div($ctor(0)), Err(DivError::DivisionByZero));
                assert_eq!(<$scalar>::MIN.try_div($ctor(-1)), Err(DivError::Overflow));
            }

            #[test]
            fn try_rem_wrapper_rhs() {
                assert_eq!($ctor(20).try_rem($ctor(3)), Ok($ctor(2)));
                assert_eq!($ctor(20).try_rem($ctor(0)), Err(DivError::DivisionByZero));
                assert_eq!(<$scalar>::MIN.try_rem($ctor(-1)), Err(DivError::Overflow));
            }

            #[test]
            fn try_div_primitive_rhs() {
                assert_eq!($ctor(20).try_div(3 as $primitive), Ok($ctor(6)));
                assert_eq!(
                    $ctor(20).try_div(0 as $primitive),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(
                    <$scalar>::MIN.try_div(-1 as $primitive),
                    Err(DivError::Overflow)
                );
            }

            #[test]
            fn try_rem_primitive_rhs() {
                assert_eq!($ctor(20).try_rem(3 as $primitive), Ok($ctor(2)));
                assert_eq!(
                    $ctor(20).try_rem(0 as $primitive),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(
                    <$scalar>::MIN.try_rem(-1 as $primitive),
                    Err(DivError::Overflow)
                );
            }

            #[test]
            fn try_div_assign_wrapper_rhs() {
                let mut div = $ctor(20);
                assert_eq!(div.try_div_assign($ctor(3)), Ok(()));
                assert_eq!(div, $ctor(6));

                let mut div_by_zero = $ctor(20);
                assert_eq!(
                    div_by_zero.try_div_assign($ctor(0)),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(div_by_zero, $ctor(20));

                let mut div_overflow = <$scalar>::MIN;
                assert_eq!(
                    div_overflow.try_div_assign($ctor(-1)),
                    Err(DivError::Overflow)
                );
                assert_eq!(div_overflow, <$scalar>::MIN);
            }

            #[test]
            fn try_rem_assign_wrapper_rhs() {
                let mut rem = $ctor(20);
                assert_eq!(rem.try_rem_assign($ctor(3)), Ok(()));
                assert_eq!(rem, $ctor(2));

                let mut rem_by_zero = $ctor(20);
                assert_eq!(
                    rem_by_zero.try_rem_assign($ctor(0)),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(rem_by_zero, $ctor(20));

                let mut rem_overflow = <$scalar>::MIN;
                assert_eq!(
                    rem_overflow.try_rem_assign($ctor(-1)),
                    Err(DivError::Overflow)
                );
                assert_eq!(rem_overflow, <$scalar>::MIN);
            }

            #[test]
            fn try_div_assign_primitive_rhs() {
                let mut div = $ctor(20);
                assert_eq!(div.try_div_assign(3 as $primitive), Ok(()));
                assert_eq!(div, $ctor(6));

                let mut div_by_zero = $ctor(20);
                assert_eq!(
                    div_by_zero.try_div_assign(0 as $primitive),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(div_by_zero, $ctor(20));

                let mut div_overflow = <$scalar>::MIN;
                assert_eq!(
                    div_overflow.try_div_assign(-1 as $primitive),
                    Err(DivError::Overflow)
                );
                assert_eq!(div_overflow, <$scalar>::MIN);
            }

            #[test]
            fn try_rem_assign_primitive_rhs() {
                let mut rem = $ctor(20);
                assert_eq!(rem.try_rem_assign(3 as $primitive), Ok(()));
                assert_eq!(rem, $ctor(2));

                let mut rem_by_zero = $ctor(20);
                assert_eq!(
                    rem_by_zero.try_rem_assign(0 as $primitive),
                    Err(DivError::DivisionByZero)
                );
                assert_eq!(rem_by_zero, $ctor(20));

                let mut rem_overflow = <$scalar>::MIN;
                assert_eq!(
                    rem_overflow.try_rem_assign(-1 as $primitive),
                    Err(DivError::Overflow)
                );
                assert_eq!(rem_overflow, <$scalar>::MIN);
            }

            #[test]
            fn neg() {
                assert_eq!(-$ctor(5), $ctor(-5));
                assert_eq!(-<$scalar>::MIN, <$scalar>::MAX);
            }

            #[test]
            fn wrapper_helpers() {
                assert_eq!(format!("{:?}", $ctor(7)), "Si(7)");
                assert_eq!($ctor(7).to_string(), "7");

                assert_eq!(<$scalar>::from(7 as $primitive), $ctor(7));

                let primitive: $primitive = $ctor(7).into();
                assert_eq!(primitive, 7);

                assert!($ctor(7) == 7 as $primitive);
                assert!(7 as $primitive == $ctor(7));
                assert_eq!(
                    $ctor(7).partial_cmp(&(8 as $primitive)),
                    Some(Ordering::Less)
                );
                assert_eq!(
                    (7 as $primitive).partial_cmp(&$ctor(8)),
                    Some(Ordering::Less)
                );

                assert_eq!([$ctor(2), $ctor(3)].into_iter().sum::<$scalar>(), $ctor(5));
                assert_eq!([$ctor(2), $ctor(3)].iter().sum::<$scalar>(), $ctor(5));
                assert_eq!(
                    [$ctor(2), $ctor(3)].into_iter().product::<$scalar>(),
                    $ctor(6)
                );
                assert_eq!([$ctor(2), $ctor(3)].iter().product::<$scalar>(), $ctor(6));
            }

            #[test]
            fn primitive_saturating_conversions() {
                fn convert_identity<T: SaturatingFrom<T>>(value: T) -> T {
                    T::saturating_from(value)
                }

                let identity: $primitive = (42 as $primitive).saturating_into();
                let generic_identity = convert_identity(std::hint::black_box(42 as $primitive));
                let same_width: $scalar = (42 as $primitive).saturating_into();
                let from_isize: $scalar = 42_isize.saturating_into();
                let from_usize: $scalar = 42_usize.saturating_into();
                let low: $scalar = i128::MIN.saturating_into();
                let high_signed: $scalar = i128::MAX.saturating_into();
                let high: $scalar = u128::MAX.saturating_into();

                assert_eq!(identity, 42 as $primitive);
                assert_eq!(generic_identity, 42 as $primitive);
                assert_eq!(same_width, $ctor(42 as $primitive));
                assert_eq!(from_isize, $ctor(42 as $primitive));
                assert_eq!(from_usize, $ctor(42 as $primitive));
                assert_eq!(low, <$scalar>::MIN);
                assert_eq!(high_signed, <$scalar>::MAX);
                assert_eq!(high, <$scalar>::MAX);
                assert_eq!(
                    <$scalar>::saturating_from(42 as $primitive),
                    $ctor(42 as $primitive)
                );
            }
        }
    };
}

macro_rules! widening_operator_tests {
    ($module:ident, $lhs:ty, $lhs_ctor:ident, $rhs:ty, $rhs_ctor:ident) => {
        mod $module {
            use super::*;

            #[test]
            fn add_sub_wrapper_rhs() {
                assert_eq!($lhs_ctor(40) + $rhs_ctor(2), $lhs_ctor(42));
                assert_eq!($lhs_ctor(40) - $rhs_ctor(2), $lhs_ctor(38));
                assert_eq!(<$lhs>::from($rhs_ctor(7)), $lhs_ctor(7));

                let mut add = $lhs_ctor(40);
                add += $rhs_ctor(2);
                assert_eq!(add, $lhs_ctor(42));

                let mut sub = $lhs_ctor(40);
                sub -= $rhs_ctor(2);
                assert_eq!(sub, $lhs_ctor(38));
            }
        }
    };
}

macro_rules! unsigned_to_signed_widening_from_tests {
    ($module:ident, $dst:ty, $dst_ctor:ident, $src:ty, $src_ctor:ident) => {
        mod $module {
            use super::*;

            #[test]
            fn from_unsigned_wrapper_is_lossless() {
                assert_eq!(<$dst>::from($src_ctor(7)), $dst_ctor(7));
            }
        }
    };
}

macro_rules! unsigned_narrowing_conversion_tests {
    ($module:ident, $src:ty, $src_ctor:ident, $dst:ty, $dst_ctor:ident) => {
        mod $module {
            use super::*;

            #[test]
            fn saturating_and_try_from() {
                assert_eq!(<$dst>::saturating_from($src_ctor(42)), $dst_ctor(42));
                assert_eq!(<$dst>::saturating_from($src_ctor(300)), <$dst>::MAX);

                assert_eq!(<$dst>::try_from($src_ctor(42)), Ok($dst_ctor(42)));
                assert!(<$dst>::try_from($src_ctor(300)).is_err());
            }
        }
    };
}

macro_rules! signed_narrowing_conversion_tests {
    ($module:ident, $src:ty, $src_ctor:ident, $dst:ty, $dst_ctor:ident) => {
        mod $module {
            use super::*;

            #[test]
            fn saturating_and_try_from() {
                assert_eq!(<$dst>::saturating_from($src_ctor(42)), $dst_ctor(42));
                assert_eq!(<$dst>::saturating_from($src_ctor(300)), <$dst>::MAX);
                assert_eq!(<$dst>::saturating_from($src_ctor(-300)), <$dst>::MIN);

                assert_eq!(<$dst>::try_from($src_ctor(42)), Ok($dst_ctor(42)));
                assert!(<$dst>::try_from($src_ctor(300)).is_err());
                assert!(<$dst>::try_from($src_ctor(-300)).is_err());
            }
        }
    };
}

macro_rules! signed_to_unsigned_conversion_tests {
    ($module:ident, $src:ty, $src_ctor:ident, $dst:ty, $dst_ctor:ident) => {
        mod $module {
            use super::*;

            #[test]
            fn saturating_and_try_from() {
                assert_eq!(<$dst>::saturating_from($src_ctor(-1)), <$dst>::ZERO);
                assert_eq!(<$dst>::saturating_from($src_ctor(42)), $dst_ctor(42));
                assert_eq!(<$dst>::saturating_from($src_ctor(300)), <$dst>::MAX);

                assert_eq!(<$dst>::try_from($src_ctor(42)), Ok($dst_ctor(42)));
                assert!(<$dst>::try_from($src_ctor(-1)).is_err());
                assert!(<$dst>::try_from($src_ctor(300)).is_err());
            }
        }
    };
}

macro_rules! unsigned_to_signed_fallible_conversion_tests {
    ($module:ident, $src:ty, $src_ctor:ident, $dst:ty, $dst_ctor:ident) => {
        mod $module {
            use super::*;

            #[test]
            fn saturating_and_try_from() {
                assert_eq!(<$dst>::saturating_from($src_ctor(42)), $dst_ctor(42));
                assert_eq!(<$dst>::saturating_from($src_ctor(200)), <$dst>::MAX);

                assert_eq!(<$dst>::try_from($src_ctor(42)), Ok($dst_ctor(42)));
                assert!(<$dst>::try_from($src_ctor(200)).is_err());
            }
        }
    };
}

unsigned_operator_tests!(su8_ops, Su8, su8, u8);
unsigned_operator_tests!(su16_ops, Su16, su16, u16);
unsigned_operator_tests!(su32_ops, Su32, su32, u32);
unsigned_operator_tests!(su64_ops, Su64, su64, u64);
unsigned_operator_tests!(su128_ops, Su128, su128, u128);

signed_operator_tests!(si8_ops, Si8, si8, i8);
signed_operator_tests!(si16_ops, Si16, si16, i16);
signed_operator_tests!(si32_ops, Si32, si32, i32);
signed_operator_tests!(si64_ops, Si64, si64, i64);
signed_operator_tests!(si128_ops, Si128, si128, i128);

widening_operator_tests!(su16_su8_ops, Su16, su16, Su8, su8);
widening_operator_tests!(su32_su8_ops, Su32, su32, Su8, su8);
widening_operator_tests!(su32_su16_ops, Su32, su32, Su16, su16);
widening_operator_tests!(su64_su8_ops, Su64, su64, Su8, su8);
widening_operator_tests!(su64_su16_ops, Su64, su64, Su16, su16);
widening_operator_tests!(su64_su32_ops, Su64, su64, Su32, su32);
widening_operator_tests!(su128_su8_ops, Su128, su128, Su8, su8);
widening_operator_tests!(su128_su16_ops, Su128, su128, Su16, su16);
widening_operator_tests!(su128_su32_ops, Su128, su128, Su32, su32);
widening_operator_tests!(su128_su64_ops, Su128, su128, Su64, su64);

widening_operator_tests!(si16_si8_ops, Si16, si16, Si8, si8);
widening_operator_tests!(si32_si8_ops, Si32, si32, Si8, si8);
widening_operator_tests!(si32_si16_ops, Si32, si32, Si16, si16);
widening_operator_tests!(si64_si8_ops, Si64, si64, Si8, si8);
widening_operator_tests!(si64_si16_ops, Si64, si64, Si16, si16);
widening_operator_tests!(si64_si32_ops, Si64, si64, Si32, si32);
widening_operator_tests!(si128_si8_ops, Si128, si128, Si8, si8);
widening_operator_tests!(si128_si16_ops, Si128, si128, Si16, si16);
widening_operator_tests!(si128_si32_ops, Si128, si128, Si32, si32);
widening_operator_tests!(si128_si64_ops, Si128, si128, Si64, si64);

unsigned_to_signed_widening_from_tests!(si16_su8_from, Si16, si16, Su8, su8);
unsigned_to_signed_widening_from_tests!(si32_su8_from, Si32, si32, Su8, su8);
unsigned_to_signed_widening_from_tests!(si32_su16_from, Si32, si32, Su16, su16);
unsigned_to_signed_widening_from_tests!(si64_su8_from, Si64, si64, Su8, su8);
unsigned_to_signed_widening_from_tests!(si64_su16_from, Si64, si64, Su16, su16);
unsigned_to_signed_widening_from_tests!(si64_su32_from, Si64, si64, Su32, su32);
unsigned_to_signed_widening_from_tests!(si128_su8_from, Si128, si128, Su8, su8);
unsigned_to_signed_widening_from_tests!(si128_su16_from, Si128, si128, Su16, su16);
unsigned_to_signed_widening_from_tests!(si128_su32_from, Si128, si128, Su32, su32);
unsigned_to_signed_widening_from_tests!(si128_su64_from, Si128, si128, Su64, su64);

unsigned_narrowing_conversion_tests!(su16_su8_narrowing, Su16, su16, Su8, su8);
signed_narrowing_conversion_tests!(si16_si8_narrowing, Si16, si16, Si8, si8);
signed_to_unsigned_conversion_tests!(si16_su8_fallible, Si16, si16, Su8, su8);
unsigned_to_signed_fallible_conversion_tests!(su16_si8_fallible, Su16, su16, Si8, si8);

#[test]
fn div_error_display_and_error_trait() {
    assert_eq!(format!("{}", DivError::DivisionByZero), "division by zero");
    assert_eq!(format!("{}", DivError::Overflow), "arithmetic overflow");
    let err: &dyn core::error::Error = &DivError::DivisionByZero;
    assert_eq!(format!("{err}"), "division by zero");
}
