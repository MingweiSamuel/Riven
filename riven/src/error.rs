use std::fmt;

use reqwest::{Response, StatusCode};

/// Result containing RiotApiError on failure.
pub type Result<T> = std::result::Result<T, RiotApiError>;

/// An error that occurred while processing a Riot API request.
#[derive(Debug)]
pub struct RiotApiError {
    reqwest_errors: Vec<reqwest::Error>,
    serde_error: Option<serde_json::Error>,
    retries: u8,
    response: Option<Response>,
    status_code: Option<StatusCode>,
}
impl RiotApiError {
    pub(crate) fn new(
        reqwest_errors: Vec<reqwest::Error>,
        serde_error: Option<serde_json::Error>,
        retries: u8,
        response: Option<Response>,
        status_code: Option<StatusCode>,
    ) -> Self {
        Self {
            reqwest_errors,
            serde_error,
            retries,
            response,
            status_code,
        }
    }

    /// Returns the final `reqwest::Error`, for the final failed request, or panics if this was a deserialization error.
    #[deprecated = "Use `reqwest_errors()` instead."]
    pub fn source_reqwest_error(&self) -> &reqwest::Error {
        &self.reqwest_errors.last().unwrap()
    }

    /// Returns all `reqwest::Error`s across all retries, in the chronological order they occurred.
    ///
    /// May be empty if there was a deserialization error.
    pub fn reqwest_errors(&self) -> &[reqwest::Error] {
        &self.reqwest_errors
    }

    /// Returns the final deserialization error if any occured.
    pub fn serde_error(&self) -> Option<&serde_json::Error> {
        self.serde_error.as_ref()
    }

    /// The number of retires attempted. Zero means exactly one request, zero retries.
    pub fn retries(&self) -> u8 {
        self.retries
    }

    /// The failed response.
    /// `Some(&reqwest::Response)` if the request was sent and failed.
    /// `None` if the request was not sent, OR if parsing the response JSON failed.
    pub fn response(&self) -> Option<&Response> {
        self.response.as_ref()
    }

    /// The failed response.
    /// `Some(reqwest::Response)` if the request was sent and failed.
    /// `None` if the request was not sent, OR if parsing the response JSON failed.
    pub fn take_response(&mut self) -> Option<Response> {
        self.response.take()
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
impl std::error::Error for RiotApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.reqwest_errors().last().map(|e| e as _)
    }
}
