use core::num::TryFromIntError;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::si::Si;
use crate::su::Su;

/// Lossy conversion that clamps the value to the destination range.
///
/// Out-of-range positive values saturate to the destination's `MAX`, and
/// out-of-range negative values saturate to its `MIN` (or `0` for unsigned
/// targets). Use `From`/`Into` for lossless conversions and `TryFrom` when
/// you need to reject out-of-range values explicitly.
pub trait SaturatingFrom<T>: Sized {
    /// Converts `value` into `Self`, clamping to the destination range when
    /// the source value cannot be represented exactly.
    fn saturating_from(value: T) -> Self;
}

/// Reciprocal of [`SaturatingFrom`]. Provided automatically via a blanket impl.
pub trait SaturatingInto<U>: Sized {
    /// Converts `self` into `U`, clamping to the destination range when the
    /// source value cannot be represented exactly.
    fn saturating_into(self) -> U;
}

impl<T, U> SaturatingInto<U> for T
where
    U: SaturatingFrom<T>,
{
    #[inline]
    fn saturating_into(self) -> U {
        U::saturating_from(self)
    }
}

impl<T> SaturatingFrom<T> for T {
    #[inline]
    fn saturating_from(value: T) -> Self {
        value
    }
}

// -- Primitive → wrapper conversions --------------------------------------

macro_rules! impl_primitive_unsigned_to_su {
    ($($src:ty => $dst:ty),+ $(,)?) => {$(
        impl SaturatingFrom<$src> for Su<$dst> {
            #[inline]
            fn saturating_from(value: $src) -> Self {
                if (value as u128) > (<$dst>::MAX as u128) {
                    Self::new(<$dst>::MAX)
                } else {
                    Self::new(value as $dst)
                }
            }
        }
    )+};
}

macro_rules! impl_primitive_signed_to_su {
    ($($src:ty => $dst:ty),+ $(,)?) => {$(
        impl SaturatingFrom<$src> for Su<$dst> {
            #[inline]
            fn saturating_from(value: $src) -> Self {
                if value <= 0 {
                    Self::new(0)
                } else if (value as u128) > (<$dst>::MAX as u128) {
                    Self::new(<$dst>::MAX)
                } else {
                    Self::new(value as $dst)
                }
            }
        }
    )+};
}

macro_rules! impl_primitive_signed_to_si {
    ($($src:ty => $dst:ty),+ $(,)?) => {$(
        impl SaturatingFrom<$src> for Si<$dst> {
            #[inline]
            fn saturating_from(value: $src) -> Self {
                let v = value as i128;
                if v > (<$dst>::MAX as i128) {
                    Self::new(<$dst>::MAX)
                } else if v < (<$dst>::MIN as i128) {
                    Self::new(<$dst>::MIN)
                } else {
                    Self::new(value as $dst)
                }
            }
        }
    )+};
}

macro_rules! impl_primitive_unsigned_to_si {
    ($($src:ty => $dst:ty),+ $(,)?) => {$(
        impl SaturatingFrom<$src> for Si<$dst> {
            #[inline]
            fn saturating_from(value: $src) -> Self {
                if (value as u128) > (<$dst>::MAX as u128) {
                    Self::new(<$dst>::MAX)
                } else {
                    Self::new(value as $dst)
                }
            }
        }
    )+};
}

macro_rules! impl_primitive_to_su {
    ($dst:ty) => {
        impl_primitive_unsigned_to_su! {
            u8 => $dst, u16 => $dst, u32 => $dst, u64 => $dst, u128 => $dst, usize => $dst,
        }

        impl_primitive_signed_to_su! {
            i8 => $dst, i16 => $dst, i32 => $dst, i64 => $dst, i128 => $dst, isize => $dst,
        }
    };
}

macro_rules! impl_primitive_to_si {
    ($dst:ty) => {
        impl_primitive_signed_to_si! {
            i8 => $dst, i16 => $dst, i32 => $dst, i64 => $dst, i128 => $dst, isize => $dst,
        }

        impl_primitive_unsigned_to_si! {
            u8 => $dst, u16 => $dst, u32 => $dst, u64 => $dst, u128 => $dst, usize => $dst,
        }
    };
}

impl_primitive_to_su!(u8);
impl_primitive_to_su!(u16);
impl_primitive_to_su!(u32);
impl_primitive_to_su!(u64);
impl_primitive_to_su!(u128);

