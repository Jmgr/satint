use crate::common::SaturatingFrom;

macro_rules! generate_saturating_from_signed_to_signed {
    ($destination:ty; $($source:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$source> for $destination {
                #[inline]
                fn saturating_from(value: $source) -> Self {
                    (value as i128).clamp(Self::MIN as i128, Self::MAX as i128) as Self
                }
            }
        )+
    };
}

generate_saturating_from_signed_to_signed!(i8; i16, i32, i64, i128, isize);
generate_saturating_from_signed_to_signed!(i16; i8, i32, i64, i128, isize);
generate_saturating_from_signed_to_signed!(i32; i8, i16, i64, i128, isize);
generate_saturating_from_signed_to_signed!(i64; i8, i16, i32, i128, isize);
generate_saturating_from_signed_to_signed!(i128; i8, i16, i32, i64, isize);
generate_saturating_from_signed_to_signed!(isize; i8, i16, i32, i64, i128);

macro_rules! generate_saturating_from_unsigned_to_unsigned {
    ($destination:ty; $($source:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$source> for $destination {
                #[inline]
                fn saturating_from(value: $source) -> Self {
                    (value as u128).min(Self::MAX as u128) as Self
                }
            }
        )+
    };
}

generate_saturating_from_unsigned_to_unsigned!(u8; u16, u32, u64, u128, usize);
generate_saturating_from_unsigned_to_unsigned!(u16; u8, u32, u64, u128, usize);
generate_saturating_from_unsigned_to_unsigned!(u32; u8, u16, u64, u128, usize);
generate_saturating_from_unsigned_to_unsigned!(u64; u8, u16, u32, u128, usize);
generate_saturating_from_unsigned_to_unsigned!(u128; u8, u16, u32, u64, usize);
generate_saturating_from_unsigned_to_unsigned!(usize; u8, u16, u32, u64, u128);

macro_rules! generate_saturating_from_signed_to_unsigned {
    ($destination:ty; $($source:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$source> for $destination {
                #[inline]
                fn saturating_from(value: $source) -> Self {
                    if value < 0 {
                        0
                    } else {
                        (value as u128).min(Self::MAX as u128) as Self
                    }
                }
            }
        )+
    };
}

generate_saturating_from_signed_to_unsigned!(u8; i8, i16, i32, i64, i128, isize);
generate_saturating_from_signed_to_unsigned!(u16; i8, i16, i32, i64, i128, isize);
generate_saturating_from_signed_to_unsigned!(u32; i8, i16, i32, i64, i128, isize);
generate_saturating_from_signed_to_unsigned!(u64; i8, i16, i32, i64, i128, isize);
generate_saturating_from_signed_to_unsigned!(u128; i8, i16, i32, i64, i128, isize);
generate_saturating_from_signed_to_unsigned!(usize; i8, i16, i32, i64, i128, isize);

macro_rules! generate_saturating_from_unsigned_to_signed {
    ($destination:ty; $($source:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$source> for $destination {
                #[inline]
                fn saturating_from(value: $source) -> Self {
                    (value as u128).min(Self::MAX as u128) as Self
                }
            }
        )+
    };
}

generate_saturating_from_unsigned_to_signed!(i8; u8, u16, u32, u64, u128, usize);
generate_saturating_from_unsigned_to_signed!(i16; u8, u16, u32, u64, u128, usize);
generate_saturating_from_unsigned_to_signed!(i32; u8, u16, u32, u64, u128, usize);
generate_saturating_from_unsigned_to_signed!(i64; u8, u16, u32, u64, u128, usize);
generate_saturating_from_unsigned_to_signed!(i128; u8, u16, u32, u64, u128, usize);
generate_saturating_from_unsigned_to_signed!(isize; u8, u16, u32, u64, u128, usize);

macro_rules! generate_saturating_from_float_to_integer {
    ($destination:ty; $($source:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$source> for $destination {
                #[inline]
                fn saturating_from(value: $source) -> Self {
                    value as Self
                }
            }
        )+
    };
}

generate_saturating_from_float_to_integer!(i8; f32, f64);
generate_saturating_from_float_to_integer!(i16; f32, f64);
generate_saturating_from_float_to_integer!(i32; f32, f64);
generate_saturating_from_float_to_integer!(i64; f32, f64);
generate_saturating_from_float_to_integer!(i128; f32, f64);
generate_saturating_from_float_to_integer!(isize; f32, f64);
generate_saturating_from_float_to_integer!(u8; f32, f64);
generate_saturating_from_float_to_integer!(u16; f32, f64);
generate_saturating_from_float_to_integer!(u32; f32, f64);
generate_saturating_from_float_to_integer!(u64; f32, f64);
generate_saturating_from_float_to_integer!(u128; f32, f64);
generate_saturating_from_float_to_integer!(usize; f32, f64);

#[cfg(test)]
mod tests {
    use crate::common::{SaturatingFrom, SaturatingInto};

