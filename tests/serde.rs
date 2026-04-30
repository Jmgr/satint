#![cfg(feature = "serde")]

use satint::{Si, Si8, Si16, Si32, Si64, Si128, Su, Su8, Su16, Su32, Su64, Su128};

fn assert_serialize<T: serde::Serialize>() {}

fn assert_deserialize<T>()
where
    T: for<'de> serde::Deserialize<'de>,
{
}

#[test]
fn concrete_aliases_implement_serde_traits() {
    assert_serialize::<Si8>();
    assert_serialize::<Si16>();
    assert_serialize::<Si32>();
    assert_serialize::<Si64>();
    assert_serialize::<Si128>();
    assert_serialize::<Su8>();
    assert_serialize::<Su16>();
    assert_serialize::<Su32>();
    assert_serialize::<Su64>();
    assert_serialize::<Su128>();

    assert_deserialize::<Si8>();
    assert_deserialize::<Si16>();
    assert_deserialize::<Si32>();
    assert_deserialize::<Si64>();
    assert_deserialize::<Si128>();
    assert_deserialize::<Su8>();
    assert_deserialize::<Su16>();
    assert_deserialize::<Su32>();
    assert_deserialize::<Su64>();
    assert_deserialize::<Su128>();
}

#[test]
fn generic_wrappers_implement_serde_traits() {
    assert_serialize::<Si<i32>>();
    assert_serialize::<Su<u32>>();
    assert_deserialize::<Si<i32>>();
    assert_deserialize::<Su<u32>>();
}

#[test]
fn round_trips_as_inner_primitive() {
    use serde_test::{Token, assert_tokens};

    assert_tokens(&Si8::new(-12), &[Token::I8(-12)]);
    assert_tokens(&Si16::new(-1234), &[Token::I16(-1234)]);
    assert_tokens(&Si32::new(-123_456), &[Token::I32(-123_456)]);
    assert_tokens(&Si64::new(-123_456_789), &[Token::I64(-123_456_789)]);

    assert_tokens(&Su8::new(200), &[Token::U8(200)]);
    assert_tokens(&Su16::new(60_000), &[Token::U16(60_000)]);
    assert_tokens(&Su32::new(3_000_000_000), &[Token::U32(3_000_000_000)]);
    assert_tokens(&Su64::new(u64::MAX), &[Token::U64(u64::MAX)]);

    assert_tokens(&Si::<i32>::new(42), &[Token::I32(42)]);
    assert_tokens(&Su::<u32>::new(42), &[Token::U32(42)]);
}
