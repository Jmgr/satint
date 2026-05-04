use crate::{
    common::{Inner, SaturatingFrom},
    si::{Si8, Si16, Si32, Si64, Si128},
    su::{Su8, Su16, Su32, Su64, Su128},
};

macro_rules! generate_wrapper_from_wrapper {
    ($source:ident; $($destination:ident),+ $(,)?) => {
        $(
            impl From<$source> for $destination {
                #[inline]
                fn from(value: $source) -> Self {
                    type DestinationTy = <$destination as Inner>::Inner;

                    Self::new(value.into_inner() as DestinationTy)
                }
            }
        )+
    };
}

generate_wrapper_from_wrapper!(Su8; Si16, Si32, Si64, Si128);
generate_wrapper_from_wrapper!(Su16; Si32, Si64, Si128);
generate_wrapper_from_wrapper!(Su32; Si64, Si128);
generate_wrapper_from_wrapper!(Su64; Si128);

macro_rules! generate_saturating_unsigned_wrapper_from_signed_wrapper {
    ($source:ident; $($destination:ident),+ $(,)?) => {
        $(
            impl SaturatingFrom<$source> for $destination {
                #[inline]
                fn saturating_from(value: $source) -> Self {
                    type DestinationTy = <$destination as Inner>::Inner;
                    let value = value.into_inner();
                    if value <= 0 {
                        Self::new(0)
                    } else if (value as u128) > (DestinationTy::MAX as u128) {
                        Self::new(DestinationTy::MAX)
                    } else {
                        Self::new(value as DestinationTy)
                    }
                }
            }
        )+
    };
}

generate_saturating_unsigned_wrapper_from_signed_wrapper!(Si8; Su8, Su16, Su32, Su64, Su128);
generate_saturating_unsigned_wrapper_from_signed_wrapper!(Si16; Su8, Su16, Su32, Su64, Su128);
generate_saturating_unsigned_wrapper_from_signed_wrapper!(Si32; Su8, Su16, Su32, Su64, Su128);
generate_saturating_unsigned_wrapper_from_signed_wrapper!(Si64; Su8, Su16, Su32, Su64, Su128);
generate_saturating_unsigned_wrapper_from_signed_wrapper!(Si128; Su8, Su16, Su32, Su64, Su128);

macro_rules! generate_saturating_signed_wrapper_from_unsigned_wrapper {
    ($source:ident; $($destination:ident),+ $(,)?) => {
        $(
            impl SaturatingFrom<$source> for $destination {
                #[inline]
                fn saturating_from(value: $source) -> Self {
                    type DestinationTy = <$destination as Inner>::Inner;
                    let value = value.into_inner();
                    if (value as u128) > (DestinationTy::MAX as u128) {
                        Self::new(DestinationTy::MAX)
                    } else {
                        Self::new(value as DestinationTy)
                    }
                }
            }
        )+
    };
}

generate_saturating_signed_wrapper_from_unsigned_wrapper!(Su8; Si8, Si16, Si32, Si64, Si128);
generate_saturating_signed_wrapper_from_unsigned_wrapper!(Su16; Si8, Si16, Si32, Si64, Si128);
generate_saturating_signed_wrapper_from_unsigned_wrapper!(Su32; Si8, Si16, Si32, Si64, Si128);
generate_saturating_signed_wrapper_from_unsigned_wrapper!(Su64; Si8, Si16, Si32, Si64, Si128);
generate_saturating_signed_wrapper_from_unsigned_wrapper!(Su128; Si8, Si16, Si32, Si64, Si128);

#[cfg(test)]
mod tests {
    use crate::{
        common::{Inner, SaturatingFrom},
        si::{Si8, Si16, Si32, Si64, Si128},
        su::{Su8, Su16, Su32, Su64, Su128},
    };

