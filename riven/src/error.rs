use std::fmt;

use reqwest::{Response, StatusCode};

/// Result containing RiotApiError on failure.
pub type Result<T> = std::result::Result<T, RiotApiError>;

/// An error that occurred while processing a Riot API request.
#[derive(Debug)]
pub struct RiotApiError {
    reqwest_errors: Vec<reqwest::Error>,
    de_error: Option<crate::de::Error>,
    retries: u8,
    response: Option<Response>,
    status_code: Option<StatusCode>,
}
impl RiotApiError {
    pub(crate) fn new(
        reqwest_errors: Vec<reqwest::Error>,
        de_error: Option<crate::de::Error>,
        retries: u8,
        response: Option<Response>,
        status_code: Option<StatusCode>,
    ) -> Self {
        Self {
            reqwest_errors,
            de_error,
            retries,
            response,
            status_code,
        }
    }

    /// Returns the final `reqwest::Error`, for the final failed request, or panics if this was a deserialization error.
    #[deprecated = "Use `reqwest_errors()` or `de_error()` instead."]
    pub fn source_reqwest_error(&self) -> &reqwest::Error {
        self.reqwest_errors.last().unwrap()
    }

    /// Returns all `reqwest::Error`s across all retries, in the chronological order they occurred.
    ///
    /// May be empty if there was a deserialization error.
    pub fn reqwest_errors(&self) -> &[reqwest::Error] {
        &self.reqwest_errors
    }

    /// Returns the final deserialization error if any occured.
    pub fn de_error(&self) -> Option<&crate::de::Error> {
        self.de_error.as_ref()
    }

    /// The number of retires attempted. Zero means exactly one request, zero retries.
    pub fn retries(&self) -> u8 {
        self.retries
    }

    /// The failed, unparsed response.
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
        writeln!(
            f,
            "Riot API request failed after {} retries with status code {:?}{}:",
            self.retries(),
            self.status_code(),
            if self.response().is_some() {
                " (response not parsed)"
            } else {
                ""
            },
        )?;
        for (i, reqwest_error) in self.reqwest_errors().iter().enumerate() {
            writeln!(f, "- Reqwest error {}: {}", i + 1, reqwest_error)?;
        }
        if let Some(serde_error) = self.de_error() {
            writeln!(f, "- Deserialization error: {}", serde_error)?;
        }
        Ok(())
    }
}
impl std::error::Error for RiotApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.reqwest_errors()
            .last()
            .map(|e| e as _)
            .or_else(|| self.de_error().map(|e| e as _))
    }
}

/// Error returned by `try_` methods. Either not enough capacity or the [`RiotApiError`] that
/// occurred while processing the request.
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs, clippy::large_enum_variant)]
pub enum TryRequestError {
    #[error("Not enough capacity available to process the request immediately (based on the capacity overhead requested)")]
    NotEnoughCapacity,
    #[error(transparent)]
    RiotApiError(#[from] RiotApiError),
}

/// Result containing either NotEnoughReserve or RiotApiError on failure.
pub type TryRequestResult<T> = std::result::Result<T, TryRequestError>;
