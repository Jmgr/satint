use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Si, Su};

impl<T> Serialize for Si<T>
where
    T: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_inner().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Si<T>
where
    T: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Self::new)
    }
}

impl<T> Serialize for Su<T>
where
    T: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_inner().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Su<T>
where
    T: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Self::new)
    }
}
