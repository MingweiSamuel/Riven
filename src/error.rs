use std::error::Error as StdError;
use std::fmt;

use crate::reqwest::*;

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
    response: Option<Response>,
    status_code: Option<StatusCode>,
}
impl RiotApiError {
    pub(crate) fn new(reqwest_error: Error, retries: u8, response: Option<Response>, status_code: Option<StatusCode>) -> Self {
        Self {
            reqwest_error: reqwest_error,
            retries: retries,
            response: response,
            status_code: status_code,
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
    /// The failed response.
    /// `Some(reqwest::Response)` if the request was sent and failed.
    /// `None` if the request was not sent, OR if parsing the response JSON failed.
    pub fn response(&self) -> Option<&Response> {
        self.response.as_ref()
    }
    /// The failed response's HTTP status code.
    /// `Some(reqwest::StatusCode)` if the request was sent and failed, OR if parsing the response JSON failed.
    /// `None` if the request was not sent.
    pub fn status_code(&self) -> Option<StatusCode> {
        self.status_code
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
