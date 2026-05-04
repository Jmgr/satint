use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    common::Inner,
    si::{Si8, Si16, Si32, Si64, Si128},
    su::{Su8, Su16, Su32, Su64, Su128},
};

macro_rules! generate_serde {
    ($($name:ident),+ $(,)?) => {
        $(
            impl Serialize for $name {
                #[inline]
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    self.as_inner().serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for $name {
                #[inline]
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    type InnerTy = <$name as Inner>::Inner;
                    InnerTy::deserialize(deserializer).map(Self::new)
                }
            }
        )+
    };
}

generate_serde!(Si8, Si16, Si32, Si64, Si128, Su8, Su16, Su32, Su64, Su128);

#[cfg(test)]
mod tests {
    use ::serde::{Deserialize, Serialize};
    use serde_test::{Token, assert_tokens};

    use crate::{
        si::{Si8, Si16, Si32, Si64, Si128},
        su::{Su8, Su16, Su32, Su64, Su128},
    };

    fn assert_serialize<T: Serialize>() {}

    fn assert_deserialize<T>()
    where
        T: for<'de> Deserialize<'de>,
    {
    }

    macro_rules! test_serde_traits {
        ($($name:ident),+ $(,)?) => {
            #[test]
            fn generated_types_implement_serde_traits() {
                $(
                    assert_serialize::<$name>();
                    assert_deserialize::<$name>();
                )+
            }
        };
    }

    test_serde_traits!(Si8, Si16, Si32, Si64, Si128, Su8, Su16, Su32, Su64, Su128);

    macro_rules! test_serde_tokens {
        ($($mod_name:ident; $name:ident; $value:expr; $token:expr)+) => {
            $(
                mod $mod_name {
                    use super::*;

                    #[test]
                    fn round_trips_as_inner_primitive() {
                        assert_tokens(&$name::new($value), &[$token]);
                    }
                }
            )+
        };
    }

    test_serde_tokens!(
        si8_tests;  Si8;  -12;          Token::I8(-12)
        si16_tests; Si16; -1234;        Token::I16(-1234)
        si32_tests; Si32; -123_456;     Token::I32(-123_456)
        si64_tests; Si64; -123_456_789; Token::I64(-123_456_789)
        su8_tests;  Su8;  200;          Token::U8(200)
        su16_tests; Su16; 60_000;       Token::U16(60_000)
        su32_tests; Su32; 3_000_000_000; Token::U32(3_000_000_000)
        su64_tests; Su64; u64::MAX;     Token::U64(u64::MAX)
    );
}
