#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
// This is for `macro_pub` to add documentation on <https://docs.rs>.
#![cfg_attr(doc, feature(decl_macro, rustc_attrs))]

#[cfg(feature = "endpoints")]
pub mod endpoints;
pub mod macros;
#[cfg(feature = "paginator")]
pub mod paginator;
#[cfg(any(feature = "serde-with-base62", feature = "serde-with-json-string"))]
pub mod serde_with;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