    // Lossless From<Su*> for wider Si*.
    macro_rules! test_su_to_si_lossless {
        ($($name:ident; $source:ident; $dest:ident)+) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($dest::from($source::ZERO), $dest::ZERO);
                    assert_eq!($dest::from($source::ONE), $dest::ONE);
                    assert_eq!($dest::from($source::new(42)).into_inner(), 42);
                    // Destination is strictly wider than source, so source MAX fits.
                    let src_max = $source::MAX.into_inner() as u128;
                    assert_eq!($dest::from($source::MAX).into_inner() as u128, src_max);
                }
            )+
        };
    }

    test_su_to_si_lossless!(
        su8_into_si16;   Su8;  Si16
        su8_into_si32;   Su8;  Si32
        su8_into_si64;   Su8;  Si64
        su8_into_si128;  Su8;  Si128
        su16_into_si32;  Su16; Si32
        su16_into_si64;  Su16; Si64
        su16_into_si128; Su16; Si128
        su32_into_si64;  Su32; Si64
        su32_into_si128; Su32; Si128
        su64_into_si128; Su64; Si128
    );

    // SaturatingFrom<Si*> for Su*: negatives -> 0, > dest MAX -> dest MAX, else pass-through.
    macro_rules! test_si_to_su_saturating {
        ($($name:ident; $source:ident; $dest:ident)+) => {
            $(
                #[test]
                fn $name() {
                    type SrcInner = <$source as Inner>::Inner;
                    type DstInner = <$dest as Inner>::Inner;

                    // Negative saturates to 0.
                    assert_eq!($dest::saturating_from($source::new(-1)), $dest::ZERO);
                    assert_eq!($dest::saturating_from($source::MIN), $dest::ZERO);

                    // Zero passes through.
                    assert_eq!($dest::saturating_from($source::ZERO), $dest::ZERO);

                    // Small positive passes through.
                    assert_eq!($dest::saturating_from($source::new(42)), $dest::new(42));

                    // Source MAX: clamps when wider than dest, else pass-through.
                    let src_max = $source::MAX.into_inner() as u128;
                    let dst_max = DstInner::MAX as u128;
                    let expected = if src_max > dst_max {
                        $dest::MAX
                    } else {
                        $dest::new(SrcInner::MAX as DstInner)
                    };
                    assert_eq!($dest::saturating_from($source::MAX), expected);
                }
            )+
        };
    }

    test_si_to_su_saturating!(
        si8_into_su8;     Si8;   Su8
        si8_into_su16;    Si8;   Su16
        si8_into_su32;    Si8;   Su32
        si8_into_su64;    Si8;   Su64
        si8_into_su128;   Si8;   Su128
        si16_into_su8;    Si16;  Su8
        si16_into_su16;   Si16;  Su16
        si16_into_su32;   Si16;  Su32
        si16_into_su64;   Si16;  Su64
        si16_into_su128;  Si16;  Su128
        si32_into_su8;    Si32;  Su8
        si32_into_su16;   Si32;  Su16
        si32_into_su32;   Si32;  Su32
        si32_into_su64;   Si32;  Su64
        si32_into_su128;  Si32;  Su128
        si64_into_su8;    Si64;  Su8
        si64_into_su16;   Si64;  Su16
        si64_into_su32;   Si64;  Su32
        si64_into_su64;   Si64;  Su64
        si64_into_su128;  Si64;  Su128
        si128_into_su8;   Si128; Su8
        si128_into_su16;  Si128; Su16
        si128_into_su32;  Si128; Su32
        si128_into_su64;  Si128; Su64
        si128_into_su128; Si128; Su128
    );

    // SaturatingFrom<Su*> for Si*: > dest signed MAX -> dest MAX, else pass-through.
    macro_rules! test_su_to_si_saturating {
        ($($name:ident; $source:ident; $dest:ident)+) => {
            $(
                #[test]
                fn $name() {
                    type SrcInner = <$source as Inner>::Inner;
                    type DstInner = <$dest as Inner>::Inner;

                    // Zero passes through.
                    assert_eq!($dest::saturating_from($source::ZERO), $dest::ZERO);

                    // Small positive passes through.
                    assert_eq!($dest::saturating_from($source::new(42)), $dest::new(42));

                    // Source MAX: clamps when source range exceeds signed dest range.
                    let src_max = $source::MAX.into_inner() as u128;
                    let dst_max = DstInner::MAX as u128;
                    let expected = if src_max > dst_max {
                        $dest::MAX
                    } else {
                        $dest::new(SrcInner::MAX as DstInner)
                    };
                    assert_eq!($dest::saturating_from($source::MAX), expected);
                }
            )+
        };
    }

    test_su_to_si_saturating!(
        su8_into_si8;     Su8;   Si8
        su16_into_si8;    Su16;  Si8
        su16_into_si16;   Su16;  Si16
        su32_into_si8;    Su32;  Si8
        su32_into_si16;   Su32;  Si16
        su32_into_si32;   Su32;  Si32
        su64_into_si8;    Su64;  Si8
        su64_into_si16;   Su64;  Si16
        su64_into_si32;   Su64;  Si32
        su64_into_si64;   Su64;  Si64
        su128_into_si8;   Su128; Si8
        su128_into_si16;  Su128; Si16
        su128_into_si32;  Su128; Si32
        su128_into_si64;  Su128; Si64
        su128_into_si128; Su128; Si128
    );

    // usize/isize are guaranteed by the Rust reference to be at least 16 bits
    // and are <= 64 bits on every supported target, so the i128/u128 widening
    // used by every impl below is exact.
    // Lossless From<wrapper> for usize/isize.
    macro_rules! test_lossless_from {
            ($($name:ident; $source:ident; $primitive:ty)+) => {
                $(
                    #[test]
                    fn $name() {
                        type SrcInner = <$source as Inner>::Inner;
                        assert_eq!(<$primitive>::from($source::ZERO), 0);
                        assert_eq!(<$primitive>::from($source::MAX), SrcInner::MAX as $primitive);
                        assert_eq!(<$primitive>::from($source::MIN), SrcInner::MIN as $primitive);
                    }
                )+
            };
        }

    test_lossless_from!(
        si8_into_isize;  Si8;  isize
        si16_into_isize; Si16; isize
        su8_into_usize;  Su8;  usize
        su16_into_usize; Su16; usize
        su8_into_isize;  Su8;  isize
    );

    // SaturatingFrom<isize> for Si* (same-sign signed).
    macro_rules! test_saturating_isize_to_si {
            ($($name:ident; $dest:ident)+) => {
                $(
                    #[test]
                    fn $name() {
                        type DstInner = <$dest as Inner>::Inner;
                        assert_eq!($dest::saturating_from(0_isize), $dest::ZERO);
                        assert_eq!($dest::saturating_from(42_isize), $dest::new(42));
                        assert_eq!($dest::saturating_from(-42_isize), $dest::new(-42));
                        let expected_max = if (isize::MAX as i128) > (DstInner::MAX as i128) {
                            $dest::MAX
                        } else {
                            $dest::new(isize::MAX as DstInner)
                        };
                        assert_eq!($dest::saturating_from(isize::MAX), expected_max);
                        let expected_min = if (isize::MIN as i128) < (DstInner::MIN as i128) {
                            $dest::MIN
                        } else {
                            $dest::new(isize::MIN as DstInner)
                        };
                        assert_eq!($dest::saturating_from(isize::MIN), expected_min);
                    }
                )+
            };
        }

    test_saturating_isize_to_si!(
        isize_into_si8;   Si8
        isize_into_si16;  Si16
        isize_into_si32;  Si32
        isize_into_si64;  Si64
        isize_into_si128; Si128
    );

    // SaturatingFrom<usize> for Si* (cross-sign; source non-negative).
    macro_rules! test_saturating_usize_to_si {
            ($($name:ident; $dest:ident)+) => {
                $(
                    #[test]
                    fn $name() {
                        type DstInner = <$dest as Inner>::Inner;
                        assert_eq!($dest::saturating_from(0_usize), $dest::ZERO);
                        assert_eq!($dest::saturating_from(42_usize), $dest::new(42));
                        let expected_max = if (usize::MAX as u128) > (DstInner::MAX as u128) {
                            $dest::MAX
                        } else {
                            $dest::new(usize::MAX as DstInner)
                        };
                        assert_eq!($dest::saturating_from(usize::MAX), expected_max);
                    }
                )+
            };
        }

    test_saturating_usize_to_si!(
        usize_into_si8;   Si8
        usize_into_si16;  Si16
        usize_into_si32;  Si32
        usize_into_si64;  Si64
        usize_into_si128; Si128
    );

    // SaturatingFrom<usize> for Su* (same-sign unsigned).
    macro_rules! test_saturating_usize_to_su {
            ($($name:ident; $dest:ident)+) => {
                $(
                    #[test]
                    fn $name() {
                        type DstInner = <$dest as Inner>::Inner;
                        assert_eq!($dest::saturating_from(0_usize), $dest::ZERO);
                        assert_eq!($dest::saturating_from(42_usize), $dest::new(42));
                        let expected_max = if (usize::MAX as u128) > (DstInner::MAX as u128) {
                            $dest::MAX
                        } else {
                            $dest::new(usize::MAX as DstInner)
                        };
                        assert_eq!($dest::saturating_from(usize::MAX), expected_max);
                    }
                )+
            };
        }

    test_saturating_usize_to_su!(
        usize_into_su8;   Su8
        usize_into_su16;  Su16
        usize_into_su32;  Su32
        usize_into_su64;  Su64
        usize_into_su128; Su128
    );

    // SaturatingFrom<isize> for Su* (cross-sign; negatives -> 0).
    macro_rules! test_saturating_isize_to_su {
            ($($name:ident; $dest:ident)+) => {
                $(
                    #[test]
                    fn $name() {
                        type DstInner = <$dest as Inner>::Inner;
                        assert_eq!($dest::saturating_from(0_isize), $dest::ZERO);
                        assert_eq!($dest::saturating_from(42_isize), $dest::new(42));
                        assert_eq!($dest::saturating_from(-1_isize), $dest::ZERO);
                        assert_eq!($dest::saturating_from(isize::MIN), $dest::ZERO);
                        let expected_max = if (isize::MAX as u128) > (DstInner::MAX as u128) {
                            $dest::MAX
                        } else {
                            $dest::new(isize::MAX as DstInner)
                        };
                        assert_eq!($dest::saturating_from(isize::MAX), expected_max);
                    }
                )+
            };
        }

    test_saturating_isize_to_su!(
        isize_into_su8;   Su8
        isize_into_su16;  Su16
        isize_into_su32;  Su32
        isize_into_su64;  Su64
        isize_into_su128; Su128
    );

    // SaturatingFrom<Si*> for isize.
    macro_rules! test_saturating_si_to_isize {
            ($($name:ident; $source:ident)+) => {
                $(
                    #[test]
                    fn $name() {
                        type SrcInner = <$source as Inner>::Inner;
                        assert_eq!(isize::saturating_from($source::ZERO), 0);
                        assert_eq!(isize::saturating_from($source::new(42)), 42);
                        assert_eq!(isize::saturating_from($source::new(-42)), -42);
                        let expected_max = if (SrcInner::MAX as i128) > (isize::MAX as i128) {
                            isize::MAX
                        } else {
                            SrcInner::MAX as isize
                        };
                        assert_eq!(isize::saturating_from($source::MAX), expected_max);
                        let expected_min = if (SrcInner::MIN as i128) < (isize::MIN as i128) {
                            isize::MIN
                        } else {
                            SrcInner::MIN as isize
                        };
                        assert_eq!(isize::saturating_from($source::MIN), expected_min);
                    }
                )+
            };
        }

    test_saturating_si_to_isize!(
        si32_into_isize;  Si32
        si64_into_isize;  Si64
        si128_into_isize; Si128
    );

    // SaturatingFrom<Su*> for usize.
    macro_rules! test_saturating_su_to_usize {
            ($($name:ident; $source:ident)+) => {
                $(
                    #[test]
                    fn $name() {
                        type SrcInner = <$source as Inner>::Inner;
                        assert_eq!(usize::saturating_from($source::ZERO), 0);
                        assert_eq!(usize::saturating_from($source::new(42)), 42);
                        let expected_max = if (SrcInner::MAX as u128) > (usize::MAX as u128) {
                            usize::MAX
                        } else {
                            SrcInner::MAX as usize
                        };
                        assert_eq!(usize::saturating_from($source::MAX), expected_max);
                    }
                )+
            };
        }

    test_saturating_su_to_usize!(
        su32_into_usize;  Su32
        su64_into_usize;  Su64
        su128_into_usize; Su128
    );

    #[test]
    fn round_trip_isize_through_si128() {
        for v in [0_isize, 1, -1, 12345, -12345, isize::MAX, isize::MIN] {
            assert_eq!(isize::saturating_from(Si128::saturating_from(v)), v);
        }
    }

    #[test]
    fn round_trip_usize_through_su128() {
        for v in [0_usize, 1, 12345, usize::MAX] {
            assert_eq!(usize::saturating_from(Su128::saturating_from(v)), v);
        }
    }
}
