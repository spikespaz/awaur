use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, DeserializeFromStr, SerializeAs, SerializeDisplay};

#[derive(SerializeDisplay, DeserializeFromStr)]
pub struct Base62<T>(T);

impl<T> fmt::Display for Base62<T>
where
    T: Clone + Into<u128>,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&base62::encode(self.0.clone()))
    }
}

impl<T> FromStr for Base62<T>
where
    u128: TryInto<T>,
{
    type Err = base62::DecodeError;

    fn from_str(other: &str) -> Result<Self, Self::Err> {
        match base62::decode(other)?.try_into() {
            Ok(n) => Ok(Base62(n)),
            Err(_) => Err(Self::Err::ArithmeticOverflow),
        }
    }
}

impl<T> SerializeAs<T> for Base62<T>
where
    T: Clone + Into<u128>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Base62(source.clone()).serialize(serializer)
    }
}

impl<'de, T> DeserializeAs<'de, T> for Base62<T>
where
    u128: TryInto<T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        Base62::deserialize(deserializer).map(|this| this.0)
    }
}
