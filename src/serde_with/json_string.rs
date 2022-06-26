//! De/serialize `T` as a JSON-encoded string, where `T: Serialize +
//! Deserialize`.
//!
//! ```rust
//! #[serde_as(as = "awaur::serde_with::JsonString<...>")]
//! ```
//! ```rust
//! #[serde(serialize_with = "awaur::serde_with::json_string::serialize")]
//! ```
//! ```rust
//! #[serde(deserialize_with = "awaur::serde_with::json_string::deserialize")]
//! ```
//! ```rust
//! #[serde(with = "awaur::serde_with::json_string")]
//! ```

pub use with::*;
#[doc(hidden)]
#[cfg(feature = "serde-as-wrapper")]
pub use wrapper::*;

mod with {
    use std::fmt;
    use std::marker::PhantomData;

    use serde::de::{DeserializeOwned, Deserializer, Error as DeserializeError, Visitor};
    use serde::ser::Error as SerializeError;
    use serde::{Serialize, Serializer};

    /// ```rust
    /// #[serde(serialize_with = "awaur::serde_with::json_string::serialize")]
    /// ```
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        serializer.serialize_str(&serde_json::to_string(value).map_err(SerializeError::custom)?)
    }

    /// ```rust
    /// #[serde(deserialize_with = "awaur::serde_with::json_string::deserialize")]
    /// ```
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned,
    {
        struct _Visitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for _Visitor<T>
        where
            T: DeserializeOwned,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a value that can be serialized as a JSON string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: DeserializeError,
            {
                serde_json::from_str(value).map_err(DeserializeError::custom)
            }
        }

        deserializer.deserialize_str(_Visitor(PhantomData))
    }
}

#[cfg(feature = "serde-as-wrapper")]
mod wrapper {
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_with::{DeserializeAs, SerializeAs};

    impl<T> Serialize for JsonString<T>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            super::with::serialize(&self.0, serializer)
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
            super::with::deserialize(deserializer).map(Self)
        }
    }

    /// Implements [`SerializeAs`][serde_with::SerializeAs] and
    /// [`DeserializeAs`][serde_with::DeserializeAs].
    pub struct JsonString<T>(T);

    impl<T, U> SerializeAs<T> for JsonString<U>
    where
        T: Serialize,
    {
        fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            super::with::serialize(source, serializer)
        }
    }

    impl<'de, T> DeserializeAs<'de, T> for JsonString<T>
    where
        T: DeserializeOwned,
    {
        fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
        {
            super::with::deserialize(deserializer)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use fake::faker::name::en::{FirstName, LastName};
    use fake::{Dummy, Fake};
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use time::OffsetDateTime;

    use super::JsonString;
    use crate::serde_with::Base62;

    #[serde_as]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Human {
        first: String,
        last: String,
        #[serde(with = "time::serde::rfc3339")]
        birth: OffsetDateTime,
    }

    impl<F> Dummy<F> for Human {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &F, rng: &mut R) -> Self {
            Human {
                first: FirstName().fake_with_rng(rng),
                last: LastName().fake_with_rng(rng),
                birth: OffsetDateTime::from_unix_timestamp(rng.gen_range(0..253402300799)).unwrap(),
            }
        }
    }

    #[test]
    fn test_roundtrip() {
        // let expect = fakes.iter().map(|v| serde_json::to_string(v).unwrap());
        let container = crate::macros::new_struct! {
            #[serde_as]
            #[derive(Serialize, Deserialize)]
            TestContainer {
                #[serde_as(as = "Vec<JsonString<Human>>")]
                pub values: Vec<Human> = fake::vec![Human; 50],
            }
        };
        let serialized = serde_json::to_string(&container).unwrap();
        let parsed = serde_json::from_str::<serde_json::Value>(&serialized).unwrap();
        let parsed = parsed
            .as_object()
            .unwrap()
            .get("values")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|v| serde_json::from_str::<Human>(v.as_str().unwrap()).unwrap());

        for (expect, actual) in std::iter::zip(container.values, parsed) {
            assert_eq!(&expect, &actual);
        }
    }

    #[test]
    fn test_wrapping() {
        let container = crate::macros::new_struct! {
            #[serde_as]
            #[derive(Serialize, Deserialize)]
            TestContainer {
                #[serde_as(as = "JsonString<Vec<Base62<u64>>>")]
                pub values: Vec<u64> = ((u64::MAX - 1000)..u64::MAX).chain([0]).collect(),
            }
        };
        let serialized = serde_json::to_string(&container).unwrap();
        let parsed = serde_json::from_str::<serde_json::Value>(&serialized).unwrap();
        let parsed = serde_json::from_str::<serde_json::Value>(
            parsed
                .as_object()
                .expect("expected an object map")
                .get("values")
                .expect("expected an object field")
                .as_str()
                .expect("expected a string"),
        )
        .expect("expected to parse string as json")
        .as_array()
        .expect("expected a vector of strings")
        .iter()
        .map(|v| base62::decode(v.as_str().unwrap()).unwrap() as u64);

        for (expect, actual) in std::iter::zip(container.values, parsed) {
            assert_eq!(&expect, &actual);
        }
    }
}
