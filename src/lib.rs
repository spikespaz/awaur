#[cfg(feature = "paginator")]
pub mod paginator;

pub mod prelude {
    #[cfg(feature = "paginator")]
    #[doc(inline)]
    pub use paginator::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