impl_primitive_to_si!(i8);
impl_primitive_to_si!(i16);
impl_primitive_to_si!(i32);
impl_primitive_to_si!(i64);
impl_primitive_to_si!(i128);

// -- Widening arithmetic --------------------------------------------------
//
// `Su<wider> + Su<narrower>` and `Si<wider> + Si<narrower>` (and Sub) — only
// emitted when the RHS provably fits in the LHS, so no clamping is needed.

macro_rules! impl_widening_ops {
    ($wrap:ident; $lhs:ty => $($rhs:ty),+ $(,)?) => {
        $(
            impl Add<$wrap<$rhs>> for $wrap<$lhs> {
                type Output = Self;

                #[inline]
                fn add(self, rhs: $wrap<$rhs>) -> Self::Output {
                    self + rhs.into_inner() as $lhs
                }
            }

            impl AddAssign<$wrap<$rhs>> for $wrap<$lhs> {
                #[inline]
                fn add_assign(&mut self, rhs: $wrap<$rhs>) {
                    *self += rhs.into_inner() as $lhs;
                }
            }

            impl Sub<$wrap<$rhs>> for $wrap<$lhs> {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: $wrap<$rhs>) -> Self::Output {
                    self - rhs.into_inner() as $lhs
                }
            }

            impl SubAssign<$wrap<$rhs>> for $wrap<$lhs> {
                #[inline]
                fn sub_assign(&mut self, rhs: $wrap<$rhs>) {
                    *self -= rhs.into_inner() as $lhs;
                }
            }
        )+
    };
}

impl_widening_ops!(Su; u16 => u8);
impl_widening_ops!(Su; u32 => u8, u16);
impl_widening_ops!(Su; u64 => u8, u16, u32);
impl_widening_ops!(Su; u128 => u8, u16, u32, u64);

impl_widening_ops!(Si; i16 => i8);
impl_widening_ops!(Si; i32 => i8, i16);
impl_widening_ops!(Si; i64 => i8, i16, i32);
impl_widening_ops!(Si; i128 => i8, i16, i32, i64);

// -- Lossless inter-wrapper widening: same-sign and unsigned → wider signed --

macro_rules! impl_same_sign_widening_from {
    ($wrap:ident; $dst:ty => $($src:ty),+ $(,)?) => {
        $(
            impl From<$wrap<$src>> for $wrap<$dst> {
                #[inline]
                fn from(value: $wrap<$src>) -> Self {
                    Self::new(value.into_inner() as $dst)
                }
            }
        )+
    };
}

impl_same_sign_widening_from!(Su; u16 => u8);
impl_same_sign_widening_from!(Su; u32 => u8, u16);
impl_same_sign_widening_from!(Su; u64 => u8, u16, u32);
impl_same_sign_widening_from!(Su; u128 => u8, u16, u32, u64);

impl_same_sign_widening_from!(Si; i16 => i8);
impl_same_sign_widening_from!(Si; i32 => i8, i16);
impl_same_sign_widening_from!(Si; i64 => i8, i16, i32);
impl_same_sign_widening_from!(Si; i128 => i8, i16, i32, i64);

macro_rules! impl_unsigned_to_signed_from {
    ($signed:ty => $($unsigned:ty),+ $(,)?) => {
        $(
            impl From<Su<$unsigned>> for Si<$signed> {
                #[inline]
                fn from(value: Su<$unsigned>) -> Self {
                    Self::new(value.into_inner() as $signed)
                }
            }
        )+
    };
}

impl_unsigned_to_signed_from!(i16 => u8);
impl_unsigned_to_signed_from!(i32 => u8, u16);
impl_unsigned_to_signed_from!(i64 => u8, u16, u32);
impl_unsigned_to_signed_from!(i128 => u8, u16, u32, u64);

// -- Fallible / saturating conversions ------------------------------------
//
// Every pair below gets two impls:
//   - `SaturatingFrom`: clamps to the destination's representable range.
//   - `TryFrom`: returns `Err(TryFromIntError)` on out-of-range input
//     (forwards to the primitive's stdlib `TryFrom`).

