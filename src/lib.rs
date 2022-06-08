#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "paginator")]
pub mod paginator;
#[cfg(any(
    feature = "wrapper-backend-isahc",
    feature = "wrapper-backend-reqwest",
    feature = "wrapper-backend-surf",
    feature = "wrapper-backend-aiohttpc"
))]
pub mod wrapper;

/// Import this module to get all the types, traits, and constants defined by
/// the features you have enabled.
pub mod prelude {
    #[cfg(feature = "paginator")]
    #[doc(inline)]
    pub use super::paginator::*;
    #[doc(inline)]
    pub use super::wrapper::*;
}
