use std::marker::PhantomData;

use serde::de::{DeserializeOwned, Error as DeserializeError, Visitor};
use serde::ser::Error as SerializeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::ser::SerializeAsWrap;
use serde_with::{DeserializeAs, SerializeAs};

pub struct JsonString<T>(T);

impl<T> Serialize for JsonString<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&serde_json::to_string(self).map_err(SerializeError::custom)?)
    }
}

impl<'de, T> Deserialize<'de> for JsonString<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JsonStringVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for JsonStringVisitor<T>
        where
            T: DeserializeOwned,
        {
            type Value = JsonString<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a value that can be serialized as a JSON string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: DeserializeError,
            {
                Ok(JsonString(
                    serde_json::from_str(value).map_err(DeserializeError::custom)?,
                ))
            }
        }

        deserializer.deserialize_str(JsonStringVisitor(PhantomData))
    }
}

impl<T, U> SerializeAs<T> for JsonString<U>
where
    U: SerializeAs<T>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializeAsWrap::<T, U>::new(source).serialize(serializer)
    }
}

impl<'de, T, U> DeserializeAs<'de, T> for JsonString<U>
where
    T: DeserializeOwned,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        JsonString::deserialize(deserializer).map(|this| this.0)
    }
}
