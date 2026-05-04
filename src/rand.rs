use rand::{
    Rng,
    distr::uniform::{Error, SampleBorrow, SampleUniform, UniformSampler},
};

use crate::{
    common::Inner,
    si::{Si8, Si16, Si32, Si64, Si128},
    su::{Su8, Su16, Su32, Su64, Su128},
};

macro_rules! generate_rand {
    ($($name:ident; $sampler:ident)+) => {
        $(
            #[doc = concat!("Uniform sampler for [`", stringify!($name), "`].")]
            #[derive(Debug, Clone)]
            pub struct $sampler {
                inner: <<$name as Inner>::Inner as SampleUniform>::Sampler,
            }

            impl UniformSampler for $sampler {
                type X = $name;

                #[inline]
                fn new<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    <<$name as Inner>::Inner as SampleUniform>::Sampler::new(
                        low.borrow().into_inner(),
                        high.borrow().into_inner(),
                    )
                    .map(|inner| Self { inner })
                }

                #[inline]
                fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    <<$name as Inner>::Inner as SampleUniform>::Sampler::new_inclusive(
                        low.borrow().into_inner(),
                        high.borrow().into_inner(),
                    )
                    .map(|inner| Self { inner })
                }

                #[inline]
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                    $name::new(self.inner.sample(rng))
                }

                #[inline]
                fn sample_single<R: Rng + ?Sized, B1, B2>(
                    low: B1,
                    high: B2,
                    rng: &mut R,
                ) -> Result<Self::X, Error>
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    <<$name as Inner>::Inner as SampleUniform>::Sampler::sample_single(
                        low.borrow().into_inner(),
                        high.borrow().into_inner(),
                        rng,
                    )
                    .map($name::new)
                }

                #[inline]
                fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
                    low: B1,
                    high: B2,
                    rng: &mut R,
                ) -> Result<Self::X, Error>
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    <<$name as Inner>::Inner as SampleUniform>::Sampler::sample_single_inclusive(
                        low.borrow().into_inner(),
                        high.borrow().into_inner(),
                        rng,
                    )
                    .map($name::new)
                }
            }

            impl SampleUniform for $name {
                type Sampler = $sampler;
            }
        )+
    };
}

generate_rand!(
    Si8; UniformSi8
    Si16; UniformSi16
    Si32; UniformSi32
    Si64; UniformSi64
    Si128; UniformSi128
    Su8; UniformSu8
    Su16; UniformSu16
    Su32; UniformSu32
    Su64; UniformSu64
    Su128; UniformSu128
);

#[cfg(test)]
mod tests {
    extern crate std;

    use core::ops::{Range, RangeInclusive};
    use std::format;

    use rand::{
        RngExt, SeedableRng,
        distr::{
            Distribution, Uniform,
            uniform::{SampleRange, SampleUniform, UniformSampler},
        },
        rngs::SmallRng,
    };

    use super::{
        UniformSi8, UniformSi16, UniformSi32, UniformSi64, UniformSi128, UniformSu8, UniformSu16,
        UniformSu32, UniformSu64, UniformSu128,
    };
    use crate::{
        si::{Si8, Si16, Si32, Si64, Si128},
        su::{Su8, Su16, Su32, Su64, Su128},
    };

    fn assert_sample_uniform<T: SampleUniform>() {}

    fn assert_sample_range<T, R>()
    where
        R: SampleRange<T>,
    {
    }

    macro_rules! test_rand_suite {
        ($($mod_name:ident; $name:ident; $sampler:ident; $low:expr; $one_above_low:expr; $high:expr)+) => {
            $(
                mod $mod_name {
                    use super::*;

                    #[test]
                    fn implements_sample_traits() {
                        assert_sample_uniform::<$name>();
                        assert_sample_range::<$name, Range<$name>>();
                        assert_sample_range::<$name, RangeInclusive<$name>>();
                    }

                    #[test]
                    fn samples_exclusive_range() {
                        let mut rng = SmallRng::seed_from_u64(1);
                        let uniform = Uniform::new($name::new($low), $name::new($high)).unwrap();

                        for _ in 0..32 {
                            let value = uniform.sample(&mut rng);
                            assert!(value >= $name::new($low));
                            assert!(value < $name::new($high));
                        }
                    }

                    #[test]
                    fn samples_inclusive_range() {
                        let mut rng = SmallRng::seed_from_u64(2);
                        let uniform =
                            Uniform::new_inclusive($name::new($low), $name::new($high)).unwrap();

                        for _ in 0..32 {
                            let value = uniform.sample(&mut rng);
                            assert!(value >= $name::new($low));
                            assert!(value <= $name::new($high));
                        }
                    }

                    #[test]
                    fn accepts_borrowed_bounds_and_derives_debug_clone() {
                        let low = $name::new($low);
                        let high = $name::new($high);
                        let sampler: $sampler =
                            <$name as SampleUniform>::Sampler::new(&low, &high).unwrap();
                        let clone = sampler.clone();

                        assert!(format!("{sampler:?}").contains(stringify!($sampler)));

                        let mut rng = SmallRng::seed_from_u64(3);
                        let value = clone.sample(&mut rng);
                        assert!(value >= low);
                        assert!(value < high);
                    }

                    #[test]
                    fn random_range_accepts_standard_ranges() {
                        let mut rng = SmallRng::seed_from_u64(4);

                        assert_eq!(
                            rng.random_range($name::new($low)..$name::new($one_above_low)),
                            $name::new($low),
                        );
                        assert_eq!(
                            rng.random_range($name::new($low)..=$name::new($low)),
                            $name::new($low),
                        );
                    }

                    #[test]
                    fn sample_single_delegates_to_inner_sampler() {
                        let mut rng = SmallRng::seed_from_u64(5);

                        assert_eq!(
                            <$sampler as UniformSampler>::sample_single(
                                $name::new($low),
                                $name::new($one_above_low),
                                &mut rng,
                            ),
                            Ok($name::new($low)),
                        );
                        assert_eq!(
                            <$sampler as UniformSampler>::sample_single_inclusive(
                                $name::new($high),
                                $name::new($high),
                                &mut rng,
                            ),
                            Ok($name::new($high)),
                        );
                    }

                    #[test]
                    fn invalid_ranges_return_errors() {
                        assert!(
                            <$sampler as UniformSampler>::new(
                                $name::new($low),
                                $name::new($low),
                            )
                            .is_err()
                        );
                        assert!(
                            <$sampler as UniformSampler>::new_inclusive(
                                $name::new($high),
                                $name::new($low),
                            )
                            .is_err()
                        );
                    }
                }
            )+
        };
    }

    test_rand_suite!(
        si8_tests;   Si8;   UniformSi8;   -5; -4; 5
        si16_tests;  Si16;  UniformSi16;  -5; -4; 5
        si32_tests;  Si32;  UniformSi32;  -5; -4; 5
        si64_tests;  Si64;  UniformSi64;  -5; -4; 5
        si128_tests; Si128; UniformSi128; -5; -4; 5
        su8_tests;   Su8;   UniformSu8;    5;  6; 15
        su16_tests;  Su16;  UniformSu16;   5;  6; 15
        su32_tests;  Su32;  UniformSu32;   5;  6; 15
        su64_tests;  Su64;  UniformSu64;   5;  6; 15
        su128_tests; Su128; UniformSu128;  5;  6; 15
    );
}
