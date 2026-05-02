use rand::{
    Rng,
    distr::uniform::{Error, SampleBorrow, SampleUniform, UniformSampler},
};

use crate::{Si, Su};

/// Uniform sampler for signed saturating scalar values.
///
/// This type is used by the `rand` crate's [`SampleUniform`] implementation
/// for [`Si<T>`].
pub struct UniformSi<T>
where
    T: SampleUniform,
{
    inner: <T as SampleUniform>::Sampler,
}

impl<T> Clone for UniformSi<T>
where
    T: SampleUniform,
    <T as SampleUniform>::Sampler: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> core::fmt::Debug for UniformSi<T>
where
    T: SampleUniform,
    <T as SampleUniform>::Sampler: core::fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UniformSi")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T> UniformSampler for UniformSi<T>
where
    T: SampleUniform + Copy,
{
    type X = Si<T>;

    #[inline]
    fn new<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        <T as SampleUniform>::Sampler::new(low.borrow().into_inner(), high.borrow().into_inner())
            .map(|inner| Self { inner })
    }

    #[inline]
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        <T as SampleUniform>::Sampler::new_inclusive(
            low.borrow().into_inner(),
            high.borrow().into_inner(),
        )
        .map(|inner| Self { inner })
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Si::new(self.inner.sample(rng))
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
        <T as SampleUniform>::Sampler::sample_single(
            low.borrow().into_inner(),
            high.borrow().into_inner(),
            rng,
        )
        .map(Si::new)
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
        <T as SampleUniform>::Sampler::sample_single_inclusive(
            low.borrow().into_inner(),
            high.borrow().into_inner(),
            rng,
        )
        .map(Si::new)
    }
}

impl<T> SampleUniform for Si<T>
where
    T: SampleUniform + Copy,
{
    type Sampler = UniformSi<T>;
}

/// Uniform sampler for unsigned saturating scalar values.
///
/// This type is used by the `rand` crate's [`SampleUniform`] implementation
/// for [`Su<T>`].
pub struct UniformSu<T>
where
    T: SampleUniform,
{
    inner: <T as SampleUniform>::Sampler,
}

impl<T> Clone for UniformSu<T>
where
    T: SampleUniform,
    <T as SampleUniform>::Sampler: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> core::fmt::Debug for UniformSu<T>
where
    T: SampleUniform,
    <T as SampleUniform>::Sampler: core::fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UniformSu")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T> UniformSampler for UniformSu<T>
where
    T: SampleUniform + Copy,
{
    type X = Su<T>;

    #[inline]
    fn new<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        <T as SampleUniform>::Sampler::new(low.borrow().into_inner(), high.borrow().into_inner())
            .map(|inner| Self { inner })
    }

    #[inline]
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        <T as SampleUniform>::Sampler::new_inclusive(
            low.borrow().into_inner(),
            high.borrow().into_inner(),
        )
        .map(|inner| Self { inner })
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Su::new(self.inner.sample(rng))
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
        <T as SampleUniform>::Sampler::sample_single(
            low.borrow().into_inner(),
            high.borrow().into_inner(),
            rng,
        )
        .map(Su::new)
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
        <T as SampleUniform>::Sampler::sample_single_inclusive(
            low.borrow().into_inner(),
            high.borrow().into_inner(),
            rng,
        )
        .map(Su::new)
    }
}

impl<T> SampleUniform for Su<T>
where
    T: SampleUniform + Copy,
{
    type Sampler = UniformSu<T>;
}
