//! Provides types and functions that can be used with either `#[serde(with =
//! "...")]` or `#[serde_as(as = "...")]`.

#[cfg(feature = "serde-with-base62")]
pub mod base62;
#[cfg(feature = "serde-with-json-string")]
pub mod json_string;

#[cfg(all(feature = "serde-with-base62", feature = "serde-as-wrapper"))]
pub use self::base62::Base62;
#[cfg(all(feature = "serde-with-json-string", feature = "serde-as-wrapper"))]
pub use self::json_string::JsonString;
