/// This is the most useful error type. This will be returned if the API
/// response failed to parse either as valid JSON, or according to the
/// policy for handling unknown fields set by the enabled Cargo features.
/// See the crate documentation for [conditional
/// compilation](crate#conditional-compilation).
#[derive(Debug, thiserror::Error)]
#[error("failed to deserialize a response from:\n{uri}\n{error}")]
pub struct DeserializeError {
    /// The URI that the initial request was sent to.
    pub uri: url::Url,
    /// The source error that this was constructed from.
    #[source]
    pub error: serde_path_to_error::Error<serde_json::Error>,
    /// The body content bytes of the response.
    pub bytes: Vec<u8>,
}

/// A request to a URI that was expected to return successfully with 200
/// OK has failed to do so. This contains the status code that was received
/// instead, and the bytes in the body of the response.
#[derive(Debug, thiserror::Error)]
#[error("received unsuccessful status code {status} from:\n{uri}")]
pub struct ResponseError {
    /// The URI that the initial request was sent to.
    pub uri: url::Url,
    /// The response status code that was received, not 200 OK.
    pub status: http::StatusCode,
    /// The body content bytes of the response.
    pub bytes: Vec<u8>,
}
