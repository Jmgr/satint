//! Signed saturating scalar aliases and constructors.

use core::{num::Saturating, ops::Neg};

use crate::{define_wrapper, scalars};

define_wrapper!(Si);

scalars! {
    Si, Si8, si8, i8;
    Si, Si16, si16, i16;
    Si, Si32, si32, i32;
    Si, Si64, si64, i64;
    Si, Si128, si128, i128;
}

impl<T> Neg for Si<T>
where
    Saturating<T>: Neg<Output = Saturating<T>>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}
