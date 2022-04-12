#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "paginator")]
pub mod paginator;

/// Import this module to get all the types, traits, and constants defined by
/// the features you have enabled.
pub mod prelude {
    #[cfg(feature = "paginator")]
    #[doc(inline)]
    pub use super::paginator::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
