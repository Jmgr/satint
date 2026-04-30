use rstest::rstest;
use satint::{
    SaturatingFrom, SaturatingInto, Si8, Si16, Si32, Si64, Si128, Su8, Su16, Su32, Su64, Su128,
    si8, si16, si32, si64, si128, su8, su16, su32, su64, su128,
};

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
            fn primitive_saturating_conversions() {
                let same_width: $scalar = (42 as $primitive).saturating_into();
                let from_isize: $scalar = 42_isize.saturating_into();
                let from_usize: $scalar = 42_usize.saturating_into();
                let negative: $scalar = (-10_i32).saturating_into();
                let wide: $scalar = u128::MAX.saturating_into();

                assert_eq!(same_width, $ctor(42 as $primitive));
                assert_eq!(from_isize, $ctor(42 as $primitive));
                assert_eq!(from_usize, $ctor(42 as $primitive));
                assert_eq!(negative, <$scalar>::ZERO);
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
            fn neg() {
                assert_eq!(-$ctor(5), $ctor(-5));
                assert_eq!(-<$scalar>::MIN, <$scalar>::MAX);
            }

            #[test]
            fn primitive_saturating_conversions() {
                let same_width: $scalar = (42 as $primitive).saturating_into();
                let from_isize: $scalar = 42_isize.saturating_into();
                let from_usize: $scalar = 42_usize.saturating_into();
                let low: $scalar = i128::MIN.saturating_into();
                let high: $scalar = u128::MAX.saturating_into();

                assert_eq!(same_width, $ctor(42 as $primitive));
                assert_eq!(from_isize, $ctor(42 as $primitive));
                assert_eq!(from_usize, $ctor(42 as $primitive));
                assert_eq!(low, <$scalar>::MIN);
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
