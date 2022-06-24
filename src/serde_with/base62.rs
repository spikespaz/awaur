use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, DeserializeFromStr, SerializeAs, SerializeDisplay};

#[derive(SerializeDisplay, DeserializeFromStr)]
pub struct Base62<T>(pub T);

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

#[cfg(test)]
pub mod tests {
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;

    use crate::serde_with::ext::Base62;

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct TestType<T>
    where
        T: Clone + TryFrom<u128> + Into<u128>,
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
        let values = range.clone().map(base62::encode);
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
        for (expect, actual) in std::iter::zip(values, parsed) {
            assert_eq!(&expect, &actual);
        }
    }
}
