//! See documentation for [`HttpStatus`].

use std::fmt::{Display, Formatter};

/// Enumeration representing HTTP response status codes defined by [RFC #7231],
/// [RFC #7232], [RFC #7535], and [RFC #6585].
///
/// [RFC #7231]: https://datatracker.ietf.org/doc/html/rfc7231
/// [RFC #7232]: https://datatracker.ietf.org/doc/html/rfc7232
/// [RFC #7235]: https://datatracker.ietf.org/doc/html/rfc7235
/// [RFC #6585]: https://datatracker.ietf.org/doc/html/rfc6585
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HttpStatus {
    //*
    // Informational (100-199)
    //*
    /// [Section 6.2.1 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.2.1)
    Continue,
    /// [Section 6.2.2 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.2.2)
    SwitchingProtocols,
    //*
    // Successful (200-299)
    //*
    /// [Section 6.3.1 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.3.1)
    Ok,
    /// [Section 6.3.2 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.3.2)
    Created,
    /// [Section 6.3.3 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.3.3)
    Accepted,
    /// [Section 6.3.4 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.3.4)
    NonAuthoritativeInformation,
    /// [Section 6.3.5 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.3.5)
    NoContent,
    /// [Section 6.3.6 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.3.6)
    ResetContent,
    /// [Section 4.1 of RFC #7233](https://datatracker.ietf.org/doc/html/rfc7233#section-4.1)
    PartialContent,
    //*
    // Redirection (300-399)
    //*
    /// [Section 6.4.1 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.4.1)
    MultipleChoices,
    /// [Section 6.4.2 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.4.2)
    MovedPermanently,
    /// [Section 6.4.3 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.4.3)
    Found,
    /// [Section 6.4.4 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.4.4)
    SeeOther,
    /// [Section 4.1 of RFC #7232](https://datatracker.ietf.org/doc/html/rfc7232#section-4.1)
    NotModified,
    /// [Section 6.4.5 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.4.5)
    UseProxy,
    /// [Section 6.4.7 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.4.7)
    TemporaryRedirect,
    //*
    // Client Error (400-499)
    //*
    /// [Section 6.5.1 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.1)
    BadRequest,
    /// [Section 3.1 of RFC #7235](https://datatracker.ietf.org/doc/html/rfc7235#section-3.1)
    Unauthorized,
    /// [Section 6.5.2 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.2)
    PaymentRequired,
    /// [Section 6.5.3 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.3)
    Forbidden,
    /// [Section 6.5.4 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.4)
    NotFound,
    /// [Section 6.5.5 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.5)
    MethodNotAllowed,
    /// [Section 6.5.6 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.6)
    NotAcceptable,
    /// [Section 3.2 of RFC #7235](https://datatracker.ietf.org/doc/html/rfc7235#section-3.2)
    ProxyAuthenticationRequired,
    /// [Section 6.5.7 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.7)
    RequestTimeout,
    /// [Section 6.5.8 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.8)
    Conflict,
    /// [Section 6.5.9 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.9)
    Gone,
    /// [Section 6.5.10 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.10)
    LengthRequired,
    /// [Section 4.2 of RFC #7232](https://datatracker.ietf.org/doc/html/rfc7232#section-4.2)
    PreconditionFailed,
    /// [Section 6.5.11 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.11)
    PayloadTooLarge,
    /// [Section 6.5.12 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.12)
    UriTooLong,
    /// [Section 6.5.13 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.13)
    UnsupportedMediaType,
    /// [Section 4.4 of RFC #7233](https://datatracker.ietf.org/doc/html/rfc7233#section-4.4)
    RangeNotSatisfiable,
    /// [Section 6.5.14 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.14)
    ExpectationFailed,
    /// [Section 6.5.15 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.5.15)
    UpgradeRequired,
    /// [Section 3 of RFC #6585](https://datatracker.ietf.org/doc/html/rfc6585#section-3)
    PreconditionRequired,
    /// [Section 4 of RFC #6585](https://datatracker.ietf.org/doc/html/rfc6585#section-4)
    TooManyRequests,
    /// [Section 5 of RFC #6585](https://datatracker.ietf.org/doc/html/rfc6585#section-5)
    RequestHeaderFieldsTooLarge,
    //*
    // Server Error (500-599)
    //*
    /// [Section 6.6.1 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.6.1)
    InternalServerError,
    /// [Section 6.6.2 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.6.2)
    NotImplemented,
    /// [Section 6.6.3 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.6.3)
    BadGateway,
    /// [Section 6.6.4 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.6.4)
    ServiceUnavailable,
    /// [Section 6.6.5 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.6.5)
    GatewayTimeout,
    /// [Section 6.6.6 of RFC #7231](https://datatracker.ietf.org/doc/html/rfc7231#section-6.6.6)
    HttpVersionNotSupported,
    /// [Section 6 of RFC #6585](https://datatracker.ietf.org/doc/html/rfc6585#section-6)
    NetworkAuthenticationRequired,
    //*
    // Custom
    //*
    /// If a server responds with a custom code this variant will be used.
    /// The `u16` contained is the status code.
    Custom(u16),
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        self.into()
    }
}