macro_rules! impl_su_su_narrowing {
    ($($src:ty => $dst:ty),+ $(,)?) => {$(
        impl SaturatingFrom<Su<$src>> for Su<$dst> {
            #[inline]
            fn saturating_from(value: Su<$src>) -> Self {
                let v = value.into_inner();
                if v > <$dst>::MAX as $src {
                    Self::new(<$dst>::MAX)
                } else {
                    Self::new(v as $dst)
                }
            }
        }

        impl TryFrom<Su<$src>> for Su<$dst> {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(value: Su<$src>) -> Result<Self, Self::Error> {
                <$dst>::try_from(value.into_inner()).map(Self::new)
            }
        }
    )+};
}

impl_su_su_narrowing! {
    u16 => u8,
    u32 => u8, u32 => u16,
    u64 => u8, u64 => u16, u64 => u32,
    u128 => u8, u128 => u16, u128 => u32, u128 => u64,
}

macro_rules! impl_si_si_narrowing {
    ($($src:ty => $dst:ty),+ $(,)?) => {$(
        impl SaturatingFrom<Si<$src>> for Si<$dst> {
            #[inline]
            fn saturating_from(value: Si<$src>) -> Self {
                let v = value.into_inner();
                if v > <$dst>::MAX as $src {
                    Self::new(<$dst>::MAX)
                } else if v < <$dst>::MIN as $src {
                    Self::new(<$dst>::MIN)
                } else {
                    Self::new(v as $dst)
                }
            }
        }

        impl TryFrom<Si<$src>> for Si<$dst> {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(value: Si<$src>) -> Result<Self, Self::Error> {
                <$dst>::try_from(value.into_inner()).map(Self::new)
            }
        }
    )+};
}

impl_si_si_narrowing! {
    i16 => i8,
    i32 => i8, i32 => i16,
    i64 => i8, i64 => i16, i64 => i32,
    i128 => i8, i128 => i16, i128 => i32, i128 => i64,
}

macro_rules! impl_si_to_su {
    ($($signed:ty => $unsigned:ty),+ $(,)?) => {$(
        impl SaturatingFrom<Si<$signed>> for Su<$unsigned> {
            #[inline]
            fn saturating_from(value: Si<$signed>) -> Self {
                let v = value.into_inner();
                if v <= 0 {
                    Self::new(0)
                } else if (v as u128) > (<$unsigned>::MAX as u128) {
                    Self::new(<$unsigned>::MAX)
                } else {
                    Self::new(v as $unsigned)
                }
            }
        }

        impl TryFrom<Si<$signed>> for Su<$unsigned> {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(value: Si<$signed>) -> Result<Self, Self::Error> {
                <$unsigned>::try_from(value.into_inner()).map(Self::new)
            }
        }
    )+};
}

impl_si_to_su! {
    i8 => u8, i8 => u16, i8 => u32, i8 => u64, i8 => u128,
    i16 => u8, i16 => u16, i16 => u32, i16 => u64, i16 => u128,
    i32 => u8, i32 => u16, i32 => u32, i32 => u64, i32 => u128,
    i64 => u8, i64 => u16, i64 => u32, i64 => u64, i64 => u128,
    i128 => u8, i128 => u16, i128 => u32, i128 => u64, i128 => u128,
}

// Su → Si: only the cases where overflow is possible. The strictly-narrower
// pairs (e.g. `Su<u8>` → `Si<i32>`) are lossless and use `From` above.
macro_rules! impl_su_to_si_fallible {
    ($($unsigned:ty => $signed:ty),+ $(,)?) => {$(
        impl SaturatingFrom<Su<$unsigned>> for Si<$signed> {
            #[inline]
            fn saturating_from(value: Su<$unsigned>) -> Self {
                let v = value.into_inner();
                if (v as u128) > (<$signed>::MAX as u128) {
                    Self::new(<$signed>::MAX)
                } else {
                    Self::new(v as $signed)
                }
            }
        }

        impl TryFrom<Su<$unsigned>> for Si<$signed> {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(value: Su<$unsigned>) -> Result<Self, Self::Error> {
                <$signed>::try_from(value.into_inner()).map(Self::new)
            }
        }
    )+};
}

impl_su_to_si_fallible! {
    u8 => i8,
    u16 => i8, u16 => i16,
    u32 => i8, u32 => i16, u32 => i32,
    u64 => i8, u64 => i16, u64 => i32, u64 => i64,
    u128 => i8, u128 => i16, u128 => i32, u128 => i64, u128 => i128,
}
