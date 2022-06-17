//! This module is a re-export of the contents of the [`serde_with`] crate. The
//! module [`ext`] contains the adapter types that are defined by AWAUR.

mod base62;
mod json_string;

pub use serde_with::*;

pub mod ext {
    //! Extension module defined by AWAUR, exporting types that can be used with
    //! [`serde_with::serde_as`].

    #[cfg(feature = "serde-with-base62")]
    pub use super::base62::Base62;
    #[cfg(feature = "serde-with-json-string")]
    pub use super::json_string::JsonString;
}