impl From<u16> for HttpStatus {
    fn from(other: u16) -> Self {
        match other {
            100 => Self::Continue,
            101 => Self::SwitchingProtocols,
            200 => Self::Ok,
            201 => Self::Created,
            202 => Self::Accepted,
            203 => Self::NonAuthoritativeInformation,
            204 => Self::NoContent,
            205 => Self::ResetContent,
            206 => Self::PartialContent,
            300 => Self::MultipleChoices,
            301 => Self::MovedPermanently,
            302 => Self::Found,
            303 => Self::SeeOther,
            304 => Self::NotModified,
            305 => Self::UseProxy,
            307 => Self::TemporaryRedirect,
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            406 => Self::NotAcceptable,
            407 => Self::ProxyAuthenticationRequired,
            408 => Self::RequestTimeout,
            409 => Self::Conflict,
            410 => Self::Gone,
            411 => Self::LengthRequired,
            412 => Self::PreconditionFailed,
            413 => Self::PayloadTooLarge,
            414 => Self::UriTooLong,
            415 => Self::UnsupportedMediaType,
            416 => Self::RangeNotSatisfiable,
            417 => Self::ExpectationFailed,
            426 => Self::UpgradeRequired,
            428 => Self::PreconditionRequired,
            429 => Self::TooManyRequests,
            431 => Self::RequestHeaderFieldsTooLarge,
            500 => Self::InternalServerError,
            501 => Self::NotImplemented,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            505 => Self::HttpVersionNotSupported,
            511 => Self::NetworkAuthenticationRequired,
            _ => Self::Custom(other),
        }
    }
}

impl From<HttpStatus> for u16 {
    fn from(other: HttpStatus) -> Self {
        match other {
            HttpStatus::Continue => 100,
            HttpStatus::SwitchingProtocols => 101,
            HttpStatus::Ok => 200,
            HttpStatus::Created => 201,
            HttpStatus::Accepted => 202,
            HttpStatus::NonAuthoritativeInformation => 203,
            HttpStatus::NoContent => 204,
            HttpStatus::ResetContent => 205,
            HttpStatus::PartialContent => 206,
            HttpStatus::MultipleChoices => 300,
            HttpStatus::MovedPermanently => 301,
            HttpStatus::Found => 302,
            HttpStatus::SeeOther => 303,
            HttpStatus::NotModified => 304,
            HttpStatus::UseProxy => 305,
            HttpStatus::TemporaryRedirect => 307,
            HttpStatus::BadRequest => 400,
            HttpStatus::Unauthorized => 401,
            HttpStatus::PaymentRequired => 402,
            HttpStatus::Forbidden => 403,
            HttpStatus::NotFound => 404,
            HttpStatus::MethodNotAllowed => 405,
            HttpStatus::NotAcceptable => 406,
            HttpStatus::ProxyAuthenticationRequired => 407,
            HttpStatus::RequestTimeout => 408,
            HttpStatus::Conflict => 409,
            HttpStatus::Gone => 410,
            HttpStatus::LengthRequired => 411,
            HttpStatus::PreconditionFailed => 412,
            HttpStatus::PayloadTooLarge => 413,
            HttpStatus::UriTooLong => 414,
            HttpStatus::UnsupportedMediaType => 415,
            HttpStatus::RangeNotSatisfiable => 416,
            HttpStatus::ExpectationFailed => 417,
            HttpStatus::UpgradeRequired => 426,
            HttpStatus::PreconditionRequired => 428,
            HttpStatus::TooManyRequests => 429,
            HttpStatus::RequestHeaderFieldsTooLarge => 431,
            HttpStatus::InternalServerError => 500,
            HttpStatus::NotImplemented => 501,
            HttpStatus::BadGateway => 502,
            HttpStatus::ServiceUnavailable => 503,
            HttpStatus::GatewayTimeout => 504,
            HttpStatus::HttpVersionNotSupported => 505,
            HttpStatus::NetworkAuthenticationRequired => 511,
            HttpStatus::Custom(code) => code,
        }
    }
}

