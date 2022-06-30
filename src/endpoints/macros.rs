use macro_pub::macro_pub;

#[doc(hidden)]
pub mod __endpoint_impl_imports {
    pub use std::option::Option::{None, Some};
    pub use std::result::Result::{Err, Ok};
    pub use std::vec::Vec;

    pub use {futures_lite, http, serde_json, serde_path_to_error, serde_qs};

    pub use crate::endpoints::errors::{DeserializeError, ResponseError};
    pub use crate::endpoints::macros::endpoint_impl;
    pub use crate::endpoints::response::ApiResponse;
}

/// This macro makes use of several calls to [`Result::unwrap`] or
/// [`Option::unwrap`]. The values that are unwrapped are expected to be of
/// types where the operation in question is guaranteed to be successful.
/// It may be the case that an unwrap fails at runtime; if the author making use
/// of the macro is certain that the hard-coded values are correct, but runtime
/// panics and unwinds, this is considered a bug. A panic means that a variant
/// needs to be added to the `Error` type, and that one of the following
/// justification comments is wrong.
#[macro_pub]
macro_rules! endpoint {
    (
        $client:ident $method:ident,
        uri: $base:ident / $path:literal,
        $(vars: [$($var:expr),+],)?
        $(params: $params:expr,)?
        $(body: $body:expr,)?
    ) => {
        $crate::endpoints::__endpoint_impl_imports::endpoint_impl!{
            $client $method,
            uri: $base / $path,
            $(vars: [$($var),*],)*
            $(params: $params,)*
            $(body: $body,)*
        }
    };
}

#[doc(hidden)]
#[macro_pub]
macro_rules! endpoint_impl {
    (
        $client:ident $method:ident,
        uri: $base:ident / $path:literal,
        $(vars: [$($var:expr),+],)?
        $(params: $params:expr,)?
        $(body: $body:expr,)?
    ) => {{
        use $crate::endpoints::__endpoint_impl_imports::*;
        use futures_lite::io::AsyncReadExt;

        #[allow(unused_mut)]
        let mut uri = endpoint_impl!(@uri, $base, $path $(, [$($var),*])?);
        // Use of unwrap:
        // The type of `$params` is expected to have been validated manually,
        // with a guarantee that it can be serialized as a query string with
        // [`serde_qs::to_string`]. This would only fail if runtime values fail
        // to serialize; this won't happen if the type of `$params` has a
        // well-defined structure.
        $(uri.set_query(Some(&serde_qs::to_string($params).unwrap()));)?

        let builder = http::Request::builder()
            .method(endpoint_impl!(@str $method))
            .uri(uri.as_str());
        // Use of unwrap:
        // Building the [`isahc::Request`] should realistically never fail,
        // because all of the involved values have already made it past every
        // preceeding point where the runtime had the opprotunity to panic.
        let request = endpoint_impl!(@build, builder $(, $body)?).unwrap();

        // Sending the request can easily fail, so this would get bubbled to
        // [`crate::Error::Request`].
        let response = $client.send_async(request).await?;
        let status = response.status();
        let mut bytes = Vec::new();

        // Use of unwrap:
        // Expect that reading the bytes from a response body is infallible.
        // Responses must always return some data, even an empty slice of bytes,
        // so unwrapping the result of the [`AsyncReadExt::read_to_end`] here
        // should be perfectly acceptable.
        response.into_body().read_to_end(&mut bytes).await.unwrap();

        // If the response status is not 200 OK, bubble the error, passing along
        // the unexpected status, the fully formed URI, and the body bytes in
        // case the server responded with more details.
        if status != 200 {
            return Err(ResponseError { uri, status, bytes }.into());
        }

        let deserializer = &mut serde_json::Deserializer::from_slice(bytes.as_slice());
        let result = serde_path_to_error::deserialize(deserializer);

        // Determine if the response's body bytes deserialized correctly into
        // the inferred type (outside the macro), and if not, bubble the error
        // to `Error::Deserialize`.
        match result {
            Ok(value) => Ok(ApiResponse::__new(bytes, value)),
            Err(error) => Err(DeserializeError { uri, error, bytes }.into()) ,
        }
    }};
    (@uri, $base:ident, $path:literal) => {
        // Use of unwrap:
        // This cannot fail as a result of a malformed `$base`, which is most
        // likely hard-coded, and at the very least, a parsing failure would
        // have already been caught. The `$path` is definitely hard-coded in
        // this branch with no variables. If this fails, the macro input was not
        // correct.
        $base.join($path).unwrap()
    };
    (@uri, $base:ident, $path:literal, [$($var:expr),+]) => {
        // Use of unwrap:
        // The call to [`url::Url::join`] takes a string that is produced by
        // `format!`, where parts of `$path` are replaced, in order, by `$var`
        // items with `ToString`. If it fails, the macro input was not correct.
        $base.join(&format!($path, $($var.to_string()),*)).unwrap()
    };
    (@build, $builder:ident) => {
        $builder.body(())
    };
    (@build, $builder:ident, $body:expr) => {
        // Use of unwrap:
        // The type of `$body` is expected to be validated manually. The user of
        // this macro should be confident that the type will serialize
        // successfully as a valid query string, even if the parameters of are
        // variadic at runtime.
        $builder.body(serde_json::to_string($body).unwrap())
    };
    (@str GET) => {
        "GET"
    };
    (@str POST) => {
        "POST"
    };
}
