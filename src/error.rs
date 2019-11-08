use std::error::Error as StdError;
use std::fmt;

/// Re-exported `reqwest` types.
pub mod reqwest {
    pub use reqwest::Error;
    pub use reqwest::Response;
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
    response: Option<Response>,
}
impl RiotApiError {
    pub fn new(reqwest_error: Error, retries: u8, response: Option<Response>) -> Self {
        Self {
            reqwest_error: reqwest_error,
            retries: retries,
            response: response,
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
    /// The failed response, if the request was sent and failed.
    /// Will be `None` if JSON parsing failed.
    pub fn response<'a>(&self) -> Option<&Response> {
        self.response.as_ref()
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
