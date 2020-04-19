use std::error::Error as StdError;
use std::fmt;

/// Re-exported `reqwest` types.
pub mod reqwest {
    pub use reqwest::{
        Error, Response, StatusCode, Url, header::HeaderMap
    };
}
use ::reqwest::*;

/// Result containing RiotApiError on failure.
pub type Result<T> = std::result::Result<T, RiotApiError>;

/// An error that occurred while processing a Riot API request.
///
/// Although Riven may make multiple requests due to retries, this will always
/// contain exactly one reqwest::Error for the final request which failed.
#[derive(Debug)]
pub struct RiotApiError {
    reqwest_error: Error,
    retries: u8,
    headers: Option<header::HeaderMap>,
    status_code: Option<StatusCode>,
    response_body: Option<String>,
}
impl RiotApiError {
    pub(crate) fn new(
        reqwest_error: Error,
        retries: u8,
        headers: Option<header::HeaderMap>,
        status_code: Option<StatusCode>,
        response_body: Option<String>
    ) -> Self {
        Self {
            reqwest_error: reqwest_error,
            retries: retries,
            headers: headers,
            status_code: status_code,
            response_body: response_body,
        }
    }
    /// The reqwest::Error for the final failed request.
    pub fn source_reqwest_error(&self) -> &Error {
        &self.reqwest_error
    }
    /// The number of retires attempted. Zero means exactly one request, zero retries.
    pub fn retries(&self) -> u8 {
        self.retries
    }
    /// The failed response's HTTP status code.
    /// `Some(reqwest::StatusCode)` if the request was sent and failed, OR if parsing the response JSON failed.
    /// `None` if the request was not sent.
    pub fn status_code(&self) -> Option<StatusCode> {
        self.status_code
    }
    /// The failed response's headers.
    /// `Some(reqwest::header::HeaderMap)` if the request was sent and failed.
    /// `None` if the request was not sent.
    pub fn headers(&self) -> Option<header::HeaderMap> {
        self.headers.clone()
    }
    /// The failed response's body (as a copy).
    /// `Some(String)` if the request was sent and failed.
    /// `None` if the request was not sent.
    pub fn response_body(&self) -> Option<String> {
        self.response_body.clone()
    }
}
impl fmt::Display for RiotApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl StdError for RiotApiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.reqwest_error)
    }
}
