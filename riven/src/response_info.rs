use reqwest::Response;

#[cfg(not(feature = "eserde"))]
use serde::de::Deserialize;
#[cfg(feature = "eserde")]
use eserde::{EDeserialize as Deserialize, json as serde_json};

use crate::{Result, RiotApiError};

/// A "raw" unparsed successful response from the Riot API, for internal or advanced use cases.
#[non_exhaustive]
pub struct ResponseInfo {
    /// The reqwest response.
    pub response: Response,
    /// The number of retries used, zero for first-try success.
    pub retries: u8,
    /// If the response has an HTTP status code indicating a `None` response (i.e. 204, 404).
    pub status_none: bool,
    /// Any reqwest errors that occurred. A response may still be successful even if this is non-empty due to retrying.
    pub reqwest_errors: Vec<reqwest::Error>,
}

impl ResponseInfo {
    /// Helper to deserialize the JSON-encoded value from a `ResponseInfo`, properly handling errors.
    pub(crate) async fn json<T: for<'de> Deserialize<'de>>(self) -> Result<T> {
        let Self {
            response,
            retries,
            status_none: _,
            mut reqwest_errors,
        } = self;
        let status = response.status();
        let bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(err) => {
                reqwest_errors.push(err);
                return Err(RiotApiError::new(
                    reqwest_errors,
                    None,
                    retries,
                    None, // `.bytes()` consumes the response.
                    Some(status),
                ));
            }
        };
        let value = match serde_json::from_slice::<T>(&bytes) {
            Ok(value) => value,
            Err(serde_err) => {
                return Err(RiotApiError::new(
                    reqwest_errors,
                    Some(serde_err),
                    retries,
                    None, // `.bytes()` consumes the response.
                    Some(status),
                ));
            }
        };
        Ok(value)
    }
}
