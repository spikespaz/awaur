//! Definitions for types used with the [`endpoint!`] macro. See the
//! documentation on that macro for information about how the members of this
//! module are intended to be used.
//!
//! [`endpoint!`]: crate::endpoints::endpoint

pub(crate) mod errors;
pub(crate) mod macros;
pub(crate) mod response;

pub use errors::*;
pub use macros::*;
pub use response::*;
