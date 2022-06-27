//! De/serialize `T` as a base-62-encoded string, where `T: Into<u128>, u128:
//! TryFrom<T>`.
//!
//! ```rust
//! #[serde_as(as = "awaur::serde_with::Base62<...>")]
//! ```
//! ```rust
//! #[serde(serialize_with = "awaur::serde_with::base62::serialize")]
//! ```
//! ```rust
//! #[serde(deserialize_with = "awaur::serde_with::base62::deserialize")]
//! ```
//! ```rust
//! #[serde(with = "awaur::serde_with::base62")]
//! ```

pub use with::*;
#[doc(hidden)]
#[cfg(feature = "serde-as-wrapper")]
pub use wrapper::*;

mod with {
    use std::fmt;
    use std::marker::PhantomData;

    use serde::de::{Error as DeserializeError, Unexpected, Visitor};
    use serde::{Deserializer, Serializer};

    /// ```rust
    /// #[serde(serialize_with = "awaur::serde_with::base62::serialize")]
    /// ```
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Clone + Into<u128>,
    {
        serializer.serialize_str(&base62::encode(value.clone()))
    }

    /// ```rust
    /// #[serde(deserialize_with = "awaur::serde_with::base62::deserialize")]
    /// ```
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        u128: TryInto<T>,
    {
        struct _Visitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for _Visitor<T>
        where
            u128: TryInto<T>,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a value that can be converted from a base-62 encoded u128")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: DeserializeError,
            {
                base62::decode(value)
                    .map_err(DeserializeError::custom)?
                    .try_into()
                    .map_err(|_| DeserializeError::invalid_value(Unexpected::Str(value), &self))
            }
        }

        deserializer.deserialize_str(_Visitor(PhantomData))
    }
}

#[cfg(feature = "serde-as-wrapper")]
mod wrapper {
    use std::marker::PhantomData;

    use serde::{Deserializer, Serializer};
    use serde_with::{DeserializeAs, SerializeAs};

    /// Implements [`SerializeAs`][serde_with::SerializeAs] and
    /// [`DeserializeAs`][serde_with::DeserializeAs].
    pub struct Base62<T>(PhantomData<T>);

    impl<T> SerializeAs<T> for Base62<T>
    where
        T: Clone + Into<u128>,
    {
        fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            super::with::serialize(source, serializer)
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
            super::with::deserialize(deserializer)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;

    use super::Base62;

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct TestType<T>
    where
        T: Clone + Into<u128>,
        u128: TryInto<T>,
    {
        #[serde_as(as = "Vec<Base62<T>>")]
        pub values: Vec<T>,
    }

    // Test both serializing and deserializing in one go
    #[test]
    fn test_roundtrip() {
        // Create a thousand large numbers, and add a zero for pedanticism
        let range = ((u128::MAX - 1000)..u128::MAX).chain([0]);
        // Encode those without the wrapper
        let expect = range.clone().map(base62::encode);
        // Create an instance of the wrapper type with the same numbers
        let container = TestType {
            values: range.collect(),
        };
        // Serialize the value as a JSON string
        let serialized = serde_json::to_string(&container).unwrap();
        // Parse the string back into a `Value` type
        let parsed = serde_json::from_str::<serde_json::Value>(&serialized).unwrap();
        // Keep unwrapping to get the vector of encoded strings
        let parsed = parsed
            .as_object()
            .unwrap()
            .get("values")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap());

        // Iterate over the manually encoded numbers and the ones encoded by the
        // wrapper's implementation of `SerializeAs` and `Serialize`
        for (expect, actual) in std::iter::zip(expect, parsed) {
            assert_eq!(&expect, &actual);
        }
    }
}
