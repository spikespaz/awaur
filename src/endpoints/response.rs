use std::ops::{Deref, DerefMut};

/// Result of a successful API request returned from and endpoint function.
///
/// This type is expected to be the successful result of the expression
/// generated by the [`endpoint!`] macro.
///
/// [`endpoint!`]: crate::endpoints::endpoint
///
/// It implements [`Deref`] and [`DerefMut`] to provide easy access to the inner
/// deserialized value of type `T`. It also contains the original body bytes of
/// the response.
#[derive(Debug, Clone, PartialEq)]
pub struct ApiResponse<T> {
    bytes: Vec<u8>,
    value: T,
}

impl<T> ApiResponse<T> {
    #[doc(hidden)]
    pub fn __new(bytes: Vec<u8>, value: T) -> Self {
        Self { bytes, value }
    }

    /// Get an immutable borrow to the response's body bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get an immutable borrow to the value deserialized from bytes.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Take out the response's body bytes, discarding the deserialized data.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Take out the value deserialized from bytes, discarding those bytes.
    pub fn into_value(self) -> T {
        self.value
    }

    /// Take out both the bytes and the deserialized value as a tuple.
    pub fn into_bytes_value(self) -> (Vec<u8>, T) {
        (self.bytes, self.value)
    }
}

impl<T> Deref for ApiResponse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for ApiResponse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