impl Display for HttpStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            HttpStatus::Continue => "100: Continue",
            HttpStatus::SwitchingProtocols => "101: Switching Protocols",
            HttpStatus::Ok => "200: OK",
            HttpStatus::Created => "201: Created",
            HttpStatus::Accepted => "202: Accepted",
            HttpStatus::NonAuthoritativeInformation => "203: Non-Authoritative Information",
            HttpStatus::NoContent => "204: No Content",
            HttpStatus::ResetContent => "205: Reset Content",
            HttpStatus::PartialContent => "206: Partial Content",
            HttpStatus::MultipleChoices => "300: Multiple Choices",
            HttpStatus::MovedPermanently => "301: Moved Permanently",
            HttpStatus::Found => "302: Found",
            HttpStatus::SeeOther => "303: See Other",
            HttpStatus::NotModified => "304: Not Modified",
            HttpStatus::UseProxy => "305: Use Proxy",
            HttpStatus::TemporaryRedirect => "307: Temporary Redirect",
            HttpStatus::BadRequest => "400: Bad Request",
            HttpStatus::Unauthorized => "401: Unauthorized",
            HttpStatus::PaymentRequired => "402: Payment Required",
            HttpStatus::Forbidden => "403: Forbidden",
            HttpStatus::NotFound => "404: Not Found",
            HttpStatus::MethodNotAllowed => "405: Method Not Allowed",
            HttpStatus::NotAcceptable => "406: Not Acceptable",
            HttpStatus::ProxyAuthenticationRequired => "407: Proxy Authentication Required",
            HttpStatus::RequestTimeout => "408: Request Timeout",
            HttpStatus::Conflict => "409: Conflict",
            HttpStatus::Gone => "410: Gone",
            HttpStatus::LengthRequired => "411: Length Required",
            HttpStatus::PreconditionFailed => "412: Precondition Failed",
            HttpStatus::PayloadTooLarge => "413: Payload Too Large",
            HttpStatus::UriTooLong => "414: URI Too Long",
            HttpStatus::UnsupportedMediaType => "415: Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "416: Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "417: Expectation Failed",
            HttpStatus::UpgradeRequired => "426: Upgrade Required",
            HttpStatus::PreconditionRequired => "428: Precondition Required",
            HttpStatus::TooManyRequests => "429: Too Many Requests",
            HttpStatus::RequestHeaderFieldsTooLarge => "431: Request Header Fields Too Large",
            HttpStatus::InternalServerError => "500: Internal Server Error",
            HttpStatus::NotImplemented => "501: Not Implemented",
            HttpStatus::BadGateway => "502: Bad Gateway",
            HttpStatus::ServiceUnavailable => "503: Service Unavailable",
            HttpStatus::GatewayTimeout => "504: Gateway Timeout",
            HttpStatus::HttpVersionNotSupported => "505: HTTP Version Not Supported",
            HttpStatus::NetworkAuthenticationRequired => "511: Network Authentication Required",
            HttpStatus::Custom(code) => &format!("{}: Custom Response", code),
        })
    }
}