    #[test]
    fn signed_to_signed_primitives_clamp_to_destination_range() {
        assert_eq!(i8::saturating_from(i16::MIN), i8::MIN);
        assert_eq!(i8::saturating_from(i16::MAX), i8::MAX);
        assert_eq!(i16::saturating_from(i8::MIN), i16::from(i8::MIN));
        assert_eq!(i16::saturating_from(i8::MAX), i16::from(i8::MAX));
        assert_eq!(i32::saturating_from(i128::MIN), i32::MIN);
        assert_eq!(i32::saturating_from(i128::MAX), i32::MAX);
        assert_eq!(isize::saturating_from(i128::MIN), isize::MIN);
        assert_eq!(isize::saturating_from(i128::MAX), isize::MAX);
    }

    #[test]
    fn unsigned_to_unsigned_primitives_clamp_to_destination_range() {
        assert_eq!(u8::saturating_from(u16::MAX), u8::MAX);
        assert_eq!(u16::saturating_from(u8::MAX), u16::from(u8::MAX));
        assert_eq!(u32::saturating_from(u128::MAX), u32::MAX);
        assert_eq!(usize::saturating_from(u128::MAX), usize::MAX);
        assert_eq!(u128::saturating_from(usize::MAX), usize::MAX as u128);
    }

    #[test]
    fn signed_to_unsigned_primitives_clamp_negative_values_to_zero() {
        assert_eq!(u8::saturating_from(-1_i8), 0);
        assert_eq!(u16::saturating_from(i16::MIN), 0);
        assert_eq!(u32::saturating_from(-42_i128), 0);
        assert_eq!(usize::saturating_from(isize::MIN), 0);
    }

    #[test]
    fn signed_to_unsigned_primitives_clamp_to_destination_max() {
        assert_eq!(u8::saturating_from(i16::MAX), u8::MAX);
        assert_eq!(u16::saturating_from(i128::MAX), u16::MAX);
        assert_eq!(usize::saturating_from(i128::MAX), usize::MAX);
        assert_eq!(u128::saturating_from(i128::MAX), i128::MAX as u128);
    }

    #[test]
    fn unsigned_to_signed_primitives_clamp_to_destination_max() {
        assert_eq!(i8::saturating_from(u16::MAX), i8::MAX);
        assert_eq!(i16::saturating_from(u8::MAX), i16::from(u8::MAX));
        assert_eq!(i32::saturating_from(u128::MAX), i32::MAX);
        assert_eq!(isize::saturating_from(u128::MAX), isize::MAX);
        assert_eq!(i128::saturating_from(u128::MAX), i128::MAX);
    }

    #[test]
    fn float_to_integer_primitives_clamp_to_destination_range() {
        // In-range values truncate toward zero.
        assert_eq!(i32::saturating_from(42.7_f32), 42);
        assert_eq!(i32::saturating_from(-42.7_f64), -42);
        assert_eq!(u32::saturating_from(42.7_f32), 42);

        // Out-of-range positive saturates to MAX.
        assert_eq!(i8::saturating_from(1000.0_f32), i8::MAX);
        assert_eq!(u8::saturating_from(1000.0_f64), u8::MAX);
        assert_eq!(i32::saturating_from(f64::MAX), i32::MAX);
        assert_eq!(u128::saturating_from(f64::MAX), u128::MAX);

        // Out-of-range negative saturates to MIN (or 0 for unsigned).
        assert_eq!(i8::saturating_from(-1000.0_f32), i8::MIN);
        assert_eq!(u8::saturating_from(-1.0_f64), 0);
        assert_eq!(i32::saturating_from(f64::MIN), i32::MIN);
        assert_eq!(u128::saturating_from(-f32::MAX), 0);

        // Infinities saturate.
        assert_eq!(i32::saturating_from(f32::INFINITY), i32::MAX);
        assert_eq!(i32::saturating_from(f32::NEG_INFINITY), i32::MIN);
        assert_eq!(u32::saturating_from(f64::INFINITY), u32::MAX);
        assert_eq!(u32::saturating_from(f64::NEG_INFINITY), 0);

        // NaN converts to zero.
        assert_eq!(i32::saturating_from(f32::NAN), 0);
        assert_eq!(u32::saturating_from(f64::NAN), 0);
        assert_eq!(isize::saturating_from(f64::NAN), 0);
        assert_eq!(usize::saturating_from(f32::NAN), 0);
    }

    #[test]
    fn saturating_into_works_between_primitives() {
        let signed_to_unsigned: u32 = (-1_i32).saturating_into();
        assert_eq!(signed_to_unsigned, 0);

        let unsigned_to_signed: i32 = u128::MAX.saturating_into();
        assert_eq!(unsigned_to_signed, i32::MAX);

        let signed_to_signed: i8 = i16::MAX.saturating_into();
        assert_eq!(signed_to_signed, i8::MAX);

        let unsigned_to_unsigned: u8 = u16::MAX.saturating_into();
        assert_eq!(unsigned_to_unsigned, u8::MAX);
    }
}
