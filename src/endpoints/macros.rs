use macro_pub::macro_pub;

/// Utility macro to help construct the bodies of functions that interface with
/// REST API endpoints.
///
/// # Overview
///
/// > *Please familiarize yourself with asynchrony in Rust before continuing.
/// > Assume that any return expression mentioned resolves to a future (which
/// > wraps the stated type), and therefore every return value must be polled
/// > either manually or with an executor.*
///
/// This macro handles the implementation of a function that constructs an
/// [`http::Request`]. The outer function signature must still be defined by
/// you; the resulting expression that an invocation expands to is intended to
/// be used as the body of your function. **To reiterate: invoking this macro
/// does not define a function, but the logic of an expression that generates
/// the return value of a signature that you create.**
///
/// The one exposed matching rule povides a simple syntax for defining your
/// inputs, such as the request body, query parameters and URI path components.
/// The syntax that this macro expects can be seen in the code block at the
/// beginning of this page. If you are unfamiliar with macros in Rust, please
/// refer to the ["Matching" section of *The Little Book of Rust Macros*].
///
/// > *Do not expect this macro's signature, output, or implementation to be
/// > stable until the first major release of this crate. After version 1.0.0,
/// > every minor increment is purported to be backwards and forwards-compatible
/// > until the next major release.*
///
/// # Function Signature
///
/// Typically your containing function will take in (at a minimum) a reference
/// to an instance of [`isahc::HttpClient`] and one to [`url::Url`]. Usually
/// these will be passed to the macro directly. You may also accept values for
/// `$params`, `$vars`, and `$body`, in any form of your choosing.
///
/// The expansion is an expression that resolves to a [`Result`], the generics
/// of which will conform to the types elided by your function signature. The
/// `Ok` variant will always be an [`ApiResponse`], whereas the `Err` variant
/// may contain any type that implements `From<DeserializeError>` and
/// `From<ResponseError>`. You may want to use the [`thiserror`] crate to wrap
/// [`DeserializeError`] and [`ResponseError`] into your own
/// [`std::error::Error`] type's variants. Conversion to your error type is
/// delegated by [`std::ops::Try`]'s interaction with [`From`].
///
/// [`thiserror`]: https://docs.rs/thiserror/latest/thiserror/
/// ["Matching" section of *The Little Book of Rust Macros*]: https://veykril.github.io/tlborm/decl-macros/macros-methodical.html#matching
///
/// **For examples of the intended usage, see the endpoint definitions for the
/// [`curseforge`] and [`modrinth`] crates.**
///
/// [`ApiResponse`]: crate::endpoints::ApiResponse
/// [`DeserializeError`]: crate::endpoints::DeserializeError
/// [`ResponseError`]: crate::endpoints::ResponseError
/// [`curseforge`]: https://docs.rs/curseforge/latest/src/curseforge/official/endpoints.rs.html
/// [`modrinth`]: https://docs.rs/modrinth/latest/src/modrinth/endpoints.rs.html
///
/// ## Input Tokens
///
/// #### `$client:ident`
///
/// Expected to be an identifier for an instance of [`isahc::HttpClient`], or
/// mimic the public API of the type at the very least. If you use a wrapper to
/// make another HTTP client compatible, be sure to review the source of this
/// macro to see which methods are used.
///
/// [`isahc::HttpClient`]: https://docs.rs/isahc/latest/isahc/struct.HttpClient.html
///
/// #### `$method:ident`
///
/// This is expecting an identifier item, but it will be converted to a string
/// and passed to [`http::request::Builder::method`]. **Currently only two
/// request methods are supported: `GET` and `POST`.** In the future this will
/// be expanded to support the full capabilities of the REST messaging paradigm.
///
/// #### `$base:ident`
///
/// Expected to be a reference to a [`url::Url`]. This value will be cloned and
/// mutated to add the URI path and query parameters. Results from mutable
/// operations here will be unwrapped; please make sure that
/// [`url::Url::cannot_be_a_base`] returns `false` at the very minimum. Do not
/// pass in values generated at runtime without validating them first.
///
/// #### `$path:literal`
///
/// Expected to be a string literal. If there are variadic components,
/// intersperse this literal with substitution placeholders (pairs of curly
/// braces, `{}`) in the style of [`format_args!`] (or [`println!`]). This gets
/// added to the end of the `$base`, and completes the URI that the request will
/// be made to.
///
/// #### `$($var:expr),+`
///
/// Expected to be a repeating pattern of valid expressions in the style of an
/// array (comma-delimited). The number of items inside the enclosing brackets
/// (`[]`) is expected to match the number of substitution placeholders (`{}`)
/// in the `$path` literal. Each expression's evaluation type must implement
/// [`ToString`] or have the `to_string` method. These will be formatted into
/// the `$path` string literal using [`format!`].
///
/// #### `$params:expr`
///
/// Expected to be an expression that resolves to a type implementing
/// [`serde::Serialize`], and compatible with [`serde_qs::to_string`]. The
/// result of that call will be unwrapped, you are responsible for validating
/// the serialization behavior.
///
/// #### `$body:expr`
///
/// Expected to be an expression that resolves to a type implementing
/// [`serde::Serialize`]. It must be compatible with [`serde_json::to_string`].
/// Just like `$params`, the result of serializing to a string will be
/// unwrapped. Validation is the responsibility of the caller.
///
/// # Disclaimer
///
/// This macro contains several calls to [`Option::unwrap`] and
/// [`Result::unwrap`] inside expressions where a value is always expected.
/// Unlike the memory safety afforded by the compiler's borrow checker, these
/// instances are not logically proven to be infallible operations; it is your
/// responsibility to ensure that the body, query parameters, and URI paths all
/// serialize correctly. There are comments in the source code that attempt to
/// justify these calls---it is highly recommended to view the source and read
/// these comments so that you can judge if your input is sufficiently robust.
///
/// It is especially recommended (as with any other nontrivial logic) to write
/// unit tests for every endpoint method. Please take care and validate the
/// serialization behavior of your input types, as incorrect implementations of
/// [`serde::Serialize`] that are likely fallible **will cause your software to
/// crash**.
///
/// **If this function body panics on a call to `unwrap`, double-check that your
/// inputs serialize properly. If you are fairly certain that something should
/// be valid (double-check with a tool like *Postman* or *[Hoppscotch]*) please
/// file a bug report on the [GitHub repository]. This may mean that an
/// assumption was incorrect and another error type is necessary.**
///
/// [Hoppscotch]: https://hoppscotch.io
/// [GitHub repository]: https://github.com/spikespaz/awaur
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
pub mod __endpoint_impl_imports {
    pub use std::option::Option::{None, Some};
    pub use std::result::Result::{Err, Ok};
    pub use std::vec::Vec;

    pub use {futures_lite, http, serde_json, serde_path_to_error, serde_qs};

    pub use crate::endpoint_impl;
    pub use crate::endpoints::errors::{DeserializeError, ResponseError};
    pub use crate::endpoints::response::ApiResponse;
}

#[doc(hidden)]
#[macro_export]
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
