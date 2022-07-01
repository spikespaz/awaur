/// Error type used if an API request recieved a successful response, but the
/// body bytes failed to deserialize into the expected strong-type. This
/// contains the original bytes that failed to deserialize, for debugging
/// purposes.
#[derive(Debug, thiserror::Error)]
#[error("failed to deserialize a response from:\n{uri}\n{inner}")]
pub struct DeserializeError {
    uri: url::Url,
    bytes: Vec<u8>,
    #[source]
    inner: serde_path_to_error::Error<serde_json::Error>,
}

/// A request to a URI that was expected to return successfully with 200
/// OK has failed to do so. This contains the status code that was received
/// instead, and the bytes in the body of the response.
#[derive(Debug, thiserror::Error)]
#[error("received unsuccessful status code {status} from:\n{uri}")]
pub struct ResponseError {
    uri: url::Url,
    bytes: Vec<u8>,
    status: http::StatusCode,
}

macro_rules! impl_field_accessors {
    ($implementor:ident) => {
        impl $implementor {
            /// Reference to the URI of the request.
            pub fn uri(&self) -> &url::Url {
                &self.uri
            }

            /// Reference to the body bytes of the response.
            pub fn bytes(&self) -> &[u8] {
                &self.bytes
            }

            /// Consume this error, taking out the URI of the request.
            pub fn into_uri(self) -> url::Url {
                self.uri
            }

            /// Consume this error, taking out the body bytes of the response.
            pub fn into_bytes(self) -> Vec<u8> {
                self.bytes
            }

            /// Consume this error, taking out both the URI of the request,
            /// and the body bytes of the response.
            pub fn into_uri_bytes(self) -> (url::Url, Vec<u8>) {
                (self.uri, self.bytes)
            }
        }
    };
}

impl_field_accessors!(DeserializeError);
impl_field_accessors!(ResponseError);

impl DeserializeError {
    #[doc(hidden)]
    pub fn __new(
        uri: url::Url,
        bytes: Vec<u8>,
        error: serde_path_to_error::Error<serde_json::Error>,
    ) -> Self {
        Self {
            uri,
            bytes,
            inner: error,
        }
    }

    /// Reference to the [`Path`] of the value that failed to deserialize.
    ///
    /// [`Path`]: serde_path_to_error::Path
    pub fn path(&self) -> &serde_path_to_error::Path {
        self.inner.path()
    }

    /// Reference to the original [`serde_json::Error`].
    pub fn inner(&self) -> &serde_json::Error {
        self.inner.inner()
    }

    /// Consume this error, taking out the original [`serde_json::Error`].
    pub fn into_inner(self) -> serde_json::Error {
        self.inner.into_inner()
    }
}

impl ResponseError {
    #[doc(hidden)]
    pub fn __new(uri: url::Url, bytes: Vec<u8>, status: http::StatusCode) -> Self {
        Self { uri, bytes, status }
    }

    /// Copy of the response's status code.
    pub fn status_code(&self) -> http::StatusCode {
        self.status
    }
}
