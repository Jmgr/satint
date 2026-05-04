#![cfg(feature = "rand")]
#![allow(
    clippy::unwrap_used,
    reason = "Clippy's allow-unwrap-in-tests config does not apply to integration test crates"
)]

use rand::{
    RngExt, SeedableRng,
    distr::{
        Distribution, Uniform,
        uniform::{SampleRange, SampleUniform, UniformSampler},
    },
    rngs::SmallRng,
};
use satint::{
    Si8, Si16, Si32, Si64, Si128, Su8, Su16, Su32, Su64, Su128, si8, si16, si32, si64, si128, su8,
    su16, su32, su64, su128,
};

fn assert_sample_uniform<T: SampleUniform>() {}

fn assert_sample_range<T, R>()
where
    R: SampleRange<T>,
{
}

#[test]
fn concrete_aliases_implement_sample_uniform() {
    assert_sample_uniform::<Si8>();
    assert_sample_uniform::<Si16>();
    assert_sample_uniform::<Si32>();
    assert_sample_uniform::<Si64>();
    assert_sample_uniform::<Si128>();
    assert_sample_uniform::<Su8>();
    assert_sample_uniform::<Su16>();
    assert_sample_uniform::<Su32>();
    assert_sample_uniform::<Su64>();
    assert_sample_uniform::<Su128>();
}

#[test]
fn standard_ranges_implement_sample_range() {
    assert_sample_range::<Si8, core::ops::Range<Si8>>();
    assert_sample_range::<Si16, core::ops::RangeInclusive<Si16>>();
    assert_sample_range::<Su8, core::ops::Range<Su8>>();
    assert_sample_range::<Su16, core::ops::RangeInclusive<Su16>>();
}

#[test]
fn uniform_samples_signed_values() {
    let mut rng = SmallRng::seed_from_u64(1);
    let uniform = Uniform::new(si16(-10), si16(10)).unwrap();

    for _ in 0..128 {
        let value = uniform.sample(&mut rng);
        assert!(value >= si16(-10));
        assert!(value < si16(10));
    }
}

#[test]
fn uniform_samples_unsigned_values() {
    let mut rng = SmallRng::seed_from_u64(2);
    let uniform = Uniform::new(su16(10), su16(20)).unwrap();

    for _ in 0..128 {
        let value = uniform.sample(&mut rng);
        assert!(value >= su16(10));
        assert!(value < su16(20));
    }
}

#[test]
fn inclusive_uniform_samples_signed_values() {
    let mut rng = SmallRng::seed_from_u64(3);
    let uniform = Uniform::new_inclusive(si8(-3), si8(3)).unwrap();

    for _ in 0..128 {
        let value = uniform.sample(&mut rng);
        assert!(value >= si8(-3));
        assert!(value <= si8(3));
    }
}

#[test]
fn inclusive_uniform_samples_unsigned_values() {
    let mut rng = SmallRng::seed_from_u64(4);
    let uniform = Uniform::new_inclusive(su8(3), su8(9)).unwrap();

    for _ in 0..128 {
        let value = uniform.sample(&mut rng);
        assert!(value >= su8(3));
        assert!(value <= su8(9));
    }
}

#[test]
fn random_range_accepts_standard_ranges() {
    let mut rng = SmallRng::seed_from_u64(5);

    assert_eq!(rng.random_range(si32(-7)..si32(-6)), si32(-7));
    assert_eq!(rng.random_range(si64(-7)..=si64(-7)), si64(-7));
    assert_eq!(rng.random_range(su32(42)..su32(43)), su32(42));
    assert_eq!(rng.random_range(su64(42)..=su64(42)), su64(42));
}

#[test]
fn sample_single_delegates_to_inner_sampler() {
    let mut rng = SmallRng::seed_from_u64(6);

    assert_eq!(
        <Si128 as SampleUniform>::Sampler::sample_single(si128(-1), si128(0), &mut rng),
        Ok(si128(-1)),
    );
    assert_eq!(
        <Su128 as SampleUniform>::Sampler::sample_single_inclusive(su128(1), su128(1), &mut rng),
        Ok(su128(1)),
    );
}

#[test]
fn samplers_are_clone_and_debug() {
    let mut rng = SmallRng::seed_from_u64(7);

    let signed = <Si16 as SampleUniform>::Sampler::new(si16(-5), si16(5)).unwrap();
    let signed_clone = signed.clone();
    assert!(format!("{signed:?}").contains("UniformSi"));
    let signed_value = signed_clone.sample(&mut rng);
    assert!(signed_value >= si16(-5) && signed_value < si16(5));

    let unsigned = <Su16 as SampleUniform>::Sampler::new(su16(1), su16(5)).unwrap();
    let unsigned_clone = unsigned.clone();
    assert!(format!("{unsigned:?}").contains("UniformSu"));
    let unsigned_value = unsigned_clone.sample(&mut rng);
    assert!(unsigned_value >= su16(1) && unsigned_value < su16(5));
}

#[test]
fn invalid_ranges_return_rand_errors() {
    assert!(Uniform::new(si8(1), si8(1)).is_err());
    assert!(Uniform::new_inclusive(si8(1), si8(0)).is_err());
    assert!(Uniform::new(su8(1), su8(1)).is_err());
    assert!(Uniform::new_inclusive(su8(1), su8(0)).is_err());
}
